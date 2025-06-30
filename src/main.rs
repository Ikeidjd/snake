use std::{env, fs};

use ggez::{event, glam::Vec2, graphics::{self, Canvas, Color, TextLayout}, input::keyboard::KeyCode, Context, GameResult};

use crate::{apple::Apple, common::SCREEN_SIZE, snake::Snake};

mod common;
mod snake;
mod apple;

struct GameState {
    snake: Snake,
    apple: Apple,

    prev_score_no_wrap_around: usize,
    high_score_no_wrap_around: usize,
    prev_score_wrap_around: usize,
    high_score_wrap_around: usize,
    wrap_around: bool,

    game_over: bool,
    game_over_text: graphics::Text
}

impl GameState {
    /// Our new function will set up the initial state of our game.
    pub fn new() -> Self {
        let file: Vec<usize> = fs::read_to_string("res/save.txt").unwrap_or("".to_owned()).split(" ").map(|num| num.parse().unwrap_or(0)).collect();
        let wrap_around = *file.get(4).unwrap_or(&0) != 0;
        GameState {
            snake: Snake::new(3, 8.0, wrap_around),
            apple: Apple::new(),

            prev_score_no_wrap_around: *file.get(0).unwrap_or(&0),
            high_score_no_wrap_around: *file.get(1).unwrap_or(&0),
            prev_score_wrap_around: *file.get(2).unwrap_or(&0),
            high_score_wrap_around: *file.get(3).unwrap_or(&0),
            wrap_around: wrap_around,

            game_over: false,
            game_over_text: graphics::Text::new("GAME OVER").set_scale(100.0).set_layout(TextLayout::center()).clone()
        }
    }

    fn reset(&mut self, wrap_around: bool) {
        self.game_over = false;
        self.update_scores();
        self.snake.reset(wrap_around);
        self.apple.reset();
        self.wrap_around = wrap_around;
    }

    fn update_scores(&mut self) {
        if self.snake.score() == 0 { return; }
        *self.prev_score() = self.snake.score();
        *self.high_score() = (*self.high_score()).max(self.snake.score());
    }

    fn prev_score(&mut self) -> &mut usize {
        match self.wrap_around {
            true => &mut self.prev_score_wrap_around,
            false => &mut self.prev_score_no_wrap_around
        }
    }

    fn high_score(&mut self) -> &mut usize {
        match self.wrap_around {
            true => &mut self.high_score_wrap_around,
            false => &mut self.high_score_no_wrap_around
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if (self.game_over || self.snake.score() == 0) && ctx.keyboard.is_key_just_released(KeyCode::F1)  { self.reset(!self.wrap_around); }
        else if self.game_over {
            if ctx.keyboard.is_key_just_released(KeyCode::R) { self.reset(self.wrap_around); }
            else { return Ok(()); }
        }

        self.snake.update(ctx, &self.apple, &mut self.game_over);
        self.apple.update(&self.snake);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        self.snake.draw(&mut canvas);
        self.apple.draw(&mut canvas);

        let score = format!("SCORE: {}", self.snake.score());
        let prev_score = format!("PREV SCORE: {}", self.prev_score());
        let high_score = format!("HIGH SCORE: {}", self.high_score());
        let wrap_around = format!("WRAP AROUND: {}", self.wrap_around as usize);

        canvas.draw(&graphics::Text::new(score), Vec2::new(0.0, 0.0));
        canvas.draw(&graphics::Text::new(prev_score), Vec2::new(0.0, 16.0));
        canvas.draw(&graphics::Text::new(high_score), Vec2::new(0.0, 32.0));
        canvas.draw(&graphics::Text::new(wrap_around), Vec2::new(0.0, 48.0));

        if self.game_over {
            canvas.draw(&graphics::Quad, graphics::DrawParam::new().dest_rect(graphics::Rect::new(0.0, 0.0, SCREEN_SIZE, SCREEN_SIZE)).color(graphics::Color::new(0.0, 0.0, 0.0, 0.9)));
            canvas.draw(&self.game_over_text, Vec2::new(SCREEN_SIZE / 2.0, SCREEN_SIZE / 2.0));
        }

        canvas.finish(ctx)?;
        Ok(())
    }
    
    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, ggez::GameError> {
        self.update_scores();
        fs::write("res/save.txt", format!("{} {} {} {} {}", self.prev_score_no_wrap_around, self.high_score_no_wrap_around, self.prev_score_wrap_around, self.high_score_wrap_around, self.wrap_around as usize)).expect("Something went wrong while saving.");
        Ok(false)
    }
}

fn main() -> GameResult {
    unsafe { env::set_var("RUST_BACKTRACE", "1") };
    let (ctx, events_loop) = ggez::ContextBuilder::new("snake", "ikeidjd")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE, SCREEN_SIZE))
        .build()?;

    let state = GameState::new();
    event::run(ctx, events_loop, state)
}