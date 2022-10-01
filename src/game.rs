use crate::examples::pong::{Ball, Enemy, Player, Pong};
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
        self.pong.key_down_event(ctx, keycode, _keymods, _repeat);
        match keycode {
            KeyCode::Escape => ctx.order_quit(),
            _ => (),
        }
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.pong.key_up_event(_ctx, keycode, _keymods)
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
    /// Constructs the game with all the necessary elements into it.
    pub fn new(ctx: &mut Context) -> Self {
        info!("creating the `Game` object");
        let (window_w, window_h) = ctx.screen_size();

        let player_position = Vec2::new(50.0, 80.0);
        let player = Player::new(ctx, player_position, 5.0);

        let ball_initial_velocity = Vec2::new(-4.0, 3.0);
        let ball = Ball::new(ctx, ball_initial_velocity);

        let enemy_position = Vec2::new(window_w - 50.0, window_h - 80.0);
        let enemy = Enemy::new(ctx, enemy_position, 5.0);

        let pong = Pong::new(player, ball, enemy);

        Self { pong }
    }
}
