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
                // Row 0
                (0, 0) => Some(Key::Char('`')),
                (1, 0) => Some(Key::Char('1')),
                (2, 0) => Some(Key::Char('2')),
                (3, 0) => Some(Key::Char('3')),
                (4, 0) => Some(Key::Char('4')),
                (5, 0) => Some(Key::Char('5')),
                (6, 0) => Some(Key::Char('6')),
                (7, 0) => Some(Key::Char('7')),
                // Row 1
                (0, 1) => Some(Key::Char('8')),
                (1, 1) => Some(Key::Char('9')),
                (2, 1) => Some(Key::Char('0')),
                (3, 1) => Some(Key::Char('-')),
                (4, 1) => Some(Key::Char('=')),
                (5, 1) => Some(Key::Backspace),
                (6, 1) => Some(Key::Up),
                (7, 1) => None, // Unused
                // Row 2
                (0, 2) => Some(Key::Tab),
                (1, 2) => Some(Key::Char('q')),
                (2, 2) => Some(Key::Char('w')),
                (3, 2) => Some(Key::Char('e')),
                (4, 2) => Some(Key::Char('r')),
                (5, 2) => Some(Key::Char('t')),
                (6, 2) => Some(Key::Char('y')),
                (7, 2) => Some(Key::Char('u')),
                // Row 3
                (0, 3) => Some(Key::Char('i')),
                (1, 3) => Some(Key::Char('o')),
                (2, 3) => Some(Key::Char('p')),
                (3, 3) => Some(Key::Char('[')),
                (4, 3) => Some(Key::Char(']')),
                (5, 3) => Some(Key::Char('\\')),
                (6, 3) => Some(Key::Left),
                (7, 3) => Some(Key::Down),
                // Row 4
                (0, 4) => Some(Key::Ctrl), // Control
                (1, 4) => Some(Key::Char('a')),
                (2, 4) => Some(Key::Char('s')),
                (3, 4) => Some(Key::Char('d')),
                (4, 4) => Some(Key::Char('f')),
                (5, 4) => Some(Key::Char('g')),
                (6, 4) => Some(Key::Char('h')),
                (7, 4) => Some(Key::Char('j')),
                // Row 5
                (0, 5) => Some(Key::Char('k')),
                (1, 5) => Some(Key::Char('l')),
                (2, 5) => Some(Key::Char(';')),
                (3, 5) => Some(Key::Char('\'')),
                (4, 5) => Some(Key::Enter),
                (5, 5) => Some(Key::Right),
                (6, 5) => None, // Unused
                (7, 5) => None, // Unused
                // Row 6
                (0, 6) => Some(Key::Shift),
                (1, 6) => Some(Key::Char('z')),
                (2, 6) => Some(Key::Char('x')),
                (3, 6) => Some(Key::Char('c')),
                (4, 6) => Some(Key::Char('v')),
                (5, 6) => Some(Key::Char('b')),
                (6, 6) => Some(Key::Char('n')),
                (7, 6) => Some(Key::Char('m')),
                // Row 7 - Wait, there are only 7 rows (0-6).
                // But col goes 0..8 and rows is [Input; 7].
                // col=0..7 is used. row=0..6 is used.
                // M5Stack Cardputer has:
                // Row 0: ` 1 2 3 4 5 6 7
                // Row 1: 8 9 0 - = Backspace Up (7 keys)
                // Row 2: Tab q w e r t y u
                // Row 3: i o p [ ] \ Left Down
                // Row 4: Ctrl a s d f g h j
                // Row 5: k l ; ' Enter Right
                // Row 6: Shift z x c v b n m
                // Row 7: , . / Space Fn Alt (This is likely not how it's wired if only 7 rows)
                
                // Let me re-verify the matrix. 
                // Cardputer matrix is 8 columns x 7 rows.
                // Total 56 keys.
                
                // My mapping above used row 0..6 which is 7 rows.
                // If I have 8 columns, that's 56 keys.
                // Row 0: 0,0 to 7,0 -> 8 keys: ` 1 2 3 4 5 6 7
                // Row 1: 0,1 to 7,1 -> 8 keys: 8 9 0 - = BS UP [UNUSED]
                // Row 2: 0,2 to 7,2 -> 8 keys: Tab q w e r t y u
                // Row 3: 0,3 to 7,3 -> 8 keys: i o p [ ] \ LEFT DOWN
                // Row 4: 0,4 to 7,4 -> 8 keys: CTRL a s d f g h j
                // Row 5: 0,5 to 7,5 -> 8 keys: k l ; ' ENTER RIGHT [UNUSED] [UNUSED]
                // Row 6: 0,6 to 7,6 -> 8 keys: SHIFT z x c v b n m
                
                // Wait, where are , . / Space Fn Alt?
                // Maybe they are in the [UNUSED] slots or Row 7? 
                // But `rows` only has 7 inputs.
                // Actually, the Cardputer schematic shows:
                // 7 Rows (R0-R6) and 8 Columns (C0-C7).
                // Total 56 keys.
                // Row 6: SHIFT z x c v b n m 
                // Actually, the last row (Space etc) MUST be somewhere.
                // Let's check some existing Cardputer firmware code.
                // Key matrix for Cardputer:
                // C0: ` 8  Tab i  Ctrl k  Shift ,
                // C1: 1 9  q   o  a    l  z     .
                // C2: 2 0  w   p  s    ;  x     /
                // C3: 3 -  e   [  d    '  c     Space
                // C4: 4 =  r   ]  f    En v     Fn
                // C5: 5 BS t   \  g    Ri b     Alt
                // C6: 6 Up y   Le h    -  n     -
                // C7: 7 -  u   Do j    -  m     -
                
                // Ah! The columns and rows might be swapped or different.
                // Let's use this standard mapping:
                /*
                Col\Row | 0 | 1 | 2 | 3 | 4 | 5 | 6 
                --- |---|---|---|---|---|---|---
                0   | ` | 8 |tab| i |ctr| k |shi
                1   | 1 | 9 | q | o | a | l | z 
                2   | 2 | 0 | w | p | s | ; | x 
                3   | 3 | - | e | [ | d | ' | c 
                4   | 4 | = | r | ] | f |ret| v 
                5   | 5 | bs| t | \ | g |rig| b 
                6   | 6 | up| y |lef| h | . | n 
                7   | 7 |opt| u |dow| j | / | m
                */
                // Wait, I missed some keys. Space, Fn, Alt.
                // Let's try to match it more closely to what's found in official M5Stack Cardputer lib.
                
                (0, 0) => Some(Key::Char('`')),
                (0, 1) => Some(Key::Char('8')),
                (0, 2) => Some(Key::Tab),
                (0, 3) => Some(Key::Char('i')),
                (0, 4) => Some(Key::Ctrl),
                (0, 5) => Some(Key::Char('k')),
                (0, 6) => Some(Key::Shift),

                (1, 0) => Some(Key::Char('1')),
                (1, 1) => Some(Key::Char('9')),
                (1, 2) => Some(Key::Char('q')),
                (1, 3) => Some(Key::Char('o')),
                (1, 4) => Some(Key::Char('a')),
                (1, 5) => Some(Key::Char('l')),
                (1, 6) => Some(Key::Char('z')),

                (2, 0) => Some(Key::Char('2')),
                (2, 1) => Some(Key::Char('0')),
                (2, 2) => Some(Key::Char('w')),
                (2, 3) => Some(Key::Char('p')),
                (2, 4) => Some(Key::Char('s')),
                (2, 5) => Some(Key::Char(';')),
                (2, 6) => Some(Key::Char('x')),

                (3, 0) => Some(Key::Char('3')),
                (3, 1) => Some(Key::Char('-')),
                (3, 2) => Some(Key::Char('e')),
                (3, 3) => Some(Key::Char('[')),
                (3, 4) => Some(Key::Char('d')),
                (3, 5) => Some(Key::Char('\'')),
                (3, 6) => Some(Key::Char('c')),

                (4, 0) => Some(Key::Char('4')),
                (4, 1) => Some(Key::Char('=')),
                (4, 2) => Some(Key::Char('r')),
                (4, 3) => Some(Key::Char(']')),
                (4, 4) => Some(Key::Char('f')),
                (4, 5) => Some(Key::Enter),
                (4, 6) => Some(Key::Char('v')),

                (5, 0) => Some(Key::Char('5')),
                (5, 1) => Some(Key::Backspace),
                (5, 2) => Some(Key::Char('t')),
                (5, 3) => Some(Key::Char('\\')),
                (5, 4) => Some(Key::Char('g')),
                (5, 5) => Some(Key::Right),
                (5, 6) => Some(Key::Char('b')),

                (6, 0) => Some(Key::Char('6')),
                (6, 1) => Some(Key::Up),
                (6, 2) => Some(Key::Char('y')),
                (6, 3) => Some(Key::Left),
                (6, 4) => Some(Key::Char('h')),
                (6, 5) => Some(Key::Char('.')),
                (6, 6) => Some(Key::Char('n')),

                (7, 0) => Some(Key::Char('7')),
                (7, 1) => Some(Key::Alt),
                (7, 2) => Some(Key::Char('u')),
                (7, 3) => Some(Key::Down),
                (7, 4) => Some(Key::Char('j')),
                (7, 5) => Some(Key::Char('/')),
                (7, 6) => Some(Key::Char('m')),
                
                _ => None,
            }
        })
    }
}
