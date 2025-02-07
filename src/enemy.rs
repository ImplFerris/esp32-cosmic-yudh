use embedded_graphics::{
    image::Image,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};
use esp_hal::rng::Rng;
use heapless::spsc::Queue;

use crate::{game::DisplayType, sprites::RAW_PLANET_KILLER};

const MAX_ENEMY_BULLETS: usize = 4;
pub const BULLET_QUEUE_SIZE: usize = MAX_ENEMY_BULLETS + 1;
const INITIAL_BULLET_VELOCITY: i32 = -3;
const INITIAL_ENEMY_VELOCITY: i32 = 1;

pub struct Enemy {
    pub img: Image<'static, super::sprites::ImgRawType>,
    velocity: i32,
    screen_height: i32,
    rng: Rng,
    // Bullet data
    pub bullets: Queue<Circle, BULLET_QUEUE_SIZE>,
    bullet_velocity: i32,
    // Current max bullet
    max_bullet: usize,
}

impl Enemy {
    pub fn new(screen_width: i32, screen_height: i32, rng: Rng) -> Self {
        let x = screen_width - (RAW_PLANET_KILLER.size().width * 15 / 10) as i32;
        let y = screen_height / 2 - RAW_PLANET_KILLER.size().height as i32 / 2;

        let position = Point::new(x, y);
        let img = Image::new(&super::sprites::RAW_PLANET_KILLER, position);

        Self {
            img,
            rng,
            velocity: INITIAL_ENEMY_VELOCITY,
            screen_height,
            bullets: Queue::new(),
            bullet_velocity: INITIAL_BULLET_VELOCITY,
            max_bullet: 1,
        }
    }

    pub fn increase_level(&mut self) {
        self.max_bullet = (self.max_bullet + 1).min(MAX_ENEMY_BULLETS);
        self.velocity += if self.velocity < 0 { -1 } else { 1 };
    }

    pub fn update(&mut self) {
        self.update_position();
        self.update_bullet();
        let rand_num = self.rng.random();
        if rand_num % 2 == 0 {
            self.shoot();
        }
    }

    pub fn update_position(&mut self) {
        let y = self.img.bounding_box().top_left.y;
        let mut new_y = y + self.velocity;

        let img_size = self.img.bounding_box().size;
        let max_bound = self.screen_height - img_size.height as i32;

        if new_y < 0 || new_y >= max_bound {
            self.velocity = -self.velocity;
            new_y = y + self.velocity;
        }

        let shift_by = Point::new(0, new_y - y);

        self.img = self.img.translate(shift_by);
    }

    pub fn shoot(&mut self) {
        if self.bullets.is_full() || self.bullets.len() >= self.max_bullet {
            return;
        }

        let bounding_box = self.img.bounding_box();

        if let Some(last_bullet) = self.bullets.iter().last() {
            // Check if the new bullet's position is too close to the last bullet's position
            if (bounding_box.top_left.x - last_bullet.top_left.x).abs() < 5
                && (bounding_box.top_left.y - last_bullet.top_left.y).abs() < 5
            {
                return;
            }
        }

        let rand_num = self.rng.random() % 5;
        let bullet_size = 5 + rand_num;

        let enemy_pos = self.img.bounding_box().top_left;
        let enemy_size = bounding_box.size;

        let pos = Point::new(
            enemy_pos.x - enemy_size.width as i32,
            enemy_pos.y + enemy_size.height as i32 / 2,
        );
        let bullet = Circle::new(pos, bullet_size);

        self.bullets.enqueue(bullet).unwrap();
    }

    fn update_bullet(&mut self) {
        if self.bullets.is_empty() {
            return;
        }

        let mut new_queue = Queue::<Circle, BULLET_QUEUE_SIZE>::new();

        while let Some(bullet) = self.bullets.dequeue() {
            let bullet = bullet.translate(Point::new(self.bullet_velocity, 0));
            if bullet.top_left.x <= 0 {
                continue;
            }
            new_queue.enqueue(bullet).unwrap()
        }
        self.bullets = new_queue;
    }

    pub fn draw(&self, display: &mut DisplayType) {
        self.img.draw(display).unwrap();
        self.draw_bullet(display);
    }

    pub fn draw_bullet(&self, display: &mut DisplayType) {
        self.bullets.iter().for_each(|bullet| {
            bullet
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
                .draw(display)
                .unwrap();
        });
    }
}
