# Iris Notes

## Pinout

### SPM1423 (microphone)
- DAT: GPIO46 (data)
- CLK: GPIO43 (clock)
- VCC: 3.3V (power)
- GND: GND (ground)

### microSD Socket
- CS: GPIO12 (chip select)
- MOSI: GPIO14 (master output slave input)
- CLK: GPIO40 (clock)
- MISO: GPIO39 (master input slave output)

### ST7789V2 (screen)
- DISP_BL: GPIO38  (backlight)
- RST: GPIO33  (reset)
- RS: GPIO34 (register select)
- DAT: GPIO35 (data)
- SCK: GPIO36 (serial clock)
- CS: GPIO37 (chip select)

### RGB LED
- VDD: GPIO38

### Battery Detect ADC
- ADC: GPIO10 (analog digital converter)

### 74HC138 (Keyboard)
- Y7-Y0: GPIO7-GPIO3, GPIO15, GPIO13 (output lines)
- A2, A1, A0: GPIO11, GPIO9, GPIO8 (address inputs)

### NS4168 (speaker)
- BCLK: GPIO41 (bit clock)
- SDATA: GPIO42  (serial data)
- LRCLK: GPIO43 (left-right clock)

### IR
- TX: GPIO44 (transmit)

### Grove
- Black: GND
- Red: 5V
- Yellow: GPIO02
- White: GPIO01

## Code Optimisations to remember

Static rather than dynamic memory allocation where possible

Smallest valid integer types

Replace large buffers with streams

Heapless collections

Avoid duplicated code

Optimize string handling with &str rather than String

Review feature flags

Use ints rather than floats where possible