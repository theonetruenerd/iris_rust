#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]


// SPM1423 (microphone)
// DAT: GPIO46 (data)
// CLK: GPIO43 (clock)
// VCC: 3.3V (power)
// GND: GND (ground)

// microSD Socket
// CS: GPIO12 (chip select)
// MOSI: GPIO14 (master output slave input)
// CLK: GPIO40 (clock)
// MISO: GPIO39 (master input slave output)

// ST7789V2 (screen)
// DISP_BL: GPIO38  (backlight)
// RST: GPIO33  (reset)
// RS: GPIO34 (register select)
// DAT: GPIO35 (data)
// SCK: GPIO36 (serial clock)
// CS: GPIO37 (chip select)

// RGB LED
// VDD: GPIO38

// Battery Detect ADC
// ADC: GPIO10 (analog digital converter)

// 74HC138 (Keyboard)
// Y7-Y0: GPIO7-GPIO3, GPIO15, GPIO13 (output lines)
// A2, A1, A0: GPIO11, GPIO9, GPIO8 (address inputs)

// NS4168 (speaker)
// BCLK: GPIO41 (bit clock)
// SDATA: GPIO42  (serial data)
// LRCLK: GPIO43 (left-right clock)

// IR
// TX: GPIO44 (transmit)

// Grove
// Black: GND
// Red: 5V
// Yellow: GPIO02
// White: GPIO01

use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig};
use esp_hal::spi::master::Spi;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::main;
use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb565,
    prelude::*
};
use embedded_sdmmc::{TimeSource, Timestamp};
use esp_hal::spi::master::Config as SpiConfig;
use esp_hal::spi::Mode as SpiMode;
use mipidsi::interface::SpiInterface;
use mipidsi::options::{ColorInversion, Orientation, Rotation};
use mipidsi::{models::ST7789, Builder};
use tinybmp::Bmp;
use esp_hal::analog::adc::{AdcConfig, Adc, Attenuation};
use esp_hal::uart::{Uart, Config as UartConfig};
use esp_println::println;
use iris::apps::file_manager;

// Consts
const DISPLAY_WIDTH: i32 = 320;
const DISPLAY_HEIGHT: i32 = 240;


#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[derive(Default)]
pub struct DummyTimesource();

impl TimeSource for DummyTimesource {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

pub struct NmeaBuffer {
    buffer: [u8; 512],
    write_pos: usize,
    read_pos: usize,
}

impl NmeaBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [0u8; 512],
            write_pos: 0,
            read_pos: 0,
        }
    }

    /// Add new data to the buffer
    pub fn add_data(&mut self, data: &[u8]) {
        for &byte in data {
            self.buffer[self.write_pos] = byte;
            self.write_pos = (self.write_pos + 1) % self.buffer.len();

            // Prevent overflow - this is a safety measure
            if self.write_pos == self.read_pos {
                self.read_pos = (self.read_pos + 1) % self.buffer.len();
            }
        }
    }

    /// Try to extract a complete NMEA sentence (ending with \r\n)
    pub fn get_sentence(&mut self) -> Option<NmeaSentence> {
        let mut sentence_len = 0;
        let mut pos = self.read_pos;

        // Search for \r\n
        while pos != self.write_pos {
            if sentence_len > 0 && self.buffer[pos] == b'\n' &&
                self.buffer[(pos + self.buffer.len() - 1) % self.buffer.len()] == b'\r' {
                // Found end of sentence
                sentence_len += 1;
                break;
            }
            pos = (pos + 1) % self.buffer.len();
            sentence_len += 1;
        }

        // If we found a complete sentence
        if pos != self.write_pos && self.buffer[pos] == b'\n' {
            let mut sentence_data = [0u8; 128];
            let mut idx = 0;
            let mut temp_pos = self.read_pos;

            while temp_pos != pos {
                sentence_data[idx] = self.buffer[temp_pos];
                idx += 1;
                temp_pos = (temp_pos + 1) % self.buffer.len();
            }

            // Skip the \r\n
            self.read_pos = (pos + 1) % self.buffer.len();

            return Some(NmeaSentence {
                data: sentence_data,
                length: idx,
            });
        }

        None
    }
}

pub struct NmeaSentence {
    pub data: [u8; 128],
    pub length: usize,
}

impl NmeaSentence {
    pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.data[..self.length.saturating_sub(2)]) // Remove \r\n
    }
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut adc1_config = AdcConfig::new();
    let mut adc = Adc::new(peripherals.ADC1, AdcConfig::default());

    let mut battery_pin = adc1_config.enable_pin(peripherals.GPIO10, Attenuation::_11dB);
    

    let spi = Spi::new(
        peripherals.SPI2,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(10))
            .with_mode(SpiMode::_0),
        )
        .unwrap()
        .with_sck(peripherals.GPIO36)
        .with_mosi(peripherals.GPIO35);

    let cs = Output::new(peripherals.GPIO37, Level::High, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO34, Level::Low, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());

    let mut delay = Delay::new();
    let bl = Output::new(peripherals.GPIO38, Level::High, OutputConfig::default());

    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let mut buffer = [0u8; 512];

    let di = SpiInterface::new(spi_device, dc, &mut buffer);

    let image_w = 240;
    let image_h = 135;

    let x_position = (DISPLAY_WIDTH - image_w) / 2;
    let y_position = (DISPLAY_HEIGHT - image_h) / 2;

    let mut display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .invert_colors(ColorInversion::Inverted)
        .orientation(Orientation::new().rotate(Rotation::Deg90))
        .init(&mut delay)
        .unwrap();

    display.clear(Rgb565::BLACK).unwrap();

    let bmp_data = include_bytes!("../../assets/images/iris_background.bmp");
    let bmp = Bmp::<Rgb565>::from_slice(bmp_data).unwrap();

    Image::new(&bmp, Point::new(x_position,y_position)).draw(&mut display).unwrap();

    let sd = file_manager::sd_card_init(
        peripherals.SPI3,
        peripherals.GPIO40,
        peripherals.GPIO14,
        peripherals.GPIO39,
        peripherals.GPIO12,
    );

    let mut uart = Uart::new(
        peripherals.UART0,
        UartConfig::default()
            .with_baudrate(115200),
        )
        .unwrap()
        .with_rx(peripherals.GPIO1)
        .with_tx(peripherals.GPIO2);

    file_manager::list_files_in_folder(sd);

    let mut nmea_buffer = NmeaBuffer::new();
    let mut buffer = [0u8; 128];

    loop {
        let delay_start = Instant::now();
        let battery_raw: u16 = nb::block!(adc.read_oneshot(&mut battery_pin)).unwrap();
        let battery_voltage = (battery_raw as f32 * 3.3) / 4095.0;
        let battery_percentage = ((battery_voltage - 2.5) / (4.2 - 2.5) * 100.0).max(0.0).min(100.0);
        println!("Battery: {:.2}%", battery_percentage);
        match uart.read(&mut buffer) {
            Ok(bytes_read) => {
                nmea_buffer.add_data(&buffer[..bytes_read]);

                // Try to extract complete sentences
                while let Some(sentence) = nmea_buffer.get_sentence() {
                    if let Ok(sentence_str) = sentence.as_str() {
                        println!("Complete NMEA: {}", sentence_str);
                        // Parse the sentence here
                    }
                }
            }
            Err(e) => println!("Error reading UART: {:?}", e),
        }
    }
}