# Feature Comparison and Roadmap

This document outlines the features present in alternative firmware (Bruce/EvilCardputer), the currently scoped but unimplemented features in Iris (based on project structure), and suggested "cyberdeck" appropriate features.

## 1. Features in Bruce/EvilCardputer (Not yet in Iris)

Bruce and EvilCardputer are highly developed firmwares for the Cardputer/ESP32-S3 platforms, focusing on offensive security and utility.

### Offensive Security (Radio/Wireless)
- **Wi-Fi Marauder Integration:** Deauthentication attacks, beacon spamming, PMKID sniffing.
- **RF/Sub-GHz Tools:** If equipped with CC1101 or similar, replaying signals, jammer, or rolling code analysis (though Cardputer lacks built-in Sub-GHz, Bruce often supports external modules).
- **Bluetooth/BLE Spamming:** BLE advertisement flooding (e.g., Apple/Android pairing pop-up spam).
- **BadUSB/USB Rubber Ducky:** More advanced script execution and payload delivery via USB HID.

### Infrared (IR)
- **IR Remote Learning:** Record and playback IR signals from remotes.
- **IR Bruteforcing:** Cycling through common power codes for TVs/ACs.

### System & UI
- **Customizable Themes:** UI skins and color schemes.
- **Web Interface:** Managing files or triggering actions via a local Wi-Fi captive portal or web server.
- **OTA Updates:** Over-the-air firmware updates.
- **Deep Sleep/Power Management:** More granular control over power states.

---

## 2. Scoped but Unimplemented Features (Iris Project Structure)

Based on the `src/apps/` directory, the following features are planned but currently consist of empty or skeletal files:

- **Wireless Connectivity:**
  - `wifi.rs`: Wi-Fi connectivity and tools.
  - `bluetooth.rs`: Bluetooth/BLE functionality.
- **Surveillance & Vision:**
  - `camera.rs`: Support for an external or integrated camera.
  - `cctv.rs`: Likely for viewing or interacting with IP cameras/streams.
  - `thermal.rs`: Support for thermal imaging modules (like MLX90640).
- **Sensing & Environment:**
  - `gas_sensor.rs`: Environmental monitoring (e.g., MQ-series or SGP sensors).
  - `heart_rate.rs`: Health monitoring.
  - `gsr.rs`: Galvanic Skin Response (often used in DIY "lie detectors").
  - `mag_switch.rs`: Magnetic/Reed switch sensing.
  - `radar.rs`: Support for Grove Doppler Radar.
- **Interaction:**
  - `voice_recognition.rs`: Local or cloud-based voice command processing.
  - `gesture.rs`: Gesture-based control (e.g., PAJ7620U2).
  - `joystick.rs`: External joystick input.
- **Prototyping:**
  - `breadboard.rs`: Likely a utility for interacting with the Grove/GPIO header for quick tests.
- **NFC:**
  - `nfc.rs`: Reading/Writing NFC tags.

---

## 3. Cyberdeck Appropriate Suggestions

Features that would enhance the "Cyberdeck" feel and utility of Iris:

### Network & Communication
- **LoRa Messenger:** Peer-to-peer encrypted messaging using LoRa modules (Reyax/Ra-02) for long-range, off-grid communication.
- **Packet Sniffer/Monitor Mode:** Basic Wi-Fi traffic visualization (SSIDs, signal strength, channel usage).
- **SSH Terminal Enhancements:** Full ANSI color support and better keyboard navigation for the existing SSH app.
- **Serial Terminal:** A "Generic Serial" app to talk to other microcontrollers or networking gear via the Grove port or USB.

### Data & Cryptography
- **Password Manager:** Encrypted database stored on the SD card, accessible via a master password.
- **TOTP Authenticator:** Generating 2FA codes (requires a stable RTC or manual time sync).
- **Text Editor:** A simple "Notepad" style app for taking notes on the go.

### Hardware Interface
- **I2C/SPI Scanner:** Automatically detect addresses of devices plugged into the Grove port.
- **Signal Generator/Logic Analyzer:** Use GPIOs to output PWM/Square waves or capture low-frequency digital signals.
- **Hardware Info:** Detailed system stats (Heap usage, CPU temp, Internal Flash usage).

### Utility
- **Flashlight Mode:** Turn the screen white at max brightness or control the RGB LED.
- **Calculators:** Standard and Programmer (Hex/Bin/Dec) calculators.
- **Clock/Alarm:** Utilizing the NS4168 speaker for alerts.
