#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_sdmmc::{Mode, SdCard, TimeSource, Timestamp, VolumeIdx, VolumeManager};
use esp_hal::clock::CpuClock;
use esp_hal::{main, spi};
use esp_hal::delay::Delay;
use esp_hal::time::{Duration, Instant, Rate};
use esp_println::println;
use esp_hal::gpio::{Level, Output};
use esp_hal::spi::master::Spi;
use esp_hal::spi::master::Config as SpiConfig;
use esp_hal::spi::Mode as SpiMode;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

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

    let cs = Output::new(peripherals.GPIO12, Level::High, Default::default());
    let sck = peripherals.GPIO40;
    let mosi = peripherals.GPIO14;
    let miso = peripherals.GPIO39;

    let spi_bus = Spi::new(
        peripherals.SPI2,
        spi::master::Config::default()
            .with_frequency(Rate::from_khz(400))
            .with_mode(spi::Mode::_0),
        )
        .unwrap()
        .with_sck(sck)
        .with_mosi(mosi)
        .with_miso(miso);

    let spi_dev = ExclusiveDevice::new(spi_bus, cs, Delay::new()).unwrap();
    let sdcard = SdCard::new(spi_dev, Delay::new());

    println!("Init SD card controller and retrieve card info...");
    let sd_size = sdcard.num_bytes().unwrap();
    println!("SD card size: {} bytes", sd_size);

    let volume_mgr = VolumeManager::new(sdcard, DummyTimesource());

    let volume0 = volume_mgr.open_volume(VolumeIdx(0)).unwrap();

    let root_dir = volume0.open_root_dir().unwrap();

    let mut my_file = root_dir.open_file_in_dir(
        "Test.txt",
        Mode::ReadWriteCreateOrTruncate
        )
        .unwrap();

    let line = "Hello World!";
    if let Ok(()) = my_file.write(line.as_bytes()) {
        my_file.flush().unwrap();
        println!("Wrote: {}", line);
    }

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
        // println!("{:?}", bl.output_level());
        // bl.toggle();
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

}
