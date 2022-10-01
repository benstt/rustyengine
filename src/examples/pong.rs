use glam::Vec2;
use miniquad::{Context, EventHandler, KeyCode};

use crate::core::{
    color::Color,
    input_handler::InputHandler,
    shape::{Shape, ShapeType},
};

pub struct Collider {
    pub position: Vec2,
    pub size: Vec2,
    pub shape: Shape,
}

impl Collider {
    pub fn new(ctx: &mut Context, x: f32, y: f32, w: f32, h: f32) -> Self {
        let position = Vec2::new(x, y);
        let size = Vec2::new(w, h);

        Self {
            position,
            size,
            shape: Shape::new(ctx, ShapeType::RectangleLines(w, h), position, Color::GREEN),
        }
    }

    pub const fn pos(&self) -> (f32, f32) {
        (self.position.x, self.position.y)
    }

    pub const fn size(&self) -> (f32, f32) {
        (self.size.x, self.size.y)
    }

    pub fn collides_with(&self, collider: &Collider) -> bool {
        let (sx, sy) = self.pos();
        let (sw, sh) = self.size();
        let (ox, oy) = collider.pos();
        let (ow, oh) = collider.size();

        sx < ox + ow && sx + sw > ox && sy < oy + oh && sy + sh > oy
    }

    pub fn move_by(&mut self, vel: Vec2) {
        self.position += vel;
        self.shape.position += vel;
    }
}

impl EventHandler for Collider {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        self.shape.draw(ctx);
    }
}

pub struct Pong {
    player: Player,
    ball: Ball,
    enemy: Enemy,
}

pub struct Player {
    position: Vec2,
    movement_speed: f32,
    shape: Shape,
    collider: Collider,
    input_handler: InputHandler,
}

pub struct Ball {
    position: Vec2,
    initial_velocity: Vec2,
    shape: Shape,
    collider: Collider,
}

pub struct Enemy {
    position: Vec2,
    movement_speed: f32,
    shape: Shape,
    collider: Collider,
}

impl Player {
    pub fn new(ctx: &mut Context, position: Vec2, movement_speed: f32) -> Self {
        let (w, h) = (16.0, 48.0);

        Self {
            position,
            movement_speed,
            shape: Shape::new(ctx, ShapeType::Rectangle(w, h), position, Color::WHITE),
            collider: Collider::new(ctx, position.x, position.y, w, h),
            input_handler: InputHandler::new(),
        }
    }

    pub const fn collider(&self) -> &Collider {
        &self.collider
    }

    fn move_y(&mut self, y: f32) {
        self.shape.position.y += y;
        self.collider.move_by((0.0, y).into());
        self.position.y += y;
    }
}

impl EventHandler for Player {
    fn update(&mut self, ctx: &mut Context) {
        let (_, window_height) = ctx.screen_size();
        let padding = 15.0;

        self.input_handler.update(ctx);

        let mut movement_dir = Vec2::ZERO;
        if self.input_handler.is_key_pressed(KeyCode::W) && self.position.y > padding {
            movement_dir.y -= 1.0 * self.movement_speed;
        }

        if self.input_handler.is_key_pressed(KeyCode::S)
            && self.position.y + self.shape.size.y < window_height - padding
        {
            movement_dir.y += 1.0 * self.movement_speed;
        }

        self.move_y(movement_dir.y);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.shape.draw(ctx);
        self.collider.draw(ctx);
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: miniquad::KeyCode,
        _keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.input_handler
            .key_down_event(_ctx, keycode, _keymods, _repeat);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: miniquad::KeyMods) {
        self.input_handler.key_up_event(_ctx, keycode, _keymods);
    }
}

impl Ball {
    pub fn new(ctx: &mut Context, initial_velocity: Vec2) -> Self {
        let (window_width, window_height) = ctx.screen_size();
        let position = Vec2::new(window_width / 2.0, window_height / 2.0);

        Self {
            position,
            initial_velocity,
            shape: Shape::new(ctx, ShapeType::Circle(16.0), position, Color::WHITE),
            collider: Collider::new(ctx, position.x, position.y, 16.0, 16.0),
        }
    }

    pub const fn collider(&self) -> &Collider {
        &self.collider
    }

    fn move_by(&mut self, vel: Vec2) {
        self.shape.position += vel;
        self.collider.move_by(vel);
        self.position += vel;
    }
}

impl EventHandler for Ball {
    fn update(&mut self, ctx: &mut Context) {
        let (window_width, window_height) = ctx.screen_size();
        let left_bound_x = self.position.x - 16.0;
        let right_bound_x = self.position.x + 16.0;
        let top_bound_y = self.position.y - 16.0;
        let bottom_bound_y = self.position.y + 16.0;

        if right_bound_x > window_width || left_bound_x < 0.0 {
            self.initial_velocity.x *= -1.0;
        }

        if bottom_bound_y > window_height || top_bound_y < 0.0 {
            self.initial_velocity.y *= -1.0;
        }

        self.move_by(self.initial_velocity)
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.shape.draw(ctx);
        self.collider.draw(ctx);
    }
}

impl Enemy {
    pub fn new(ctx: &mut Context, position: Vec2, movement_speed: f32) -> Self {
        let (w, h) = (16.0, 48.0);

        Self {
            position,
            movement_speed,
            shape: Shape::new(ctx, ShapeType::Rectangle(w, h), position, Color::WHITE),
            collider: Collider::new(ctx, position.x, position.y, w, h),
        }
    }

    pub const fn collider(&self) -> &Collider {
        &self.collider
    }

    fn move_y(&mut self, y: f32) {
        self.shape.position.y += y;
        self.collider.move_by((0.0, y).into());
        self.position.y += y;
    }
}

impl EventHandler for Enemy {
    fn update(&mut self, ctx: &mut Context) {
        let (_, window_height) = ctx.screen_size();
        let padding = 15.0;

        if self.position.y < padding
            || self.position.y + self.shape.size.y > window_height - padding
        {
            debug!("pos + size: {}", self.position.y + self.shape.size.y);
            debug!("windowh - pad: {}", window_height - padding);
            self.movement_speed *= -1.0;
        }

        self.move_y(self.movement_speed);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.shape.draw(ctx);
        self.collider.draw(ctx);
    }
}

impl Pong {
    pub fn new(player: Player, ball: Ball, enemy: Enemy) -> Self {
        Self {
            player,
            ball,
            enemy,
        }
    }
}

impl EventHandler for Pong {
    fn update(&mut self, ctx: &mut Context) {
        let player_collider = self.player.collider();
        let ball_collider = self.ball.collider();
        let enemy_collider = self.enemy.collider();

        if ball_collider.collides_with(player_collider)
            || ball_collider.collides_with(enemy_collider)
        {
            debug!("Collided!");
            self.ball.initial_velocity.x *= -1.0;
        }

        self.player.update(ctx);
        self.ball.update(ctx);
        self.enemy.update(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.player.draw(ctx);
        self.ball.draw(ctx);
        self.enemy.draw(ctx);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.player.key_down_event(ctx, keycode, _keymods, _repeat)
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: miniquad::KeyMods) {
        self.player.key_up_event(_ctx, keycode, _keymods)
    }
}
