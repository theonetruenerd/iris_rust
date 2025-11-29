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

    // let grove = peripherals.GPIO1;
    // let a0 = peripherals.GPIO8;
    // let a1 = peripherals.GPIO9;
    // let vbat = peripherals.GPIO10;
    // let a2 = peripherals.GPIO11;
    // let d_minus = peripherals.GPIO19;
    // let d_plus = peripherals.GPIO20;
    // let rst = peripherals.GPIO33;
    // let rs = peripherals.GPIO34;
    // let data = peripherals.GPIO35;
    // let sck = peripherals.GPIO36;
    // let cs = peripherals.GPIO37;
    // let bl = peripherals.GPIO38;
    // let bk = peripherals.GPIO41;
    // let dat = peripherals.GPIO42;
    // let lr = peripherals.GPIO43;

    loop {
        println!("{:?}", bl.output_level());
        // bl.toggle();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0/examples/src/bin
}
