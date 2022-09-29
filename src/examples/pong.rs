use gilrs::{Axis, Button, Gamepad};
use glam::Vec2;
use miniquad::{Context, EventHandler};

use crate::core::{
    color::Color,
    input_handler::{InputHandler, Stick},
    shape::{Shape, ShapeType},
};

pub struct Pong {
    player: Player,
}

pub struct Player {
    position: Vec2,
    movement_speed: f32,
    shape: Shape,
    input_handler: InputHandler,
}

impl Player {
    pub fn new(ctx: &mut Context, position: Vec2, movement_speed: f32) -> Self {
        Self {
            position,
            movement_speed,
            shape: Shape::new(
                ctx,
                ShapeType::Rectangle(16.0, 48.0),
                position,
                Color::WHITE,
            ),
            input_handler: InputHandler::new(),
        }
    }

    fn move_y(&mut self, y: f32) {
        self.shape.position.y += y;
        self.position.y += y;
    }
}

impl EventHandler for Player {
    fn update(&mut self, ctx: &mut Context) {
        self.input_handler.update(ctx);

        let mut movement_dir = Vec2::ZERO;

        let stick_threshold = 0.5;
        let (_, mut left_stick_y_amount) = self.input_handler.axis_values(Stick::Left);

        if left_stick_y_amount.abs() < stick_threshold {
            left_stick_y_amount = 0.0;
        } else {
            left_stick_y_amount = left_stick_y_amount.signum();
        }

        movement_dir.y += left_stick_y_amount * self.movement_speed;
        self.move_y(movement_dir.y);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.shape.draw(ctx);
    }
}

impl Pong {
    pub fn new(player: Player) -> Self {
        Self { player }
    }
}

impl EventHandler for Pong {
    fn update(&mut self, ctx: &mut Context) {
        self.player.update(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) {
        self.player.draw(ctx);
    }
}
