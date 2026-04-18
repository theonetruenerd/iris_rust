use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use esp_hal::delay::Delay;
use esp_println::println;
use crate::drivers::display::{clear_screen, draw_text};
use crate::drivers::keyboard::Keyboard;

pub struct SettingsApp {
    pub selected_option: usize,
    pub options: [&'static str; 5],
    pub is_running: bool,
    pub wifi_connected: bool,
    pub brightness: u8,
    pub volume: u8,
}

impl SettingsApp {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: [
                "Wi-Fi",
                "Brightness",
                "Volume",
                "About",
                "Back",
            ],
            is_running: false,
            wifi_connected: false,
            brightness: 100,
            volume: 50,
        }
    }

    pub fn render<D>(&self, display: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "System Settings", Point::new(10, 15), Rgb565::CYAN);
        
        for (i, &option) in self.options.iter().enumerate() {
            let color = if i == self.selected_option {
                Rgb565::YELLOW
            } else {
                Rgb565::WHITE
            };
            
            let mut text = heapless::String::<32>::new();
            use core::fmt::Write;
            match option {
                "Wi-Fi" => {
                    let status = if self.wifi_connected { " (Connected)" } else { " (Disconnected)" };
                    write!(text, "{}{}", option, status).ok();
                }
                "Brightness" => {
                    write!(text, "{}: {}%", option, self.brightness).ok();
                }
                "Volume" => {
                    write!(text, "{}: {}%", option, self.volume).ok();
                }
                _ => {
                    write!(text, "{}", option).ok();
                }
            }
            
            draw_text(display, text.as_str(), Point::new(20, 35 + (i as i32 * 12)), color);
        }
    }

    pub fn run<D>(&mut self, display: &mut D, delay: &mut Delay, keyboard: &mut Keyboard)
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
                            "Wi-Fi" => self.wifi_menu(display, delay, keyboard),
                            "Brightness" => self.adjust_brightness(display, delay, keyboard),
                            "Volume" => self.adjust_volume(display, delay, keyboard),
                            "About" => self.show_about(display, delay, keyboard),
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

    fn wifi_menu<D>(&mut self, display: &mut D, delay: &mut Delay, keyboard: &mut Keyboard)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let mut wifi_running = true;
        let wifi_options = ["Scan Networks", "Saved Networks", "Disconnect", "Back"];
        let mut wifi_selected = 0;

        while wifi_running {
            clear_screen(display);
            draw_text(display, "Wi-Fi Settings", Point::new(10, 15), Rgb565::CYAN);
            
            for (i, &opt) in wifi_options.iter().enumerate() {
                let color = if i == wifi_selected { Rgb565::YELLOW } else { Rgb565::WHITE };
                draw_text(display, opt, Point::new(20, 35 + (i as i32 * 12)), color);
            }

            if let Some(key) = keyboard.get_key() {
                use crate::drivers::keyboard::Key;
                match key {
                    Key::Down | Key::Up => {
                        wifi_selected = (wifi_selected + 1) % wifi_options.len();
                        delay.delay_millis(200);
                    }
                    Key::Enter => {
                        match wifi_options[wifi_selected] {
                            "Scan Networks" => self.scan_wifi(display, delay, keyboard),
                            "Saved Networks" => self.saved_networks(display, delay, keyboard),
                            "Disconnect" => {
                                self.wifi_connected = false;
                                println!("Wi-Fi Disconnected");
                                delay.delay_millis(500);
                            }
                            "Back" => wifi_running = false,
                            _ => {}
                        }
                        delay.delay_millis(200);
                    }
                    Key::Backspace | Key::Esc => wifi_running = false,
                    _ => {}
                }
            }
            delay.delay_millis(50);
        }
    }

    fn scan_wifi<D>(&mut self, display: &mut D, delay: &mut Delay, keyboard: &mut Keyboard)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Scanning...", Point::new(10, 30), Rgb565::GREEN);
        delay.delay_millis(1000);

        let networks = ["Home_Router", "Office_WiFi", "Guest_Network"];
        let mut selected_net = 0;
        let mut scanning = true;

        while scanning {
            clear_screen(display);
            draw_text(display, "Select Network", Point::new(10, 15), Rgb565::GREEN);
            for (i, net) in networks.iter().enumerate() {
                let color = if i == selected_net { Rgb565::YELLOW } else { Rgb565::WHITE };
                draw_text(display, net, Point::new(20, 35 + (i as i32 * 12)), color);
            }

            if let Some(key) = keyboard.get_key() {
                use crate::drivers::keyboard::Key;
                match key {
                    Key::Down | Key::Up => {
                        selected_net = (selected_net + 1) % networks.len();
                        delay.delay_millis(200);
                    }
                    Key::Enter => {
                        self.connect_to(networks[selected_net], display, delay, keyboard);
                        scanning = false;
                        delay.delay_millis(200);
                    }
                    Key::Backspace | Key::Esc => scanning = false,
                    _ => {}
                }
            }
            delay.delay_millis(50);
        }
    }

    fn connect_to<D>(&mut self, ssid: &str, display: &mut D, delay: &mut Delay, _keyboard: &mut Keyboard)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        let mut msg = heapless::String::<64>::new();
        use core::fmt::Write;
        write!(msg, "Connecting to {}...", ssid).ok();
        draw_text(display, msg.as_str(), Point::new(10, 30), Rgb565::WHITE);
        
        // Mock connection process
        delay.delay_millis(1500);
        
        clear_screen(display);
        draw_text(display, "Connected!", Point::new(10, 30), Rgb565::GREEN);
        println!("Connected to Wi-Fi: {}", ssid);
        self.wifi_connected = true;
        
        // In a real app, we'd prompt for password if not saved, and save it to SD
        println!("Password for {} saved to /config/wifi.txt", ssid);
        
        delay.delay_millis(1000);
    }

    fn saved_networks<D>(&mut self, display: &mut D, delay: &mut Delay, _keyboard: &mut Keyboard)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Saved Networks:", Point::new(10, 15), Rgb565::CYAN);
        draw_text(display, "1. Home_Router", Point::new(20, 35), Rgb565::WHITE);
        draw_text(display, "Press any key...", Point::new(10, 100), Rgb565::CSS_GRAY);
        delay.delay_millis(1000);
    }

    fn adjust_brightness<D>(&mut self, display: &mut D, delay: &mut Delay, keyboard: &mut Keyboard)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let mut adjusting = true;
        while adjusting {
            clear_screen(display);
            draw_text(display, "Adjust Brightness", Point::new(10, 15), Rgb565::CYAN);
            
            let mut val_text = heapless::String::<16>::new();
            use core::fmt::Write;
            write!(val_text, "Value: {}%", self.brightness).ok();
            draw_text(display, val_text.as_str(), Point::new(20, 40), Rgb565::YELLOW);
            draw_text(display, "[0] Up  [Back] Exit", Point::new(10, 100), Rgb565::CSS_GRAY);

            if let Some((col, row)) = keyboard.scan() {
                match (col, row) {
                    (0, 0) => {
                        self.brightness = (self.brightness + 10) % 110;
                        println!("Brightness set to {}%", self.brightness);
                        delay.delay_millis(150);
                    }
                    (0, 2) => adjusting = false,
                    _ => {}
                }
            }
            delay.delay_millis(50);
        }
    }

    fn adjust_volume<D>(&mut self, display: &mut D, delay: &mut Delay, keyboard: &mut Keyboard)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        let mut adjusting = true;
        while adjusting {
            clear_screen(display);
            draw_text(display, "Adjust Volume", Point::new(10, 15), Rgb565::CYAN);
            
            let mut val_text = heapless::String::<16>::new();
            use core::fmt::Write;
            write!(val_text, "Value: {}%", self.volume).ok();
            draw_text(display, val_text.as_str(), Point::new(20, 40), Rgb565::YELLOW);
            draw_text(display, "[0] Up  [Back] Exit", Point::new(10, 100), Rgb565::CSS_GRAY);

            if let Some((col, row)) = keyboard.scan() {
                match (col, row) {
                    (0, 0) => {
                        self.volume = (self.volume + 10) % 110;
                        println!("Volume set to {}%", self.volume);
                        delay.delay_millis(150);
                    }
                    (0, 2) => adjusting = false,
                    _ => {}
                }
            }
            delay.delay_millis(50);
        }
    }

    fn show_about<D>(&mut self, display: &mut D, delay: &mut Delay, _keyboard: &mut Keyboard)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "Iris Cyberdeck v0.1", Point::new(10, 15), Rgb565::GREEN);
        draw_text(display, "Developed for ESP32-S3", Point::new(10, 35), Rgb565::WHITE);
        draw_text(display, "Hardware: Cardputer", Point::new(10, 50), Rgb565::WHITE);
        draw_text(display, "Press any key...", Point::new(10, 100), Rgb565::CSS_GRAY);
        delay.delay_millis(1000);
    }
}
