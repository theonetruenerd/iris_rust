use zssh::{AuthMethod, PublicKey::Ed25519};
use zssh::ed25519_dalek::{VerifyingKey, PUBLIC_KEY_LENGTH};
use embedded_graphics::prelude::*;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::mono_font::{ascii::FONT_6X10, MonoTextStyle};
use embedded_graphics::text::Text;
use crate::drivers::display::draw_text;

const FULL_KEY: &[u8] = b"AAAAC3NzaC1lZDI1NTE5AAAAICHAM1KLDKxCvqUmGSNsKjc3/rGue0OHHBkX/NWSQ8n5";

pub fn setup_auth() -> AuthMethod {
    let decoded = &(FULL_KEY)[15..47];  // 32 bytes
    let actual_key: [u8; PUBLIC_KEY_LENGTH] = decoded.try_into().unwrap();
    let auth = AuthMethod::PublicKey(Ed25519 { public_key: VerifyingKey::from_bytes(&actual_key).unwrap() });
    auth
}

pub struct Terminal {
    pub rows: i32,
    pub cols: i32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub buffer: [[(char, Rgb565); 40]; 24], // cols, rows
    pub current_color: Rgb565,
}

impl Terminal {
    pub fn new() -> Self {
        Self {
            rows: 13,
            cols: 40,
            cursor_x: 0,
            cursor_y: 0,
            buffer: [[(' ', Rgb565::WHITE); 40]; 24], // Internal buffer can stay 24 for now, but rows set to 13
            current_color: Rgb565::WHITE,
        }
    }

    pub fn write_char(&mut self, c: char) {
        let color = self.current_color;
        match c {
            '\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
            '\r' => {
                self.cursor_x = 0;
            }
            '\x08' | '\x7f' => { // Backspace
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                    self.buffer[self.cursor_y as usize][self.cursor_x as usize] = (' ', color);
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = self.cols - 1;
                    self.buffer[self.cursor_y as usize][self.cursor_x as usize] = (' ', color);
                }
            }
            _ => {
                if self.cursor_x >= self.cols {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                }
                if self.cursor_y >= self.rows {
                    self.scroll_up();
                    self.cursor_y = self.rows - 1;
                }
                self.buffer[self.cursor_y as usize][self.cursor_x as usize] = (c, color);
                self.cursor_x += 1;
            }
        }

        if self.cursor_y >= self.rows {
            self.scroll_up();
            self.cursor_y = self.rows - 1;
        }
    }

    fn scroll_up(&mut self) {
        for y in 0..(self.rows - 1) as usize {
            self.buffer[y] = self.buffer[y + 1];
        }
        self.buffer[(self.rows - 1) as usize] = [(' ', Rgb565::WHITE); 40];
    }

    pub fn render<D>(&self, display: &mut D)
    where
        D: DrawTarget<Color = Rgb565>,
    {
        crate::drivers::display::clear_screen(display);
        // FONT_6X10 is 6 pixels wide, 10 pixels high.
        // Screen is 240x135.
        // 240 / 6 = 40 columns.
        // 135 / 10 = 13.5 rows. 
        for y in 0..self.rows as usize {
            for x in 0..self.cols as usize {
                let (c, color) = self.buffer[y][x];
                if c != ' ' {
                    let mut s = [0u8; 4];
                    let text = c.encode_utf8(&mut s);
                    draw_text(display, text, Point::new(x as i32 * 6, y as i32 * 10 + 10), color);
                }
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                // ANSI escape sequence
                if let Some('[') = chars.peek() {
                    chars.next();
                    let mut params = [0u16; 8];
                    let mut param_idx = 0;
                    let mut current_param = 0;
                    let mut has_params = false;

                    while let Some(&next_c) = chars.peek() {
                        if next_c.is_digit(10) {
                            current_param = current_param * 10 + (next_c as u16 - '0' as u16);
                            has_params = true;
                            chars.next();
                        } else if next_c == ';' {
                            params[param_idx] = current_param;
                            param_idx += 1;
                            current_param = 0;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if has_params {
                        params[param_idx] = current_param;
                        param_idx += 1;
                    }

                    if let Some(command) = chars.next() {
                        match command {
                            'm' => {
                                // Select Graphic Rendition
                                if param_idx == 0 {
                                    self.current_color = Rgb565::WHITE;
                                } else {
                                    for i in 0..param_idx {
                                        match params[i] {
                                            0 => self.current_color = Rgb565::WHITE,
                                            30 => self.current_color = Rgb565::BLACK,
                                            31 => self.current_color = Rgb565::RED,
                                            32 => self.current_color = Rgb565::GREEN,
                                            33 => self.current_color = Rgb565::YELLOW,
                                            34 => self.current_color = Rgb565::BLUE,
                                            35 => self.current_color = Rgb565::MAGENTA,
                                            36 => self.current_color = Rgb565::CYAN,
                                            37 => self.current_color = Rgb565::WHITE,
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            _ => {} // Ignore other commands for now
                        }
                    }
                    continue;
                }
            }
            self.write_char(c);
        }
    }
}