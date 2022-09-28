use crate::core::color::Color;
use crate::core::shape::{Shape, ShapeType};
use glam::Vec2;
use log::info;
use miniquad::*;

#[repr(C)]
pub struct Game {
    pub shapes: Vec<Shape>,
}

/// Core game loop.
impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        for shape in &mut self.shapes {
            shape.draw(ctx);
        }

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
        info!("Creating the `Game` object");
        Game {
            shapes: vec![Shape::new(
                ctx,
                ShapeType::Circle(8.0),
                Vec2::new(500.0, 500.0),
                Color::WHITE,
            )],
        }
    }
}
