use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use esp_hal::Blocking;
use esp_hal::gpio::Output;
use esp_hal::spi::master::Spi;
use mipidsi::Display;
use mipidsi::interface::SpiInterface;
use mipidsi::models::ST7789;
use tinybmp::Bmp;

// Consts
const APP_IMAGE_CENTER_X: i32 = 70;
const APP_IMAGE_CENTER_Y: i32 = 70;

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
    Image::new(&image, Point::new(APP_IMAGE_CENTER_X, APP_IMAGE_CENTER_X)).draw(&mut display).unwrap();
}