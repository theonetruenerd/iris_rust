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

use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::main;
use embedded_sdmmc::{TimeSource, Timestamp};
use esp_hal::uart::{Uart, Config as UartConfig};
use esp_println::println;
use iris::apps::file_manager;
use iris::apps::gps;
use iris::apps::power::get_battery_percentage;
use core::panic::PanicInfo;
use iris::apps::display::{display_background};
use iris::apps::file_manager::get_bmp;
use iris::apps::usb;
use iris::apps::ssh;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic occurred: {}", info);
    loop {}
}

// Creates the dummy timesource for the SD card
#[derive(Default)]
pub struct DummyTimesource();

// Returns a dummy timestamp
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

// ESP-IDF app descriptor - required for bootloader
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // Initialize logger
    esp_println::logger::init_logger_from_env();

    // Setup ESP32 peripherals
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Initialize backlight
    Output::new(peripherals.GPIO38, Level::High, OutputConfig::default());

    // Create buffer for reading GPS data
    let mut buffer = [0u8; 128];

    // Create bmp buffer for loading images
    let mut bmp_buffer = [0u8; 102400];

    // Initialize SD card
    let sd = file_manager::sd_card_init(
        peripherals.SPI3,
        peripherals.GPIO40,
        peripherals.GPIO14,
        peripherals.GPIO39,
        peripherals.GPIO12,
    );

    // Get iris background image from SD card
    let bmp = get_bmp(sd, "iris.bmp", &mut bmp_buffer );

    // Display background image
    display_background(
        peripherals.GPIO37,
        peripherals.GPIO34,
        peripherals.GPIO33,
        peripherals.SPI2,
        peripherals.GPIO36,
        peripherals.GPIO35,
        bmp,
    );

    // Initialize UART
    let mut uart = Uart::new(
        peripherals.UART0,
        UartConfig::default()
            .with_baudrate(115200),
        )
        .unwrap()
        .with_rx(peripherals.GPIO1)
        .with_tx(peripherals.GPIO2);

    // file_manager::list_files_in_folder(sd);

    // Create NMEA buffer for GPS data reading
    let mut nmea_buffer = gps::NmeaBuffer::new();

    // Write string to USB connected device
    usb::write_str(peripherals.USB_DEVICE, "Hello from Iris!\r\n");

    // Setup SSH server authentication
    ssh::setup_auth();

    // Prints current battery percentage to USB connected terminal
    println!("Battery percentage: {}%", get_battery_percentage(peripherals.ADC1, peripherals.GPIO10));

    // Creates main loop
    loop {
        // Reads data from UART and parses NMEA sentences
        match uart.read(&mut buffer) {
            Ok(bytes_read) => {
                nmea_buffer.add_data(&buffer[..bytes_read]);

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