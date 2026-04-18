use esp_hal::gpio::{Input, Output};

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
}
