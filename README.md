# Iris

Iris is a custom firmware for the ESP32-S3, specifically optimized for the M5Stack Cardputer.

## Prerequisites

Before you can flash Iris, you need to have the Rust environment set up for ESP32 development.

1. **Install Rust:**
   Follow the instructions at [rustup.rs](https://rustup.rs/).

2. **Install the ESP Toolchain:**
   Iris uses the `esp` toolchain. You can install it using `espup`:
   ```powershell
   cargo install espup
   espup install
   ```

3. **Install espflash:**
   You will need `espflash` to flash the firmware to your device:
   ```powershell
   cargo install espflash
   ```

## Flashing

To flash Iris to your Cardputer, connect it to your computer via USB and run the following command from the project root:

```powershell
cargo espflash flash --release --monitor
```

### Options:
- `--release`: Builds the firmware in release mode (recommended for performance).
- `--monitor`: Opens a serial monitor after flashing to view log output.
- `--board esp32s3`: If the board is not automatically detected, you might need to specify it.

## Features

For a detailed list of features and the project roadmap, see [feature_roadmap.md](feature_roadmap.md).

## Hardware Pinout

Hardware pinout details can be found in [notes.md](notes.md).
