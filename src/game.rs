use crate::core::shape::{Shape, ShapeType};
use miniquad::*;

pub struct Game {
    pub shape: Shape,
}

/// Core game loop.
impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear(Some((0., 0., 0., 0.)), None, None);
        self.shape.draw(ctx);
    }

    fn char_event(&mut self, ctx: &mut Context, character: char, _keymods: KeyMods, _repeat: bool) {
        match character {
            'z' => ctx.set_fullscreen(true),
            'x' => ctx.set_fullscreen(false),
            _ => (),
        }
    }
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            shape: Shape::new(ctx, ShapeType::SQUARE),
        }
    }
}
