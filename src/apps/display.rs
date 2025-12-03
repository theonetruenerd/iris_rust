use esp_hal::gpio::Output;

pub fn turn_on_backlight(pin: &mut Output) {
    pin.set_high();
}

pub fn turn_off_backlight(pin: &mut Output) {
    pin.set_low();
}

pub fn toggle_backlight(pin: &mut Output) {
    if pin.is_set_low() {
        pin.set_high()
    } else {
        pin.set_low()
    }
}