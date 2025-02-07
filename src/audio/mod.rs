use esp_hal::delay::Delay;
use esp_hal::gpio::{GpioPin, Level, Output};
use esp_hal::ledc::{channel, timer, HighSpeed, Ledc};
use esp_hal::prelude::*;
pub mod music;

const BUZZER_PIN: u8 = 33;
pub struct AudioEffect<'a> {
    ledc: Ledc<'a>,
    buzzer: Output<'static>,
    delay: Delay,
}

impl<'a> AudioEffect<'a> {
    pub fn new(ledc: Ledc<'a>, buzzer_pin: GpioPin<BUZZER_PIN>) -> Self {
        let buzzer = Output::new(buzzer_pin, Level::Low);

        Self {
            ledc,
            buzzer,
            delay: Delay::new(),
        }
    }

    pub fn play_tone(&mut self, note: u32, duration: u32) {
        let freq = note.Hz();
        let mut hstimer0 = self.ledc.timer::<HighSpeed>(timer::Number::Timer0);
        hstimer0
            .configure(timer::config::Config {
                duty: timer::config::Duty::Duty10Bit,
                // clock_source: timer::LSClockSource::APBClk,
                clock_source: timer::HSClockSource::APBClk,
                frequency: freq,
            })
            .unwrap();

        let mut channel0 = self
            .ledc
            .channel(channel::Number::Channel0, &mut self.buzzer);
        channel0
            .configure(channel::config::Config {
                timer: &hstimer0,
                duty_pct: 50,
                pin_config: channel::config::PinConfig::PushPull,
            })
            .unwrap();

        self.delay.delay_millis(duration);
        channel0.set_duty(0).unwrap();
    }
}
