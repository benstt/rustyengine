use crate::core::color::Color;
use crate::core::shape::Shape;
use glam::Vec2;
use miniquad::*;

#[repr(C)]
pub struct Game {
    pub shape: Shape,
}

/// Core game loop.
impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        self.shape.draw(ctx);

        ctx.end_render_pass();
        ctx.commit_frame();
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
        Game {
            shape: Shape::new_square(ctx, Vec2::new(500.0, 500.0), 16.0, Color::BLUE),
        }
    }
}
