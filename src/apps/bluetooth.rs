use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use esp_hal::delay::Delay;
use esp_println::println;
use core::fmt::Write;
use crate::drivers::display::{clear_screen, draw_text};

pub struct BluetoothApp {
    pub selected_option: usize,
    pub options: [&'static str; 5],
    pub is_running: bool,
}

impl BluetoothApp {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: [
                "BLE Scanner",
                "Apple Pop-up Spam",
                "Android Pair Spam",
                "Windows SwiftPair",
                "Back",
            ],
            is_running: false,
        }
    }

    pub fn render<D>(&self, display: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Bluetooth Tools", Point::new(10, 15), Rgb565::CYAN);
        
        for (i, &option) in self.options.iter().enumerate() {
            let color = if i == self.selected_option {
                Rgb565::YELLOW
            } else {
                Rgb565::WHITE
            };
            draw_text(display, option, Point::new(20, 35 + (i as i32 * 12)), color);
        }
    }

    pub fn run<D>(&mut self, display: &mut D, delay: &mut Delay, keyboard: &mut crate::drivers::keyboard::Keyboard)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        self.is_running = true;
        let mut needs_redraw = true;
        while self.is_running {
            if needs_redraw {
                self.render(display);
                needs_redraw = false;
            }
            
            if let Some(key) = keyboard.get_key() {
                use crate::drivers::keyboard::Key;
                match key {
                    Key::Down | Key::Up => { // Next/Down
                        self.selected_option = (self.selected_option + 1) % self.options.len();
                        needs_redraw = true;
                        delay.delay_millis(200);
                    }
                    Key::Enter => { // Select
                        match self.options[self.selected_option] {
                            "BLE Scanner" => self.ble_scanner(display, delay),
                            "Apple Pop-up Spam" => self.ble_spam(display, delay, "Apple Pop-up"),
                            "Android Pair Spam" => self.ble_spam(display, delay, "Android Pair"),
                            "Windows SwiftPair" => self.ble_spam(display, delay, "Windows SwiftPair"),
                            "Back" => self.is_running = false,
                            _ => {}
                        }
                        needs_redraw = true;
                        delay.delay_millis(200);
                    }
                    Key::Backspace | Key::Esc => { // Back
                        self.is_running = false;
                        delay.delay_millis(200);
                    }
                    _ => {}
                }
            }
            delay.delay_millis(10);
        }
    }

    fn ble_scanner<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Scanning BLE...", Point::new(10, 30), Rgb565::GREEN);
        println!("Bluetooth: Starting BLE Scan...");
        
        // Mock scan results
        delay.delay_millis(1500);
        clear_screen(display);
        draw_text(display, "Found 4 Devices:", Point::new(10, 15), Rgb565::GREEN);
        draw_text(display, "1. [TV] Samsung (70:2C:...)", Point::new(10, 35), Rgb565::WHITE);
        draw_text(display, "2. WH-1000XM4 (CC:98:...)", Point::new(10, 50), Rgb565::WHITE);
        draw_text(display, "3. Unknown (45:A2:...)", Point::new(10, 65), Rgb565::WHITE);
        draw_text(display, "4. iPhone (D1:55:...)", Point::new(10, 80), Rgb565::WHITE);
        draw_text(display, "Press any key...", Point::new(10, 110), Rgb565::CYAN);
        
        delay.delay_millis(1000);
    }

    fn ble_spam<D>(&self, display: &mut D, delay: &mut Delay, spam_type: &str)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        let mut title_buf = [0u8; 64];
        let mut writer = Writer::new(&mut title_buf);
        write!(writer, "{} Spam...", spam_type).ok();
        draw_text(display, writer.as_str(), Point::new(10, 30), Rgb565::RED);
        println!("Bluetooth: Starting {} BLE Spam...", spam_type);
        
        for i in 0..20 {
            let mut buf = [0u8; 32];
            let mut writer = Writer::new(&mut buf);
            write!(writer, "Packet {} sent...", i).ok();
            draw_text(display, writer.as_str(), Point::new(10, 50), Rgb565::WHITE);
            delay.delay_millis(200);
        }
        draw_text(display, "Done.", Point::new(10, 70), Rgb565::GREEN);
        delay.delay_millis(1000);
    }
}

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

impl<'a> Write for Writer<'a> {
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
