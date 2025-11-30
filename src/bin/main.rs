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
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::master::Spi;
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::main;

use zssh::AuthMethod;

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


esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

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

    file_manager::list_files_in_folder(sd);



    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}