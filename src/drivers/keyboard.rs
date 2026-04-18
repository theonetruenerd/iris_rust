use esp_hal::gpio::{Input, Output};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Enter,
    Backspace,
    Esc,
    Tab,
    Shift,
    Fn,
    Alt,
    Ctrl,
}

pub struct Keyboard<'a> {
    // Address pins for 74HC138
    a0: Output<'a>,
    a1: Output<'a>,
    a2: Output<'a>,
    // Input pins for rows
    rows: [Input<'a>; 7],
}

impl<'a> Keyboard<'a> {
    pub fn new(
        a0: Output<'a>,
        a1: Output<'a>,
        a2: Output<'a>,
        rows: [Input<'a>; 7],
    ) -> Self {
        Self { a0, a1, a2, rows }
    }

    pub fn scan(&mut self) -> Option<(u8, u8)> {
        for col in 0..8 {
            // Set column address
            // Note: 74HC138 is active low on outputs, but we are just selecting the column
            if (col & 1) != 0 { self.a0.set_high(); } else { self.a0.set_low(); }
            if (col & 2) != 0 { self.a1.set_high(); } else { self.a1.set_low(); }
            if (col & 4) != 0 { self.a2.set_high(); } else { self.a2.set_low(); }

            // Small delay for 74HC138 to settle
            for _ in 0..100 { core::hint::spin_loop(); }
            
            for (row_idx, row_pin) in self.rows.iter().enumerate() {
                if row_pin.is_low() {
                    return Some((col as u8, row_idx as u8));
                }
            }
        }
        None
    }

    pub fn get_key(&mut self) -> Option<Key> {
        self.scan().and_then(|(col, row)| {
            match (col, row) {
                (0, 0) => Some(Key::Ctrl),
                (0, 1) => Some(Key::Char('z')),
                (0, 2) => Some(Key::Char('c')),
                (0, 3) => Some(Key::Char('b')),
                (0, 4) => Some(Key::Char('m')),
                (0, 5) => Some(Key::Down),
                (0, 6) => Some(Key::Char(' ')),

                (1, 0) => Some(Key::Shift),
                (1, 1) => Some(Key::Char('s')),
                (1, 2) => Some(Key::Char('f')),
                (1, 3) => Some(Key::Char('h')),
                (1, 4) => Some(Key::Char('k')),
                (1, 5) => Some(Key::Up),
                (1, 6) => Some(Key::Enter),

                (2, 0) => Some(Key::Char('q')),
                (2, 1) => Some(Key::Char('e')),
                (2, 2) => Some(Key::Char('t')),
                (2, 3) => Some(Key::Char('u')),
                (2, 4) => Some(Key::Char('o')),
                (2, 5) => Some(Key::Char('[')),
                (2, 6) => Some(Key::Char('\\')),

                (3, 0) => Some(Key::Char('1')),
                (3, 1) => Some(Key::Char('3')),
                (3, 2) => Some(Key::Char('5')),
                (3, 3) => Some(Key::Char('7')),
                (3, 4) => Some(Key::Char('9')),
                (3, 5) => Some(Key::Char('-')),
                (3, 6) => Some(Key::Backspace),

                (4, 0) => Some(Key::Fn),
                (4, 1) => Some(Key::Alt),
                (4, 2) => Some(Key::Char('x')),
                (4, 3) => Some(Key::Char('v')),
                (4, 4) => Some(Key::Char('n')),
                (4, 5) => Some(Key::Left),
                (4, 6) => Some(Key::Right),

                (5, 0) => Some(Key::Char(' ')),
                (5, 1) => Some(Key::Char('a')),
                (5, 2) => Some(Key::Char('d')),
                (5, 3) => Some(Key::Char('g')),
                (5, 4) => Some(Key::Char('j')),
                (5, 5) => Some(Key::Char('l')),
                (5, 6) => Some(Key::Char('\'')),

                (6, 0) => Some(Key::Tab),
                (6, 1) => Some(Key::Char('w')),
                (6, 2) => Some(Key::Char('r')),
                (6, 3) => Some(Key::Char('y')),
                (6, 4) => Some(Key::Char('i')),
                (6, 5) => Some(Key::Char('p')),
                (6, 6) => Some(Key::Char(']')),

                (7, 0) => Some(Key::Esc),
                (7, 1) => Some(Key::Char('2')),
                (7, 2) => Some(Key::Char('4')),
                (7, 3) => Some(Key::Char('6')),
                (7, 4) => Some(Key::Char('8')),
                (7, 5) => Some(Key::Char('0')),
                (7, 6) => Some(Key::Char('=')),

                _ => None,
            }
        })
    }

    pub fn get_char(&mut self) -> Option<char> {
        let (col, row) = self.scan()?;
        let key = self.get_key_from_matrix(col, row)?;
        
        let is_shift = self.is_modifier_pressed(Key::Shift);
        
        match key {
            Key::Char(c) => {
                if is_shift {
                    Some(self.apply_shift(c))
                } else {
                    Some(c)
                }
            }
            Key::Enter => Some('\n'),
            Key::Tab => Some('\t'),
            _ => None,
        }
    }

    fn get_key_from_matrix(&self, col: u8, row: u8) -> Option<Key> {
        match (col, row) {
            (0, 0) => Some(Key::Ctrl),
            (0, 1) => Some(Key::Char('z')),
            (0, 2) => Some(Key::Char('c')),
            (0, 3) => Some(Key::Char('b')),
            (0, 4) => Some(Key::Char('m')),
            (0, 5) => Some(Key::Down),
            (0, 6) => Some(Key::Char(' ')),

            (1, 0) => Some(Key::Shift),
            (1, 1) => Some(Key::Char('s')),
            (1, 2) => Some(Key::Char('f')),
            (1, 3) => Some(Key::Char('h')),
            (1, 4) => Some(Key::Char('k')),
            (1, 5) => Some(Key::Up),
            (1, 6) => Some(Key::Enter),

            (2, 0) => Some(Key::Char('q')),
            (2, 1) => Some(Key::Char('e')),
            (2, 2) => Some(Key::Char('t')),
            (2, 3) => Some(Key::Char('u')),
            (2, 4) => Some(Key::Char('o')),
            (2, 5) => Some(Key::Char('[')),
            (2, 6) => Some(Key::Char('\\')),

            (3, 0) => Some(Key::Char('1')),
            (3, 1) => Some(Key::Char('3')),
            (3, 2) => Some(Key::Char('5')),
            (3, 3) => Some(Key::Char('7')),
            (3, 4) => Some(Key::Char('9')),
            (3, 5) => Some(Key::Char('-')),
            (3, 6) => Some(Key::Backspace),

            (4, 0) => Some(Key::Fn),
            (4, 1) => Some(Key::Alt),
            (4, 2) => Some(Key::Char('x')),
            (4, 3) => Some(Key::Char('v')),
            (4, 4) => Some(Key::Char('n')),
            (4, 5) => Some(Key::Left),
            (4, 6) => Some(Key::Right),

            (5, 0) => Some(Key::Char(' ')),
            (5, 1) => Some(Key::Char('a')),
            (5, 2) => Some(Key::Char('d')),
            (5, 3) => Some(Key::Char('g')),
            (5, 4) => Some(Key::Char('j')),
            (5, 5) => Some(Key::Char('l')),
            (5, 6) => Some(Key::Char('\'')),

            (6, 0) => Some(Key::Tab),
            (6, 1) => Some(Key::Char('w')),
            (6, 2) => Some(Key::Char('r')),
            (6, 3) => Some(Key::Char('y')),
            (6, 4) => Some(Key::Char('i')),
            (6, 5) => Some(Key::Char('p')),
            (6, 6) => Some(Key::Char(']')),

            (7, 0) => Some(Key::Esc),
            (7, 1) => Some(Key::Char('2')),
            (7, 2) => Some(Key::Char('4')),
            (7, 3) => Some(Key::Char('6')),
            (7, 4) => Some(Key::Char('8')),
            (7, 5) => Some(Key::Char('0')),
            (7, 6) => Some(Key::Char('=')),

            _ => None,
        }
    }

    fn is_modifier_pressed(&mut self, modifier: Key) -> bool {
        // This is tricky because scan() returns the FIRST key found.
        // For true modifiers we'd need to check if multiple keys are down.
        // But for now, let's assume modifiers are handled by the caller or we check specifically.
        // Actually, the Cardputer keyboard usually supports multiple key presses if diodes are present.
        // Let's implement a specific check for a key.
        
        for col in 0..8 {
            if (col & 1) != 0 { self.a0.set_high(); } else { self.a0.set_low(); }
            if (col & 2) != 0 { self.a1.set_high(); } else { self.a1.set_low(); }
            if (col & 4) != 0 { self.a2.set_high(); } else { self.a2.set_low(); }
            for _ in 0..100 { core::hint::spin_loop(); }
            
            for (row_idx, row_pin) in self.rows.iter().enumerate() {
                if row_pin.is_low() {
                    if let Some(key) = self.get_key_from_matrix(col as u8, row_idx as u8) {
                        if key == modifier {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn apply_shift(&self, c: char) -> char {
        match c {
            '1' => '!', '2' => '@', '3' => '#', '4' => '$', '5' => '%', '6' => '^', '7' => '&', '8' => '*', '9' => '(', '0' => ')',
            '-' => '_', '=' => '+', '[' => '{', ']' => '}', '\\' => '|', ';' => ':', '\'' => '"', ',' => '<', '.' => '>', '/' => '?', '`' => '~',
            'q' => 'Q', 'w' => 'W', 'e' => 'E', 'r' => 'R', 't' => 'T', 'y' => 'Y', 'u' => 'U', 'i' => 'I', 'o' => 'O', 'p' => 'P',
            'a' => 'A', 's' => 'S', 'd' => 'D', 'f' => 'F', 'g' => 'G', 'h' => 'H', 'j' => 'J', 'k' => 'K', 'l' => 'L',
            'z' => 'Z', 'x' => 'X', 'c' => 'C', 'v' => 'V', 'b' => 'B', 'n' => 'N', 'm' => 'M',
            _ => c,
        }
    }
}
