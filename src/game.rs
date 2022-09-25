use crate::core::shape::Shape;
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
            'z' => ctx.show_mouse(true),
            'x' => ctx.show_mouse(false),
            _ => (),
        }
    }
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            shape: Shape::new_square(ctx),
        }
    }
}
