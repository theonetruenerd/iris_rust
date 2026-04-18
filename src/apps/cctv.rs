use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::StyledDrawable;
use crate::drivers::display::{draw_text, clear_screen};
use esp_hal::delay::Delay;
use esp_println::println;

pub struct CctvToolkit {
    pub selected_module: usize,
    pub modules: [&'static str; 5],
}

impl CctvToolkit {
    pub fn new() -> Self {
        Self {
            selected_module: 0,
            modules: [
                "Scan Local (LAN)",
                "Scan Unique IP",
                "Scan from FILE",
                "MJPEG Live Viewer",
                "Spycam Detector",
            ],
        }
    }

    pub fn render_menu<D>(&self, display: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "📹 CCTV Toolkit", Point::new(10, 20), Rgb565::CYAN);
        
        for (i, &module) in self.modules.iter().enumerate() {
            let color = if i == self.selected_module {
                Rgb565::YELLOW
            } else {
                Rgb565::WHITE
            };
            draw_text(display, module, Point::new(20, 35 + (i as i32 * 12)), color);
        }

        draw_text(display, ";/. Up/Down  Enter Select", Point::new(10, 110), Rgb565::new(31, 63, 31));
        draw_text(display, "Backspace Back", Point::new(10, 125), Rgb565::new(31, 63, 31));
    }

    pub fn run_module<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        match self.selected_module {
            0 => self.scan_lan(display, delay),
            1 => self.scan_ip(display, delay),
            2 => self.scan_file(display, delay),
            3 => self.mjpeg_viewer(display, delay),
            4 => self.spycam_detector(display, delay),
            _ => {}
        }
    }

    fn scan_lan<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "LAN Scan (ARP Sweep)", Point::new(10, 20), Rgb565::GREEN);
        draw_text(display, "Scanning 192.168.1.0/24...", Point::new(10, 40), Rgb565::WHITE);
        
        // Mock scanning
        for i in 1..10 {
            let mut buf = [0u8; 64];
            let mut writer = Writer::new(&mut buf);
            let _ = core::fmt::write(&mut writer, format_args!("Checking 192.168.1.{}", i * 10));
            let msg = writer.as_str();

            draw_text(display, msg, Point::new(10, 60 + (i as i32 * 15)), Rgb565::new(20, 40, 20));
            delay.delay_millis(300);
            
            // Allow early exit
            println!("LAN Scanning... (mock)");
        }
        
        draw_text(display, "Scan complete.", Point::new(10, 100), Rgb565::YELLOW);
        delay.delay_millis(2000);
    }

    fn scan_ip<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Single IP Scan", Point::new(10, 20), Rgb565::GREEN);
        draw_text(display, "Target: 192.168.1.23", Point::new(10, 40), Rgb565::WHITE);
        
        draw_text(display, "[1] Port Scan: 80, 554, 8080...", Point::new(10, 70), Rgb565::WHITE);
        delay.delay_millis(500);
        draw_text(display, "[2] Camera Heuristics...", Point::new(10, 85), Rgb565::WHITE);
        delay.delay_millis(500);
        draw_text(display, "[3] Brand: Hikvision", Point::new(10, 100), Rgb565::CYAN);
        delay.delay_millis(500);
        draw_text(display, "[4] CVE Hints: 12 found", Point::new(10, 115), Rgb565::RED);
        delay.delay_millis(500);
        
        draw_text(display, "Done. Backspace to return.", Point::new(10, 120), Rgb565::YELLOW);
        // In a real app, this would wait for input
        delay.delay_millis(2000);
    }

    fn scan_file<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Batch Scan from File", Point::new(10, 20), Rgb565::GREEN);
        draw_text(display, "Reading /evil/CCTV/CCTV_IP.txt", Point::new(10, 40), Rgb565::WHITE);
        delay.delay_millis(1000);
        draw_text(display, "Processing 5 targets...", Point::new(10, 60), Rgb565::WHITE);
        delay.delay_millis(2000);
        draw_text(display, "Batch finished.", Point::new(10, 100), Rgb565::YELLOW);
        delay.delay_millis(1000);
    }

    fn mjpeg_viewer<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        // UI for MJPEG Viewer
        draw_text(display, "MJPEG Live Viewer", Point::new(10, 10), Rgb565::BLACK); // Top bar area
        // Mock top bar
        let top_bar_style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(10, 20, 31))
            .build();
        embedded_graphics::primitives::Rectangle::new(Point::new(0, 0), Size::new(240, 25))
            .draw_styled(&top_bar_style, display)
            .ok();
            
        draw_text(display, "Cam_Front | 640x480 | MJPEG | 15fps", Point::new(5, 15), Rgb565::WHITE);

        // Mock video frame
        let frame_style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::new(20, 20, 20))
            .stroke_width(2)
            .build();
        embedded_graphics::primitives::Rectangle::new(Point::new(10, 40), Size::new(220, 70))
            .draw_styled(&frame_style, display)
            .ok();
        
        draw_text(display, "LIVE FEED MOCK", Point::new(100, 120), Rgb565::RED);

        draw_text(display, ",/. Prev/Next  r Res  ;/. Comp", Point::new(10, 120), Rgb565::WHITE);
        
        delay.delay_millis(3000);
    }

    fn spycam_detector<D>(&self, display: &mut D, delay: &mut Delay)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "📡 Spycam Detector", Point::new(10, 20), Rgb565::MAGENTA);
        draw_text(display, "Scanning for SSIDs/OUIs...", Point::new(10, 40), Rgb565::WHITE);

        // Mock detection
        for i in 0..5 {
            delay.delay_millis(800);
            if i == 2 {
                draw_text(display, "HIT: IPCAM_A8F2 (-38 dBm) [NEAR]", Point::new(10, 70), Rgb565::RED);
                // In real device, beep() and led_blink()
                println!("Spycam hit! Beep!");
            } else if i == 4 {
                draw_text(display, "HIT: PV-900 (Bilian OUI) (-45 dBm)", Point::new(10, 85), Rgb565::YELLOW);
            }
        }

        draw_text(display, "Scan paused. Enter to resume.", Point::new(10, 120), Rgb565::WHITE);
        delay.delay_millis(2000);
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