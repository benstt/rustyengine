use egui::*;
use egui_miniquad::EguiMq;
use miniquad::*;

pub struct Editor {
    egui_mq: EguiMq,
}

impl Editor {
    pub fn new(ctx: &mut miniquad::Context) -> Self {
        Self {
            egui_mq: EguiMq::new(ctx),
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
        // self.new_window(ctx, "Egui Window", |ui| {
        //     ui.heading("text");
        //     ui.checkbox(&mut true, "Checkbox");
        // });

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
}
