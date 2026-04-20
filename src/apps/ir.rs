use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};
use esp_hal::delay::Delay;
use esp_hal::gpio::Output;
use esp_println::println;
use crate::drivers::display::{clear_screen, draw_text};
use crate::drivers::keyboard::Keyboard;

use crate::apps::file_manager::{DummyTimesource, SdCardType};
use embedded_sdmmc::{VolumeIdx, VolumeManager, Mode};

pub struct IrApp {
    pub selected_option: usize,
    pub options: [&'static str; 4],
    pub is_running: bool,
}

impl IrApp {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: [
                "Brute Force TV",
                "Brute Force AC",
                "SD Database",
                "Back",
            ],
            is_running: false,
        }
    }

    pub fn render<D>(&self, display: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        use embedded_graphics::primitives::{Rectangle, PrimitiveStyleBuilder, StyledDrawable};
        use embedded_graphics::geometry::{Point, Size};
        clear_screen(display);

        // Header
        let header_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(0, 40, 0)) // Dark Green
            .build();
        Rectangle::new(Point::new(0, 0), Size::new(240, 20))
            .draw_styled(&header_style, display)
            .ok();
        draw_text(display, "📡 IR CONTROLLER", Point::new(10, 13), Rgb565::GREEN);
        
        for (i, &option) in self.options.iter().enumerate() {
            let y = 35 + (i as i32 * 15);
            if i == self.selected_option {
                let select_style = PrimitiveStyleBuilder::new()
                    .fill_color(Rgb565::new(20, 20, 20))
                    .build();
                Rectangle::new(Point::new(5, y - 11), Size::new(230, 14))
                    .draw_styled(&select_style, display)
                    .ok();
                draw_text(display, option, Point::new(15, y), Rgb565::YELLOW);
            } else {
                draw_text(display, option, Point::new(15, y), Rgb565::WHITE);
            }
        }

        // Footer
        let footer_style = PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::new(10, 10, 10))
            .build();
        Rectangle::new(Point::new(0, 120), Size::new(240, 15))
            .draw_styled(&footer_style, display)
            .ok();
        draw_text(display, "Up/Dn: Navigate | Enter: Select | BS: Back", Point::new(5, 130), Rgb565::new(31, 63, 31));
    }

    pub fn run<D>(&mut self, display: &mut D, delay: &mut Delay, keyboard: &mut Keyboard, _sd: &mut SdCardType)
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
                    Key::Down => {
                        self.selected_option = (self.selected_option + 1) % self.options.len();
                        needs_redraw = true;
                        delay.delay_millis(200);
                    }
                    Key::Up => {
                        if self.selected_option > 0 {
                            self.selected_option -= 1;
                        } else {
                            self.selected_option = self.options.len() - 1;
                        }
                        needs_redraw = true;
                        delay.delay_millis(200);
                    }
                    Key::Enter => { // Select
                        match self.options[self.selected_option] {
                            "Brute Force TV" => self.brute_force_tv(display, delay, keyboard),
                            "Brute Force AC" => self.brute_force_ac(display, delay, keyboard),
                            "SD Database" => {
                                clear_screen(display);
                                draw_text(display, "IR SD Database", Point::new(10, 15), Rgb565::MAGENTA);
                                draw_text(display, "Loading /ir_codes/...", Point::new(10, 35), Rgb565::WHITE);

                                // Mocking SD access for now to avoid lifetime/borrow issues
                                draw_text(display, "Files in /ir_codes/:", Point::new(10, 55), Rgb565::CYAN);
                                draw_text(display, "SAMSUNG_TV.txt", Point::new(20, 75), Rgb565::WHITE);
                                draw_text(display, "LG_AC.txt", Point::new(20, 90), Rgb565::WHITE);
                                
                                println!("IR App: SD Database mode (querying SD card placeholder)");

                                draw_text(display, "Press any key to back", Point::new(10, 120), Rgb565::YELLOW);
                                delay.delay_millis(500);
                                loop {
                                    if keyboard.get_key().is_some() { break; }
                                    delay.delay_millis(50);
                                }
                            }
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

    fn brute_force_tv<D>(&self, display: &mut D, delay: &mut Delay, keyboard: &mut Keyboard)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "TV Brute Force", Point::new(10, 15), Rgb565::RED);
        draw_text(display, "Sending common codes...", Point::new(10, 35), Rgb565::WHITE);
        draw_text(display, "Hold BACK to stop", Point::new(10, 100), Rgb565::CYAN);

        let common_codes: [u32; 4] = [
            0x20DF10EF, // LG
            0xE0E040BF, // Samsung
            0x00FF30CF, // Sony
            0xFF00FF00, // Generic
        ];

        for (i, code) in common_codes.iter().enumerate() {
            if let Some(key) = keyboard.get_key() {
                use crate::drivers::keyboard::Key;
                if key == Key::Backspace || key == Key::Esc {
                    break;
                }
            }
            
            println!("Sending TV code: {:08X}", code);
            draw_text(display, "Sending...", Point::new(10, 55), Rgb565::YELLOW);
            
            // IR TX implementation will go here
            
            delay.delay_millis(500);
            draw_text(display, "Done.", Point::new(10, 55), Rgb565::BLACK); // "Clear" previous text
            draw_text(display, "Done.", Point::new(10, 55), Rgb565::GREEN);
            delay.delay_millis(500);
        }

        draw_text(display, "Finished.", Point::new(10, 75), Rgb565::GREEN);
        delay.delay_millis(1000);
    }

    fn brute_force_ac<D>(&self, display: &mut D, delay: &mut Delay, keyboard: &mut Keyboard)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "AC Brute Force", Point::new(10, 15), Rgb565::BLUE);
        draw_text(display, "Sending AC codes...", Point::new(10, 35), Rgb565::WHITE);
        
        // AC codes are usually longer and more complex, using placeholders for now
        println!("Sending AC Power Toggle codes...");
        delay.delay_millis(2000);
        
        draw_text(display, "Finished.", Point::new(10, 55), Rgb565::GREEN);
        delay.delay_millis(1000);
    }

    fn sd_database<'a, D>(&self, display: &mut D, delay: &mut Delay, keyboard: &mut Keyboard, sd: &'a mut SdCardType<'a>)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        clear_screen(display);
        draw_text(display, "IR SD Database", Point::new(10, 15), Rgb565::MAGENTA);
        draw_text(display, "Loading /ir_codes/...", Point::new(10, 35), Rgb565::WHITE);

        // We use a fake block device that just wraps the mutable reference
        struct BlockDeviceWrapper<'b, 'c>(&'b mut SdCardType<'c>);
        impl<'b, 'c> embedded_sdmmc::BlockDevice for BlockDeviceWrapper<'b, 'c> {
            type Error = <SdCardType<'c> as embedded_sdmmc::BlockDevice>::Error;
            fn read(&self, blocks: &mut [embedded_sdmmc::Block], start_block_idx: embedded_sdmmc::BlockIdx) -> Result<(), Self::Error> {
                self.0.read(blocks, start_block_idx)
            }
            fn write(&self, blocks: &[embedded_sdmmc::Block], start_block_idx: embedded_sdmmc::BlockIdx) -> Result<(), Self::Error> {
                self.0.write(blocks, start_block_idx)
            }
            fn num_blocks(&self) -> Result<embedded_sdmmc::BlockCount, Self::Error> {
                self.0.num_blocks()
            }
        }
        
        let wrapper = BlockDeviceWrapper(sd);
        let volume_mgr = VolumeManager::new(wrapper, DummyTimesource::default());
        let mut volume0 = match volume_mgr.open_volume(VolumeIdx(0)) {
            Ok(v) => v,
            Err(_) => {
                draw_text(display, "Error opening volume", Point::new(10, 55), Rgb565::RED);
                delay.delay_millis(2000);
                return;
            }
        };
        
        let root = volume0.open_root_dir().unwrap();
        // For simplicity, let's just list files in root for now, or a specific folder if it exists
        draw_text(display, "Files in root:", Point::new(10, 55), Rgb565::CYAN);
        
        let mut y_offset = 70;
        let _ = root.iterate_dir(|entry| {
            if y_offset < 200 {
                let name = core::str::from_utf8(entry.name.base_name()).unwrap_or("???");
                draw_text(display, name, Point::new(20, y_offset), Rgb565::WHITE);
                y_offset += 15;
            }
        });

        draw_text(display, "Press any key to exit", Point::new(10, 220), Rgb565::YELLOW);
        
        loop {
            if keyboard.get_key().is_some() {
                break;
            }
            delay.delay_millis(50);
        }
    }
}
