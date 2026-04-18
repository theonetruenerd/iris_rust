use esp_hal::i2c::master::I2c;
use esp_hal::gpio::{Input, InputConfig, Pull, Output, Level, OutputConfig};
use esp_hal::delay::Delay;
use esp_println::println;
use esp_hal::Blocking;

pub fn scan_i2c(i2c: &mut I2c<'_, Blocking>) {
    println!("Scanning I2C bus...");
    for address in 1..127 {
        match i2c.write(address, &[]) {
            Ok(_) => println!("Found I2C device at address: 0x{:02X}", address),
            Err(_) => {}
        }
    }
    println!("I2C scan complete.");
}

pub fn scan_spi(
    sck_pin: esp_hal::peripherals::GPIO2,
    miso_pin: esp_hal::peripherals::GPIO1,
) {
    println!("Scanning SPI (basic detection)...");
    
    let mut sck = Output::new(sck_pin, Level::Low, OutputConfig::default());
    let miso = Input::new(miso_pin, InputConfig::default().with_pull(Pull::Up));
    let delay = Delay::new();

    let initial_state = miso.is_high();
    let mut changed = false;
    for _ in 0..10 {
        sck.set_high();
        delay.delay_millis(1);
        if miso.is_high() != initial_state {
            changed = true;
        }
        sck.set_low();
        delay.delay_millis(1);
        if miso.is_high() != initial_state {
            changed = true;
        }
    }

    if changed {
        println!("Detected potential SPI device (MISO toggled with SCK)");
    } else {
        println!("No SPI device detected via MISO toggling.");
    }
}
