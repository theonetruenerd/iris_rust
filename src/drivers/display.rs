use embedded_graphics::Drawable;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
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
    Image::new(&image, Point::new(APP_IMAGE_CENTER_X, APP_IMAGE_CENTER_Y)).draw(&mut display).unwrap();
}

pub fn draw_text<D>(display: &mut D, text: &str, position: Point, color: Rgb565)
where
    D: DrawTarget<Color = Rgb565>,
{
    use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
    use embedded_graphics::text::Text;

    let style = MonoTextStyle::new(&FONT_6X10, color);
    Text::new(text, position, style).draw(display).ok();
}

pub fn draw_menu<D>(display: &mut D, items: &[&str], selected_idx: usize)
where
    D: DrawTarget<Color = Rgb565>,
{
    use embedded_graphics::primitives::{Rectangle, PrimitiveStyleBuilder, StyledDrawable};
    use embedded_graphics::Drawable;

    clear_screen(display);

    for (i, &item) in items.iter().enumerate() {
        let color = if i == selected_idx {
            Rgb565::YELLOW
        } else {
            Rgb565::WHITE
        };
        draw_text(display, item, Point::new(10, 20 + (i as i32 * 15)), color);
    }
}

pub fn clear_screen<D>(display: &mut D)
where
    D: DrawTarget<Color = Rgb565>,
{
    use embedded_graphics::primitives::{Rectangle, PrimitiveStyleBuilder, StyledDrawable};
    let bg_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
    Rectangle::new(Point::new(0, 0), Size::new(240, 135))
        .draw_styled(&bg_style, display)
        .ok();
}