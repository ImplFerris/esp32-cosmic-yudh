use core::sync::atomic::Ordering;

use atomic_enum::atomic_enum;
use embedded_graphics::{
    image::Image,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
};
use heapless::spsc::Queue;

use crate::game::DisplayType;

#[atomic_enum]
#[derive(PartialEq)]
pub enum PlayerDirection {
    Up,
    Down,
    Idle,
}

pub static PLAYER_DIRECTION: AtomicPlayerDirection =
    AtomicPlayerDirection::new(PlayerDirection::Idle);
const PLAYER_LIVES: u8 = 3;
const BULLET_SIZE: Size = Size::new(5, 2);
const MAX_PLAYER_BULLETS: usize = 1;
const BULLET_QUEUE_SIZE: usize = MAX_PLAYER_BULLETS + 1;
const INITIAL_BULLET_SPEED: i32 = 3;
const INITIAL_PLAYER_SPEED: i32 = 3;

pub struct Player {
    // Display Resolution
    screen_width: i32,
    screen_height: i32,
    // Player Data
    pub img: Image<'static, super::sprites::ImgRawType>,
    speed: i32,
    pub lives: u8,
    // Bullets Data
    pub bullets: Queue<Rectangle, BULLET_QUEUE_SIZE>,
    bullet_speed: i32,
}

impl Player {
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        let player_size = super::sprites::RAW_PLAYER_JET.bounding_box().size;

        let x = 10;
        let y = screen_height / 2 - player_size.height as i32 / 2;

        let position = Point::new(x, y);
        let img = Image::new(&super::sprites::RAW_PLAYER_JET, position);
        Self {
            img,
            lives: PLAYER_LIVES,
            screen_width,
            screen_height,
            speed: INITIAL_PLAYER_SPEED,
            bullets: Queue::new(),
            bullet_speed: INITIAL_BULLET_SPEED,
        }
    }

    pub fn draw(&self, display: &mut DisplayType) {
        self.img.draw(display).unwrap();
        self.draw_bullet(display);
    }

    pub fn draw_bullet(&self, display: &mut DisplayType) {
        self.bullets.iter().for_each(|bullet| {
            bullet
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
                .draw(display)
                .unwrap();
        });
    }

    pub fn update(&mut self) {
        self.update_position();
        self.update_bullet();
    }

    fn update_position(&mut self) {
        let direction = PLAYER_DIRECTION.load(Ordering::Relaxed);

        let bounding_box = self.img.bounding_box();

        let y = bounding_box.top_left.y;
        let bound_pad = 5;
        let max_bound = self.screen_height - bound_pad - bounding_box.size.height as i32;

        let new_y = match direction {
            PlayerDirection::Idle => y,
            PlayerDirection::Up => (y - self.speed).max(bound_pad),
            PlayerDirection::Down => (y + self.speed).min(max_bound),
        };

        let shift_by = Point::new(0, new_y - y);

        self.img = self.img.translate(shift_by);
    }

    fn update_bullet(&mut self) {
        if self.bullets.is_empty() {
            return;
        }

        let mut new_queue = Queue::<Rectangle, BULLET_QUEUE_SIZE>::new();

        while let Some(bullet) = self.bullets.dequeue() {
            let bullet = bullet.translate(Point::new(self.bullet_speed, 0));
            if bullet.top_left.x > self.screen_width {
                continue;
            }
            new_queue.enqueue(bullet).unwrap()
        }
        self.bullets = new_queue;
    }

    pub fn shoot(&mut self) {
        if self.bullets.is_full() {
            return;
        }

        let bounding_box = self.img.bounding_box();
        let player_pos = self.img.bounding_box().top_left;
        let player_size = bounding_box.size;

        let pos = Point::new(
            player_pos.x + player_size.width as i32,
            player_pos.y + player_size.height as i32 / 2,
        );
        let bullet = Rectangle::new(pos, BULLET_SIZE);

        self.bullets.enqueue(bullet).unwrap();
    }
}
