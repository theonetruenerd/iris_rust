use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use esp_hal::delay::Delay;
use esp_println::println;
use core::fmt::Write;
use crate::drivers::display::{clear_screen, draw_text};

pub struct MarauderApp {
    pub selected_option: usize,
    pub options: [&'static str; 5],
    pub is_running: bool,
}

impl MarauderApp {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: [
                "Scan APs",
                "Deauth Attack",
                "Beacon Spam",
                "Rickroll Spam",
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
        draw_text(display, "Wi-Fi Marauder", Point::new(10, 15), Rgb565::RED);
        
        for (i, &option) in self.options.iter().enumerate() {
            let color = if i == self.selected_option {
                Rgb565::YELLOW
            } else {
                Rgb565::WHITE
            };
            draw_text(display, option, Point::new(20, 35 + (i as i32 * 15)), color);
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
            
            if let Some((col, row)) = keyboard.scan() {
                match (col, row) {
                    (0, 0) => { // Next/Down
                        self.selected_option = (self.selected_option + 1) % self.options.len();
                        needs_redraw = true;
                        delay.delay_millis(200);
                    }
                    (0, 1) => { // Select
                        match self.options[self.selected_option] {
                            "Scan APs" => self.scan_aps(display, delay),
                            "Deauth Attack" => self.deauth_attack(display, delay),
                            "Beacon Spam" => self.beacon_spam(display, delay),
                            "Rickroll Spam" => self.rickroll_spam(display, delay),
                            "Back" => self.is_running = false,
                            _ => {}
                        }
                        needs_redraw = true;
                        delay.delay_millis(200);
                    }
                    (0, 2) => { // Back
                        self.is_running = false;
                        delay.delay_millis(200);
                    }
                    _ => {}
                }
            }
            delay.delay_millis(10);
        }
    }

    fn scan_aps<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Scanning...", Point::new(10, 30), Rgb565::GREEN);
        println!("Marauder: Starting Wi-Fi Scan...");
        
        // Mock scan results
        delay.delay_millis(1500);
        clear_screen(display);
        draw_text(display, "Found 3 APs:", Point::new(10, 15), Rgb565::GREEN);
        draw_text(display, "1. Home_WiFi (-65dBm)", Point::new(10, 35), Rgb565::WHITE);
        draw_text(display, "2. Starbucks (-80dBm)", Point::new(10, 50), Rgb565::WHITE);
        draw_text(display, "3. Hidden_Network (-72dBm)", Point::new(10, 65), Rgb565::WHITE);
        draw_text(display, "Press any key...", Point::new(10, 110), Rgb565::CYAN);
        
        delay.delay_millis(1000);
    }

    fn deauth_attack<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Deauthenticating...", Point::new(10, 30), Rgb565::RED);
        println!("Marauder: Sending Deauth packets...");
        
        for i in 0..10 {
            let mut buf = [0u8; 32];
            let mut writer = Writer::new(&mut buf);
            write!(writer, "Packet {} sent...", i).ok();
            draw_text(display, writer.as_str(), Point::new(10, 50), Rgb565::WHITE);
            delay.delay_millis(300);
        }
        draw_text(display, "Done.", Point::new(10, 70), Rgb565::GREEN);
        delay.delay_millis(1000);
    }

    fn beacon_spam<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Beacon Spamming...", Point::new(10, 30), Rgb565::YELLOW);
        println!("Marauder: Starting Beacon Spam...");
        delay.delay_millis(2000);
        draw_text(display, "Spamming 50 SSIDs", Point::new(10, 50), Rgb565::WHITE);
        delay.delay_millis(1000);
    }

    fn rickroll_spam<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Never gonna give...", Point::new(10, 30), Rgb565::MAGENTA);
        println!("Marauder: Rickrolling nearby devices...");
        delay.delay_millis(2000);
        draw_text(display, "You up!", Point::new(10, 50), Rgb565::WHITE);
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
