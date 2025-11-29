#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_println::println;
use esp_hal::gpio::{Level, Output, Input};

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    // generator version: 1.0.1
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut bl = Output::new(peripherals.GPIO38, Level::Low, Default::default());

    // SPM1423: ESP32-S3FN8
    // DAT: GPIO46
    // CLK: GPIO43
    // VCC: 3.3V
    // GND: GND
    // microSD Socket: ESP32-S3FN8
    // CS: GPIO12
    // MOSI: GPIO14
    // CLK: GPIO40
    // MISO: GPIO39
    // ST7789V2: ESP32-S3FN8
    // RGB LED: ESP32-S3FN8
    // DISP_BL: GPIO38
    // VDD: GPIO38
    // RST: GPIO33
    // RS: GPIO34
    // DAT: GPIO35
    // SCK: GPIO36
    // CS: GPIO37
    // Battery Detect ADC: ESP32-S3FN8
    // 74HC138: ESP32-S3FN8
    // ADC: GPIO10
    // Y7-Y0: GPIO7-GPIO3, GPIO15, GPIO13
    // A2, A1, A0: GPIO11, GPIO9, GPIO8
    // NS4168 Speaker: ESP32-S3FN8
    // IR: ESP32-S3FN8
    // BCLK: GPIO41
    // SDATA: GPIO42
    // LRCLK: GPIO43
    // TX: GPIO44
    // Grove Black: GND
    // Grove Red: 5V
    // Grove Yellow: GPIO02
    // Grove White: GPIO01

    loop {
        println!("{:?}", bl.output_level());
        // bl.toggle();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples/src/bin
}
