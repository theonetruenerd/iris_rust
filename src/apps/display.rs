use embedded_graphics::geometry::Point;
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::Drawable;
use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_6X10};
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::Text;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::peripherals;
use esp_hal::spi::master::Config as SpiConfig;
use esp_hal::spi::master::Spi;
use esp_hal::spi::Mode as SpiMode;
use esp_hal::time::Rate;
use esp_hal::Blocking;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use mipidsi::options::{ColorInversion, Orientation, Rotation};
use mipidsi::{Builder, Display};
use tinybmp::Bmp;

// Consts
const APP_IMAGE_CENTER_X: i32 = 70;
const APP_IMAGE_CENTER_Y: i32 = 70;
const DISPLAY_WIDTH: i32 = 320;
const DISPLAY_HEIGHT: i32 = 240;


pub fn turn_on_backlight(pin: &mut Output) {
    pin.set_high();
}

pub fn turn_off_backlight(pin: &mut Output) {
    pin.set_low();
}

pub fn toggle_backlight(pin: &mut Output) {
    if pin.is_set_low() {
        pin.set_high()
    } else {
        pin.set_low()
    }
}

pub fn display_app_icon(
    image: Bmp<Rgb565>, mut display: Display<SpiInterface<ExclusiveDevice<Spi<Blocking>, Output, NoDelay>, Output>, ST7789, Output>)
{
    Image::new(&image, Point::new(APP_IMAGE_CENTER_X, APP_IMAGE_CENTER_Y)).draw(&mut display).unwrap();
}

pub fn display_text(
    text: &str, mut display: Display<SpiInterface<ExclusiveDevice<Spi<Blocking>, Output, NoDelay>, Output>, ST7789, Output>
) {
    let character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    Text::new(&text, Point::new(80, 100), character_style).draw(&mut display).unwrap();
}


pub fn display_background(
    gpio37: peripherals::GPIO37<'static>,
    gpio34: peripherals::GPIO34<'static>,
    gpio33: peripherals::GPIO33<'static>,
    spi2: peripherals::SPI2<'static>,
    gpio36: peripherals::GPIO36<'static>,
    gpio35: peripherals::GPIO35<'static>,
    bmp: Bmp<Rgb565>
)
{

    let image_w = 240;
    let image_h = 135;

    let x_position = (DISPLAY_WIDTH - image_w) / 2;
    let y_position = (DISPLAY_HEIGHT - image_h) / 2;


    let spi = Spi::new(
        spi2,
        SpiConfig::default()
            .with_frequency(Rate::from_mhz(10))
            .with_mode(SpiMode::_0),
    )
        .unwrap()
        .with_sck(gpio36)
        .with_mosi(gpio35);

    let mut buffer = [0u8; 512];

    let cs = Output::new(gpio37, Level::High, OutputConfig::default());
    let dc = Output::new(gpio34, Level::Low, OutputConfig::default());
    let rst = Output::new(gpio33, Level::Low, OutputConfig::default());
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let mut delay = Delay::new();
    let di = SpiInterface::new(spi_device, dc, &mut buffer);

    let mut display = Builder::new(ST7789, di)
        .reset_pin(rst)
        .invert_colors(ColorInversion::Inverted)
        .orientation(Orientation::new().rotate(Rotation::Deg90))
        .init(&mut delay)
        .unwrap();


    Image::new(&bmp, Point::new(x_position,y_position)).draw(&mut display).unwrap();
    display_text("Welcome, Tarun", display);
}