use core::fmt::Write;
use core::sync::atomic::{AtomicBool, Ordering};
use embassy_time::{Duration, Timer};
use embedded_graphics::image::Image;
use embedded_graphics::mono_font::ascii::FONT_9X18_BOLD;
use embedded_graphics::primitives::{Circle, PrimitiveStyle};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
    text::{Baseline, Text},
};
use esp_hal::{i2c::master::I2c, rng::Rng};
use heapless::spsc::Queue;
use heapless::String;
use ssd1306::{
    mode::BufferedGraphicsModeAsync, prelude::I2CInterface, size::DisplaySize128x64, Ssd1306Async,
};

use crate::sprites::{self};
use crate::{enemy::Enemy, player::Player};

pub type DisplayType<'a> = Ssd1306Async<
    I2CInterface<I2c<'a, esp_hal::Async>>,
    DisplaySize128x64,
    BufferedGraphicsModeAsync<DisplaySize128x64>,
>;

pub static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[derive(PartialEq, Clone, Copy)]
pub enum GameState {
    Menu,
    Playing,
    LevelCompleted,
    Dead,
}

pub struct Game<'a> {
    state: GameState,
    score: u32,
    player: Player,
    enemy: Enemy,
    display: DisplayType<'a>,
    rng: Rng,
}

impl<'a> Game<'a> {
    pub fn new(display: DisplayType<'a>, rng: Rng) -> Self {
        let (player, enemy) = Game::init_game_state(&display, rng);
        Self {
            state: GameState::Menu,
            score: 0,
            player,
            enemy,
            display,
            rng,
        }
    }

    fn init_game_state(display: &DisplayType<'a>, rng: Rng) -> (Player, Enemy) {
        (Game::spawn_player(display), Game::init_enemy(display, rng))
    }

    pub fn reset_game(&mut self) {
        self.score = 0;
        let (player, enemy) = Game::init_game_state(&self.display, self.rng);
        self.player = player;
        self.enemy = enemy;
    }

    fn init_enemy(display: &DisplayType, rng: Rng) -> Enemy {
        let screen_dims = display.dimensions();
        Enemy::new(screen_dims.0 as i32, screen_dims.1 as i32, rng)
    }

    pub async fn start(&mut self) {
        self.clear_display();
        let mut title_buff: String<64> = String::new();

        let mut prev_state;

        loop {
            title_buff.clear();

            prev_state = self.state;

            match self.state {
                GameState::Menu => {
                    if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
                        self.reset_game();
                        self.state = GameState::Playing;
                    }
                }
                GameState::Playing => {
                    if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
                        self.player.shoot();
                    }
                    self.enemy.update();
                    self.player.update();

                    self.enemy_collison();
                    self.player_collison();

                    self.draw_game();

                    self.display.flush().await.unwrap();
                }
                _ => {
                    if BUTTON_PRESSED.swap(false, Ordering::Relaxed) {
                        self.state = GameState::Menu;
                    }
                }
            }

            self.clear_display();

            match self.state {
                GameState::Menu => self.draw_welcome_screen(),
                GameState::Playing => self.draw_game(),
                GameState::Dead => {
                    self.draw_game_over();
                }
                GameState::LevelCompleted => {}
            }

            self.display.flush().await.unwrap();

            if prev_state == GameState::Playing && self.state == GameState::Dead {
                BUTTON_PRESSED.store(false, Ordering::Relaxed);
                // Wait and show the game over screen
                Timer::after(Duration::from_millis(500)).await;
                BUTTON_PRESSED.store(false, Ordering::Relaxed);
            }

            Timer::after(Duration::from_millis(15)).await;
        }
    }

    pub fn spawn_player(display: &DisplayType) -> Player {
        let screen_dims = display.dimensions();
        Player::new(screen_dims.0 as i32, screen_dims.1 as i32)
    }

    pub fn clear_display(&mut self) {
        self.display.clear_buffer();
        self.display.clear(BinaryColor::Off).unwrap();
    }

    fn draw_game(&mut self) {
        self.player.draw(&mut self.display);
        self.draw_enemy();
        self.print_score();
        self.print_lives();
        self.draw_universe();
    }

    pub fn draw_game_over(&mut self) {
        let mut score_text: String<32> = String::new();

        Image::new(&sprites::RAW_GAME_OVER, Point::new(16, 32))
            .draw(&mut self.display)
            .unwrap();

        write!(score_text, "Score: {}", self.score).unwrap();
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let text_width = score_text.len() as i32 * FONT_6X10.character_size.width as i32;
        // let text_height = FONT_6X10.character_size.height as i32;

        // // Get display dimensions
        let (width, _) = self.display.dimensions();

        // // Calculate top-left position to center the text
        let x = (width as i32 - text_width) / 2;
        // let y = (height as i32 - text_height) / 2;

        Text::with_baseline(&score_text, Point::new(x, 42), text_style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
    }

    pub fn draw_welcome_screen(&mut self) {
        Image::new(&sprites::RAW_BOW_ARROW, Point::new(16, 0))
            .draw(&mut self.display)
            .unwrap();

        let tile1 = "COSMIC";
        let title2 = "YUDH";

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_9X18_BOLD)
            .text_color(BinaryColor::On)
            .build();

        let x = sprites::RAW_BOW_ARROW.size().width as i32 + 30;
        Text::with_baseline(tile1, Point::new(x, 15), text_style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();

        Text::with_baseline(title2, Point::new(x + 3, 35), text_style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();

        self.draw_universe();
    }

    fn draw_universe(&mut self) {
        let stars = [
            // Star size : 1
            (10, 10, 1),
            (30, 5, 1),
            (50, 25, 1),
            (80, 10, 1),
            (100, 30, 1),
            (110, 5, 1),
            (60, 40, 1),
            (15, 25, 1),
            (25, 35, 1),
            (35, 45, 1),
            (55, 60, 1),
            (65, 20, 1),
            (75, 30, 1),
            (85, 40, 1),
            (105, 60, 1),
            (115, 15, 1),
            // Star size: 2
            (5, 50, 2),
            (20, 15, 2),
            (25, 58, 2),
            (90, 20, 2),
            (95, 50, 2),
            (123, 25, 2),
        ];

        for (x, y, size) in stars {
            Circle::new(Point::new(x, y), size)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(&mut self.display)
                .unwrap();
        }
    }

    fn print_score(&mut self) {
        let mut score_text: String<16> = String::new();
        write!(score_text, "Score: {}", self.score).unwrap();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        let text_width = score_text.len() as i32 * FONT_6X10.character_size.width as i32;
        let text_height = FONT_6X10.character_size.height as i32;

        let (width, height) = self.display.dimensions();

        // Calculate top-left position to center the text
        let x = (width as i32 - text_width) / 2;
        let y = height as i32 - text_height;

        Text::with_baseline(&score_text, Point::new(x, y), text_style, Baseline::Top)
            .draw(&mut self.display)
            .unwrap();
    }

    fn print_lives(&mut self) {
        let img_width: i32 = sprites::RAW_HEART.bounding_box().size.width as i32;

        let width = self.display.dimensions().0;
        let x = (width as i32 / 2 - img_width * self.player.lives as i32) + img_width;

        for i in 0..self.player.lives {
            let image = Image::new(
                &sprites::RAW_HEART,
                Point::new(x + (i as i32 * img_width), 0),
            );
            image.draw(&mut self.display).unwrap();
        }
    }

    fn draw_enemy(&mut self) {
        self.enemy.draw(&mut self.display);
    }

    fn enemy_collison(&mut self) {
        let enemy_bb = self.enemy.img.bounding_box();

        let mut new_queue = Queue::new();

        while let Some(bullet) = self.player.bullets.dequeue() {
            if detect_collison(bullet, enemy_bb) {
                self.score += 1;
            } else {
                new_queue.enqueue(bullet).unwrap();
            }
        }

        self.player.bullets = new_queue;
    }

    fn player_collison(&mut self) {
        let player_bb = self.player.img.bounding_box();

        let mut new_queue = Queue::new();

        while let Some(bullet) = self.enemy.bullets.dequeue() {
            if detect_collison(bullet.bounding_box(), player_bb) {
                self.player.lives = self.player.lives.saturating_sub(1);
            } else {
                new_queue.enqueue(bullet).unwrap();
            }
        }

        self.enemy.bullets = new_queue;

        if self.player.lives == 0 {
            self.state = GameState::Dead;
            // Clear any accident click during the game play
            BUTTON_PRESSED.store(false, Ordering::Relaxed);
        }
    }
}

pub fn detect_collison(a: Rectangle, b: Rectangle) -> bool {
    let intersection = a.intersection(&b);

    if intersection.size.width == 0 || intersection.size.height == 0 {
        return false;
    }

    true
}
