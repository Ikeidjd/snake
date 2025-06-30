use ggez::{glam::Vec2, graphics::{Canvas, Color}};

use crate::{common, snake::Snake};

pub struct Apple {
    pos: Vec2,
    color: Color
}

impl Apple {
    pub fn new() -> Self {
        Self {
            pos: common::get_random_pos(),
            color: Color::RED
        }
    }

    pub fn reset(&mut self) {
        self.pos = common::get_random_pos();
    }

    pub fn update(&mut self, snake: &Snake) {
        if self.get_vec() == snake.get_vec() { self.reset(); }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        common::draw_vec(canvas, self.pos, self.color);
    }

    pub fn get_vec(&self) -> &Vec2 {
        &self.pos
    }
}