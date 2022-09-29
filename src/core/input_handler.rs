use gilrs::{Button, ConnectedGamepadsIterator, Event, Gamepad, GamepadId, Gilrs};
use miniquad::EventHandler;

pub struct InputHandler {
    gilrs: Gilrs,
    current_gamepad_id: Option<GamepadId>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            gilrs: Gilrs::new().unwrap(),
            current_gamepad_id: None,
        }
    }

    pub fn gamepads(&self) -> ConnectedGamepadsIterator {
        self.gilrs.gamepads()
    }

    pub fn current_gamepad(&self) -> Option<Gamepad> {
        if let Some(current_gamepad_id) = self.current_gamepad_id {
            let gamepad = self.gilrs.gamepad(current_gamepad_id);
            return Some(gamepad);
        }

        None
    }

    fn check_for_gamepad(&mut self) {
        if let Some(event) = self.gilrs.next_event() {
            if self.current_gamepad_id == Some(event.id) {
                return;
            }

            debug!("detected input with ID {}", event.id);
            self.current_gamepad_id = Some(event.id);
        }
    }
}

impl EventHandler for InputHandler {
    fn update(&mut self, _ctx: &mut miniquad::Context) {
        self.check_for_gamepad();
    }

    fn draw(&mut self, _ctx: &mut miniquad::Context) {}
}
