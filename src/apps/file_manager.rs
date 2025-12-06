use embedded_graphics::pixelcolor::Rgb565;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use embedded_sdmmc::{Mode, SdCard, TimeSource, VolumeIdx, VolumeManager};
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::peripherals::{GPIO12, GPIO14, GPIO39, GPIO40, SPI3};
use esp_hal::spi::master::Config as SpiConfig;
use esp_hal::spi::master::Spi;
use esp_hal::spi::Mode as SpiMode;
use esp_hal::time::Rate;
use esp_hal::Blocking;
use esp_println::println;
use tinybmp::Bmp;

#[derive(Default)]
pub struct DummyTimesource();

impl TimeSource for DummyTimesource {
    fn get_timestamp(&self) -> embedded_sdmmc::Timestamp {
        embedded_sdmmc::Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

pub type SdCardType<'a> = SdCard<ExclusiveDevice<Spi<'a, Blocking>, Output<'a>, NoDelay>, Delay>;


pub fn sd_card_init<'a> (
    spi: SPI3<'a>,
    sck: GPIO40<'a>,
    mosi: GPIO14<'a>,
    miso: GPIO39<'a>,
    cs: GPIO12<'a>,
) -> SdCardType<'a> {

    let spi_sd = Spi::new(
        spi,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(10))
            .with_mode(SpiMode::_0),
    )
        .unwrap()
        .with_sck(sck)
        .with_mosi(mosi)
        .with_miso(miso);

    let sd_cs = Output::new(cs, Level::High, OutputConfig::default());
    let sd_spi_device = ExclusiveDevice::new_no_delay(spi_sd, sd_cs).unwrap();
    let sdcard = SdCard::new(sd_spi_device, Delay::new());

    sdcard
}

pub fn sd_card_bytes(
    sdcard: &mut SdCardType
) -> u64 {
    println!("Initializing SD card...");
    let sd_size = sdcard.num_bytes().unwrap();
    println!("SD card size: {} bytes", sd_size);

    sd_size
}

pub fn list_files_in_folder(
    sdcard: SdCardType
) {
    let volume_mgr = VolumeManager::new(sdcard, DummyTimesource::default());
    let volume0 = volume_mgr.open_volume(VolumeIdx(0)).unwrap();
    
    let root_dir = volume0.open_root_dir().unwrap();

    let _ = root_dir.iterate_dir(|entry| {
        println!("{:?}", core::str::from_utf8(entry.name.base_name()).unwrap());
    });
}

pub fn get_bmp<'a>(
    sdcard: SdCardType<'a>,
    icon_name: &'a str,
    buffer: &'a mut [u8]
) -> Bmp<'a, Rgb565> {
    let volume_mgr = VolumeManager::new(sdcard, DummyTimesource::default());
    let volume0 = volume_mgr.open_volume(VolumeIdx(0)).unwrap();

    // Open file
    let mut root = volume0.open_root_dir().unwrap();
    let mut file = root.open_file_in_dir(icon_name,Mode::ReadOnly).unwrap();
    let mut bytes_read = 0;
    let file_size = file.length() as usize;
    while bytes_read < file_size {
        match file.read(&mut buffer[bytes_read..file_size]) {
            Ok(0) => break, // End of file
            Ok(n) => bytes_read += n,
            _ => {}
        }
    }

    // Parse BMP
    let bmp = Bmp::from_slice(&buffer[..file_size]).expect("Failed to decode BMP");
    bmp
}