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
use embedded_sdmmc::{Mode, SdCard, TimeSource, Timestamp, VolumeIdx, VolumeManager};
use esp_hal::clock::CpuClock;
use esp_hal::{main, spi};
use esp_hal::delay::Delay;
use esp_hal::time::{Duration, Instant, Rate};
use esp_println::println;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::spi::master::Spi;

use esp_hal::spi::master::Config as SpiConfig;
use esp_hal::spi::Mode as SpiMode;
use mipidsi::interface::SpiInterface;
use mipidsi::{Builder, models::ST7789};
use embedded_graphics::{
    prelude::*,
    pixelcolor::Rgb565,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    text::Text,
    primitives::{Circle, PrimitiveStyle, Primitive},
};
use mipidsi::options::{ColorInversion, Orientation, Rotation};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
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

    let mut display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .invert_colors(ColorInversion::Inverted)
        .orientation(Orientation::new().rotate(Rotation::Deg90))
        .init(&mut delay)
        .unwrap();

    display.clear(Rgb565::WHITE).unwrap();

    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::BLACK);

    Text::new("Hello World!", Point::new(10, 50), text_style)
        .draw(&mut display)
        .unwrap();

    let circle_style = PrimitiveStyle::with_stroke(Rgb565::RED, 5);

    Circle::new(Point::new(50, 80), 60)
        .into_styled(circle_style)
        .draw(&mut display)
        .unwrap();

    loop {
        println!("Hello World!");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
