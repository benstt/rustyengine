use crate::core::color::Color;
use crate::core::shape::{Shape, ShapeType};
use crate::examples::pong::{Player, Pong};
use glam::Vec2;
use log::info;
use miniquad::*;

#[repr(C)]
pub struct Game {
    pub pong: Pong,
}

/// Core game loop.
impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) {
        self.pong.update(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        self.pong.draw(ctx);

        ctx.end_render_pass();
        ctx.commit_frame();
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => ctx.order_quit(),
            _ => (),
        }
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
        info!("creating the `Game` object");
        let player_position = Vec2::new(100.0, 40.0);
        let player = Player::new(ctx, player_position, 3.0);
        let pong = Pong::new(player);

        Self { pong }
    }
}
