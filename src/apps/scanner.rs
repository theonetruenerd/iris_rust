use esp_hal::i2c::master::I2c;
use esp_hal::gpio::{Input, InputConfig, Pull, Output, Level, OutputConfig};
use esp_hal::delay::Delay;
use esp_println::println;
use esp_hal::Blocking;
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;

struct Writer<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> Writer<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, offset: 0 }
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.offset]).unwrap_or("")
    }
}

impl<'a> core::fmt::Write for Writer<'a> {
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

pub fn scan_i2c(i2c: &mut I2c<'_, Blocking>, display: &mut impl DrawTarget<Color = Rgb565>, delay: &mut Delay, keyboard: &mut crate::drivers::keyboard::Keyboard) {
    use crate::drivers::display::{clear_screen, draw_text};
    use embedded_graphics::primitives::{Rectangle, PrimitiveStyleBuilder, StyledDrawable};
    use embedded_graphics::geometry::{Point, Size};

    clear_screen(display);
    
    // Header
    let header_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::new(0, 0, 40)) // Dark Blue
        .build();
    Rectangle::new(Point::new(0, 0), Size::new(240, 20))
        .draw_styled(&header_style, display)
        .ok();
    draw_text(display, "🔍 I2C SCANNER", Point::new(10, 13), Rgb565::CYAN);

    draw_text(display, "Scanning I2C bus...", Point::new(10, 40), Rgb565::WHITE);
    println!("Scanning I2C bus...");
    
    let mut found = 0;
    let mut last_y = 55;

    for address in 1..127 {
        match i2c.write(address, &[]) {
            Ok(_) => {
                println!("Found I2C device at address: 0x{:02X}", address);
                let mut buf = [0u8; 32];
                let mut writer = Writer::new(&mut buf);
                use core::fmt::Write;
                write!(writer, "Found: 0x{:02X}", address).ok();
                draw_text(display, writer.as_str(), Point::new(20, last_y), Rgb565::GREEN);
                last_y += 12;
                found += 1;
            }
            Err(_) => {}
        }
        delay.delay_millis(5);
    }

    if found == 0 {
        draw_text(display, "No devices found.", Point::new(10, 60), Rgb565::RED);
    }

    // Footer
    let footer_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::new(10, 10, 10))
        .build();
    Rectangle::new(Point::new(0, 120), Size::new(240, 15))
        .draw_styled(&footer_style, display)
        .ok();
    draw_text(display, "Press any key to return...", Point::new(5, 130), Rgb565::new(31, 63, 31));

    loop {
        if keyboard.get_key().is_some() {
            break;
        }
        delay.delay_millis(50);
    }
    
    println!("I2C scan complete.");
}

pub fn scan_spi(
    sck_pin: esp_hal::peripherals::GPIO2,
    miso_pin: esp_hal::peripherals::GPIO1,
) {
    println!("Scanning SPI (basic detection)...");
    
    let mut sck = Output::new(sck_pin, Level::Low, OutputConfig::default());
    let miso = Input::new(miso_pin, InputConfig::default().with_pull(Pull::Up));
    let delay = Delay::new();

    let initial_state = miso.is_high();
    let mut changed = false;
    for _ in 0..10 {
        sck.set_high();
        delay.delay_millis(1);
        if miso.is_high() != initial_state {
            changed = true;
        }
        sck.set_low();
        delay.delay_millis(1);
        if miso.is_high() != initial_state {
            changed = true;
        }
    }

    if changed {
        println!("Detected potential SPI device (MISO toggled with SCK)");
    } else {
        println!("No SPI device detected via MISO toggling.");
    }
}
