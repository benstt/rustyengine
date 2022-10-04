use std::path::Path;

use egui::{panel::Side, *};
use egui_miniquad::EguiMq;
use miniquad::*;

use super::sprite::Sprite;

pub struct Editor {
    pub background_image: Sprite,
    egui_mq: EguiMq,
}

impl Editor {
    pub fn new(ctx: &mut miniquad::Context) -> Self {
        let image_pos = glam::Vec2::new(0.0, 0.0);
        let image_path = Path::new("src/content/editor-background.png");
        let texture_params = TextureParams {
            filter: FilterMode::Nearest,
            wrap: TextureWrap::Repeat,
            ..Default::default()
        };
        let (screen_size_x, screen_size_y) = ctx.screen_size();

        let mut background_image = Sprite::with_params(ctx, image_pos, image_path, texture_params);
        background_image.scale_to(screen_size_x, screen_size_y);

        Self {
            egui_mq: EguiMq::new(ctx),
            background_image,
        }
    }

    pub fn new_window<F>(&mut self, ctx: &mut miniquad::Context, title: &str, ui: F)
    where
        F: FnOnce(&mut Ui),
    {
        self.egui_mq.run(ctx, |_ctx, egui_ctx| {
            egui::Window::new(title).show(egui_ctx, ui);
        })
    }
}

impl EventHandler for Editor {
    fn update(&mut self, _ctx: &mut miniquad::Context) {}

    fn draw(&mut self, ctx: &mut miniquad::Context) {
        let mut ordered_quit = false;

        self.egui_mq.run(ctx, |ctx, egui_ctx| {
            TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit editor").clicked() {
                        ordered_quit = true;
                    }
                });
            });

            SidePanel::left("left_panel").show(egui_ctx, |ui| {
                ui.heading("Rusty Engine");
            });
        });

        if ordered_quit {
            ctx.order_quit();
        }

        self.egui_mq.draw(ctx);
    }

    fn mouse_motion_event(&mut self, _: &mut miniquad::Context, x: f32, y: f32) {
        self.egui_mq.mouse_motion_event(x, y);
    }

    fn mouse_wheel_event(&mut self, _: &mut miniquad::Context, dx: f32, dy: f32) {
        self.egui_mq.mouse_wheel_event(dx, dy);
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut miniquad::Context,
        mb: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_down_event(ctx, mb, x, y);
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut miniquad::Context,
        mb: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.egui_mq.mouse_button_up_event(ctx, mb, x, y);
    }

    fn char_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        character: char,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.char_event(character);
    }

    fn key_down_event(
        &mut self,
        ctx: &mut miniquad::Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        self.egui_mq.key_down_event(ctx, keycode, keymods);
    }

    fn key_up_event(&mut self, _ctx: &mut miniquad::Context, keycode: KeyCode, keymods: KeyMods) {
        self.egui_mq.key_up_event(keycode, keymods);
    }

    fn resize_event(&mut self, _ctx: &mut GraphicsContext, width: f32, height: f32) {
        self.background_image.scale_to(width, height);
    }
}
