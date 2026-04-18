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
use esp_hal::time::Rate;
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
use esp_hal::uart::{Uart, Config as UartConfig};
use esp_println::println;
use iris::apps::file_manager;
use iris::apps::gps;
use iris::drivers::power::get_battery_percentage;
use core::panic::PanicInfo;
use iris::drivers::display::{display_app_icon, draw_menu};
use iris::drivers::keyboard::Keyboard;
use esp_hal::gpio::{Input, InputConfig, Pull};
use iris::drivers::usb;
use iris::apps::ssh;
use iris::apps::scanner;
use esp_hal::i2c::master::{Config as I2cConfig, I2c};

// Consts
const DISPLAY_WIDTH: i32 = 320;
const DISPLAY_HEIGHT: i32 = 240;


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Panic occurred: {}", info);
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
    let mut backlight = Output::new(peripherals.GPIO38, Level::High, OutputConfig::default());

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

    // let bmp_data = include_bytes!("../../assets/images/iris_background.bmp");
    // let bmp = Bmp::<Rgb565>::from_slice(bmp_data).unwrap();
    //
    // Image::new(&bmp, Point::new(x_position,y_position)).draw(&mut display).unwrap();

    let sd = file_manager::sd_card_init(
        peripherals.SPI3,
        peripherals.GPIO40,
        peripherals.GPIO14,
        peripherals.GPIO39,
        peripherals.GPIO12,
    );

    // let mut uart = Uart::new(
    //     peripherals.UART0,
    //     UartConfig::default()
    //         .with_baudrate(115200),
    //     )
    //     .unwrap()
    //     .with_rx(peripherals.GPIO1)
    //     .with_tx(peripherals.GPIO2);

    file_manager::list_files_in_folder(sd);

    let mut nmea_buffer = gps::NmeaBuffer::new();
    let mut buffer = [0u8; 128];

    usb::write_str(peripherals.USB_DEVICE, "Hello from Iris!\r\n");

    ssh::setup_auth();

    println!("Battery percentage: {}%", get_battery_percentage(peripherals.ADC1, peripherals.GPIO10));
    let menu_items = ["GPS", "File Manager", "SSH Auth", "Scanner", "Power"];
    let mut selected_idx = 0;

    let keyboard_a0 = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let keyboard_a1 = Output::new(peripherals.GPIO9, Level::Low, OutputConfig::default());
    let keyboard_a2 = Output::new(peripherals.GPIO11, Level::Low, OutputConfig::default());

    let keyboard_rows = [
        Input::new(peripherals.GPIO13, InputConfig::default().with_pull(Pull::Up)),
        Input::new(peripherals.GPIO15, InputConfig::default().with_pull(Pull::Up)),
        Input::new(peripherals.GPIO3, InputConfig::default().with_pull(Pull::Up)),
        Input::new(peripherals.GPIO4, InputConfig::default().with_pull(Pull::Up)),
        Input::new(peripherals.GPIO5, InputConfig::default().with_pull(Pull::Up)),
        Input::new(peripherals.GPIO6, InputConfig::default().with_pull(Pull::Up)),
        Input::new(peripherals.GPIO7, InputConfig::default().with_pull(Pull::Up)),
    ];

    let mut keyboard = Keyboard::new(keyboard_a0, keyboard_a1, keyboard_a2, keyboard_rows);

    let mut i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(100)),
    )
    .unwrap()
    .with_sda(peripherals.GPIO1)
    .with_scl(peripherals.GPIO2);

    loop {
        draw_menu(&mut display, &menu_items, selected_idx);

        if let Some((col, row)) = keyboard.scan() {
            println!("Key pressed: col {}, row {}", col, row);
            match (col, row) {
                (0, 0) => {
                    selected_idx = (selected_idx + 1) % menu_items.len();
                    delay.delay_millis(200);
                }
                (0, 1) => {
                    match menu_items[selected_idx] {
                        "Scanner" => {
                            println!("Starting I2C Scanner...");
                            scanner::scan_i2c(&mut i2c);
                        }
                        "SSH Auth" => {
                            println!("Starting SSH Terminal...");
                            let mut terminal = ssh::Terminal::new();
                            terminal.write_str("\x1b[32mIris SSH Terminal\x1b[0m\n");
                            terminal.write_str("Connecting...\n");
                            
                            // Mock terminal interaction for now
                            terminal.render(&mut display);
                            
                            loop {
                                if let Some((c, r)) = keyboard.scan() {
                                    if c == 0 && r == 2 { // Assume escape or something to exit
                                        break;
                                    }
                                    // Map keyboard to characters and write to terminal
                                    // For now just show we can write
                                    terminal.write_char('.');
                                    terminal.render(&mut display);
                                    delay.delay_millis(150);
                                }
                                delay.delay_millis(50);
                            }
                        }
                        _ => {
                            println!("Selected: {}", menu_items[selected_idx]);
                        }
                    }
                    delay.delay_millis(200);
                }
                _ => {}
            }
        }

        delay.delay_millis(100);
    }
}