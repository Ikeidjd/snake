use std::collections::VecDeque;

use ggez::{glam::Vec2, graphics::{Canvas, Color}, input::keyboard::KeyCode, Context};

use crate::{apple::Apple, common::{self, CELL_SIZE, SCREEN_SIZE}};

pub struct Snake {
    body: VecDeque<(Vec2, Vec2)>,
    start_len: usize,
    speed: f32,
    movement: f32,
    next_dir: VecDeque<Vec2>,
    color: Color,
    wrap_around: bool
}

impl Snake {
    pub fn new(start_len: usize, speed: f32, wrap_around: bool) -> Self {
        Self {
            body: Self::create_body(start_len),
            start_len: start_len,
            speed: speed,
            movement: 0.0,
            next_dir: VecDeque::new(),
            color: Color::GREEN,
            wrap_around: wrap_around
        }
    }

    pub fn reset(&mut self, wrap_around: bool) {
        self.body = Self::create_body(self.start_len);
        self.movement = 0.0;
        self.next_dir = VecDeque::new();
        self.wrap_around = wrap_around;
    }

    fn create_body(n: usize) -> VecDeque<(Vec2, Vec2)> {
        let mut body = VecDeque::new();
        for _ in 0..n { body.push_front((Vec2::new(0.0, 0.0), Vec2::X)); }
        body
    }

    pub fn update(&mut self, ctx: &mut Context, apple: &Apple, game_over: &mut bool) {
        self.get_input(ctx);

        self.movement += self.speed * CELL_SIZE * ctx.time.delta().as_secs_f32();
        if self.movement >= CELL_SIZE {
            self.snap_to_grid(game_over);
            self.check_apple_collisions(apple);
            self.check_own_collisions(game_over);
        }
    }

    fn get_input(&mut self, ctx: &Context) {
        self.try_set_dir(ctx, KeyCode::W, Vec2::NEG_Y);
        self.try_set_dir(ctx, KeyCode::A, Vec2::NEG_X);
        self.try_set_dir(ctx, KeyCode::S, Vec2::Y);
        self.try_set_dir(ctx, KeyCode::D, Vec2::X);

        self.try_set_dir(ctx, KeyCode::Up, Vec2::NEG_Y);
        self.try_set_dir(ctx, KeyCode::Left, Vec2::NEG_X);
        self.try_set_dir(ctx, KeyCode::Down, Vec2::Y);
        self.try_set_dir(ctx, KeyCode::Right, Vec2::X);
    }

    fn snap_to_grid(&mut self, game_over: &mut bool) {
        self.movement %= CELL_SIZE;

        let new_head = *self.get_head() + self.get_dir() * CELL_SIZE;
        match self.wrap_around {
            true => {
                self.body.push_front((common::wrap_around(new_head), self.get_dir()));
                self.body.pop_back();
            }
            false => {
                if new_head.x < 0.0 || new_head.y < 0.0 || new_head.x >= SCREEN_SIZE || new_head.y >= SCREEN_SIZE {
                    self.movement = 0.0;
                    *game_over = true;
                }
                else {
                    self.body.push_front((new_head, self.get_dir()));
                    self.body.pop_back();
                }
            }
        }

        loop {
            if let Some(dir) = self.next_dir.pop_front() && !self.set_dir(dir) { continue; }
            break;
        }
    }

    fn check_apple_collisions(&mut self, apple: &Apple) {
        if self.get_vec() == apple.get_vec() {
            self.body.push_back(*self.body.back().unwrap());
            let back = self.body.back_mut().unwrap();
            back.0 -= back.1 * CELL_SIZE;
        }
    }

    fn check_own_collisions(&self, game_over: &mut bool) {
        for v in self.body.iter() {
            if v.0 == *self.get_head() && !std::ptr::eq(&v.0, self.get_head()) {
                *game_over = true;
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        for v in self.body.iter() {
            let moved = v.0 + v.1 * self.movement;
            common::draw_vec(canvas, moved, self.color);
            if !self.wrap_around { continue; }

            if moved.x + CELL_SIZE > SCREEN_SIZE { common::draw_cell(canvas, moved.x - SCREEN_SIZE, moved.y, self.color); }
            else if moved.y + CELL_SIZE > SCREEN_SIZE { common::draw_cell(canvas, moved.x, moved.y - SCREEN_SIZE, self.color); }
            else if moved.x < 0.0 { common::draw_cell(canvas, SCREEN_SIZE + moved.x, moved.y, self.color); }
            else if moved.y < 0.0 { common::draw_cell(canvas, moved.x, SCREEN_SIZE + moved.y, self.color); }
        }
    }

    pub fn get_vec(&self) -> &Vec2 {
        self.get_head()
    }

    pub fn score(&self) -> usize {
        self.body.len() - self.start_len
    }

    fn try_set_dir(&mut self, ctx: &Context, key: KeyCode, dir: Vec2) {
        if ctx.keyboard.is_key_just_pressed(key) { self.next_dir.push_back(dir) };
    }

    fn get_head(&self) -> &Vec2 {
        &self.body.front().unwrap().0
    }

    fn get_dir(&self) -> Vec2 {
        self.body.front().unwrap().1
    }

    fn set_dir(&mut self, dir: Vec2) -> bool {
        if self.get_dir().abs() == dir.abs() { return false; }
        self.body.front_mut().unwrap().1 = dir;
        true
    }
}