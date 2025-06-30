use ggez::{glam::Vec2, graphics::{self, Canvas, Color, Rect}};

pub const GRID_SIZE: f32 = 20.0;
pub const CELL_SIZE: f32 = 32.0;
pub const SCREEN_SIZE: f32 = GRID_SIZE * CELL_SIZE;

pub fn vec2_to_rect(v: Vec2) -> Rect {
    Rect::new(
        v.x,
        v.y,
        CELL_SIZE,
        CELL_SIZE
    )
}

pub fn wrap_around(v: Vec2) -> Vec2 {
    ((v % SCREEN_SIZE) + SCREEN_SIZE) % SCREEN_SIZE
}

pub fn draw_vec(canvas: &mut Canvas, v: Vec2, color: Color) {
    canvas.draw(&graphics::Quad, graphics::DrawParam::new().dest_rect(vec2_to_rect(v)).color(color));
}

pub fn draw_cell(canvas: &mut Canvas, x: f32, y: f32, color: Color) {
    draw_vec(canvas, Vec2::new(x, y), color);
}

pub fn get_random_pos() -> Vec2 {
    Vec2::new(
        rand::random_range(0..(GRID_SIZE as u32)) as f32 * CELL_SIZE,
        rand::random_range(0..(GRID_SIZE as u32)) as f32 * CELL_SIZE
    )
}