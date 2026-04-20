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

pub fn draw_menu<D>(display: &mut D, items: &[&str], selected_idx: usize, battery: u8)
where
    D: DrawTarget<Color = Rgb565>,
{
    use embedded_graphics::primitives::{Rectangle, PrimitiveStyleBuilder, StyledDrawable};
    use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder};
    use embedded_graphics::text::Text;
    use core::fmt::Write;

    clear_screen(display);

    // Header
    let header_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::new(0, 20, 40)) // Dark Blue
        .build();
    Rectangle::new(Point::new(0, 0), Size::new(240, 20))
        .draw_styled(&header_style, display)
        .ok();
    
    draw_text(display, "IRIS OS - v0.1.0", Point::new(10, 13), Rgb565::CYAN);

    // Battery info in header
    let mut bat_buf = [0u8; 16];
    let mut bat_writer = BatWriter { buf: &mut bat_buf, offset: 0 };
    write!(bat_writer, "BAT: {}%", battery).ok();
    draw_text(display, bat_writer.as_str(), Point::new(180, 13), if battery > 20 { Rgb565::GREEN } else { Rgb565::RED });

    // Menu Items
    for (i, &item) in items.iter().enumerate() {
        let y = 35 + (i as i32 * 15);
        if i == selected_idx {
            // Draw selection box
            let select_style = PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::new(60, 60, 0)) // Darker Yellow/Gold
                .build();
            Rectangle::new(Point::new(5, y - 11), Size::new(230, 14))
                .draw_styled(&select_style, display)
                .ok();
            
            draw_text(display, item, Point::new(15, y), Rgb565::WHITE);
            // Draw a small indicator
            draw_text(display, ">", Point::new(225, y), Rgb565::YELLOW);
        } else {
            draw_text(display, item, Point::new(15, y), Rgb565::new(40, 40, 40)); // Grayish
        }
    }

    // Footer / Status Bar
    let footer_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::new(20, 20, 20))
        .build();
    Rectangle::new(Point::new(0, 120), Size::new(240, 15))
        .draw_styled(&footer_style, display)
        .ok();
    draw_text(display, "Up/Dn: Nav | Enter: Open", Point::new(10, 130), Rgb565::new(31, 63, 31));
}

struct BatWriter<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> BatWriter<'a> {
    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.offset]).unwrap_or("")
    }
}

impl<'a> core::fmt::Write for BatWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let len = bytes.len();
        if self.offset + len > self.buf.len() {
            return Err(core::fmt::Error);
        }
        self.buf[self.offset..self.offset + len].copy_from_slice(bytes);
        self.offset += len;
        Ok(())
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
    // Cardputer display is 240x135, but ST7789 might need offsets.
    // In main.rs, it's initialized with .display_size(135, 240) and .display_offset(40, 52).
    // The orientation is rotated 90 deg, so width=240, height=135.
    Rectangle::new(Point::new(0, 0), Size::new(240, 135))
        .draw_styled(&bg_style, display)
        .ok();
}