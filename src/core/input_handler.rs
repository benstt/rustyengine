use gilrs::{Axis, ConnectedGamepadsIterator, Gamepad, GamepadId, Gilrs};
use miniquad::EventHandler;

pub enum Stick {
    Left,
    Right,
}

/// Handles the input.
/// Keeps track of the current gamepads connected as well as provides different input functionality.
pub struct InputHandler {
    gilrs: Gilrs,
    current_gamepad_id: Option<GamepadId>,
}

impl InputHandler {
    pub fn new() -> Self {
        let gilrs = Gilrs::new().unwrap();

        Self {
            gilrs,
            current_gamepad_id: None,
        }
    }

    /// Gets all the connected gamepads.
    pub fn gamepads(&self) -> ConnectedGamepadsIterator {
        self.gilrs.gamepads()
    }

    /// Gets the current used gamepad.
    pub fn current_gamepad(&self) -> Option<Gamepad> {
        if let Some(current_gamepad_id) = self.current_gamepad_id {
            let gamepad = self.gilrs.gamepad(current_gamepad_id);
            return Some(gamepad);
        }

        None
    }

    /// Gets the given stick's axis values, ranging from -1.0 to 1.0.
    pub fn axis_values(&self, stick: Stick) -> (f32, f32) {
        let (mut value_x, mut value_y) = (0.0, 0.0);

        if let Some(current_gamepad) = self.current_gamepad() {
            let (axis_x, axis_y) = match stick {
                Stick::Left => (Axis::LeftStickX, Axis::LeftStickY),
                Stick::Right => (Axis::RightStickX, Axis::RightStickY),
            };

            if let Some(axis_data_x) = current_gamepad.axis_data(axis_x) {
                value_x = axis_data_x.value();
            }

            if let Some(axis_data_y) = current_gamepad.axis_data(axis_y) {
                value_y = axis_data_y.value();
            }
        }

        (value_x, value_y)
    }

    /// Checks if a gamepad is found. If it's a new one, mark it as the currently used.
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
