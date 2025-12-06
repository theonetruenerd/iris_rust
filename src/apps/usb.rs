use core::fmt::Write;
use esp_hal::peripherals::USB_DEVICE;
use esp_hal::usb_serial_jtag::UsbSerialJtag;

pub fn write_str(
    usb_device: USB_DEVICE,
    output_string: &str
) {
    let mut usb_serial = UsbSerialJtag::new(usb_device);
    usb_serial.write_str(output_string).expect("TODO: panic message");
}