use std::path::Path;

use crate::core::{editor::Editor, sprite::Sprite};
use glam::Vec2;
use log::info;
use miniquad::*;

/// The game. :)
#[repr(C)]
pub struct Game {
    pub sprite: Sprite,
    pub editor: Editor,
}

/// Core game loop.
impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) {
        self.sprite.update(ctx);
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.begin_default_pass(Default::default());

        self.editor.draw(ctx);
        self.sprite.draw(ctx);

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
        self.editor.key_down_event(ctx, keycode, _keymods, _repeat);
        self.sprite.key_down_event(ctx, keycode, _keymods, _repeat);
        match keycode {
            KeyCode::Escape => ctx.order_quit(),
            _ => (),
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        self.editor.key_up_event(ctx, keycode, _keymods);
        self.sprite.key_up_event(ctx, keycode, _keymods)
    }

    fn char_event(&mut self, ctx: &mut Context, character: char, _keymods: KeyMods, _repeat: bool) {
        self.editor.char_event(ctx, character, _keymods, _repeat);
        match character {
            'z' => ctx.set_fullscreen(true),
            'x' => ctx.set_fullscreen(false),
            _ => (),
        }
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        self.editor.mouse_motion_event(ctx, x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        self.editor.mouse_wheel_event(ctx, x, y);
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.editor.mouse_button_down_event(ctx, button, x, y);
    }

    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        self.editor.mouse_button_up_event(ctx, button, x, y);
    }
}

impl Game {
    /// Constructs the game with all the necessary elements into it.
    pub fn new(ctx: &mut Context) -> Self {
        info!("Creating the Game instance");
        let (window_w, window_h) = ctx.screen_size();
        let position = Vec2::new(window_w / 2.0, window_h / 2.0);
        let image_path = Path::new("src/add.png");
        let sprite = Sprite::new(ctx, position, image_path);
        let editor = Editor::new(ctx);

        Self { sprite, editor }
    }
}
