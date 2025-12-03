use esp_hal::analog::adc::{AdcConfig, Adc, Attenuation};
use esp_hal::peripherals::{ADC1, GPIO10};

pub fn get_battery_percentage(
    adc1: ADC1,
    gpio10: GPIO10
) -> f32 {
    let mut adc1_config: AdcConfig<ADC1> = AdcConfig::new();
    let mut adc = Adc::new(adc1, AdcConfig::default());

    let mut battery_pin = adc1_config.enable_pin(gpio10, Attenuation::_11dB);

    let battery_raw: u16 = nb::block!(adc.read_oneshot(&mut battery_pin)).unwrap();
    let battery_voltage = (battery_raw as f32 * 3.3) / 4095.0;
    let battery_percentage = ((battery_voltage - 2.5) / (4.2 - 2.5) * 100.0).max(0.0).min(100.0);
    battery_percentage
}