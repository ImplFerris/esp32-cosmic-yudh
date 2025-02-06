# Cosmic Yudh: a Shooting Game written in Rust for ESP32 with OLED Display
 
A Shooting Game written in Rust for the ESP32 with an OLED display, using the Embassy framework.

## Hardware Requirements
- ESP32 (WROOM Dev Kit 1)
- SSD1306 OLED I2C 128x64 Display
- Joystick Module
- Jumper wires and breadboard
    
## Circuit

| ESP32 Pin | Component               |
|----------|-------------------------|
| GPIO 23  | SDA pin of OLED         |
| GPIO 18  | SCL pin of OLED         |
| 3.3V     | VCC pin of OLED         |
| GND      | GND pin of OLED         |
| 3.3V     | 5V pin of Joystick      |
| GPIO 32  | SW pin of Joystick      |
| GPIO 13  | VRX pin of Joystick     |
| GPIO 14  | VRY pin of Joystick (unused)     |

Note: I used only the VRX input for the player's movement(Up and Down) and won't be tracking VRY.


## Related Tutorials

You can refer to the following tutorials in the "impl Rust on ESP32" book to learn how to use the joystick and OLED with the ESP32.

- [Using Joystick Module with ESP32](https://esp32.implrust.com/joystick/index.html)
- [Using OLED Display Module with ESP32](https://esp32.implrust.com/oled/index.html)


## TODO

- Optional feature to use buttons instead of joystick
- Sound Effects
