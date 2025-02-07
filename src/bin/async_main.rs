#![no_std]
#![no_main]

use cosmic_yudh::{audio::AudioEffect, control, game::Game};
use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::{ledc::Ledc, prelude::*, rng::Rng};
use log::info;
use ssd1306::{
    mode::DisplayConfigAsync, prelude::DisplayRotation, size::DisplaySize128x64,
    I2CDisplayInterface, Ssd1306Async,
};

#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    let timer0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1);
    esp_hal_embassy::init(timer0.timer0);

    info!("Embassy initialized!");

    // Track Joystick movements in background
    spawner
        .spawn(control::track_joystick(
            peripherals.GPIO13,
            peripherals.GPIO14,
            peripherals.ADC2,
        ))
        .unwrap();

    // Track Joystick button state
    spawner
        .spawn(control::button_press(peripherals.GPIO32))
        .unwrap();

    // Initialize the OLED Display
    let i2c = esp_hal::i2c::master::I2c::new(
        peripherals.I2C0,
        esp_hal::i2c::master::Config {
            frequency: 400.kHz(),
            timeout: Some(100),
        },
    )
    .with_scl(peripherals.GPIO18)
    .with_sda(peripherals.GPIO23)
    .into_async();
    let interface = I2CDisplayInterface::new(i2c);
    // initialize the display
    let mut display = Ssd1306Async::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().await.unwrap();

    // sound effects module
    let ledc = Ledc::new(peripherals.LEDC);
    let audio = AudioEffect::new(ledc, peripherals.GPIO33);

    let rng = Rng::new(peripherals.RNG);
    // Initialize the Game
    let mut game = Game::new(display, rng, audio);
    game.start().await;
}
