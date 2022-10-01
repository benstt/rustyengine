use gilrs::{Axis, ConnectedGamepadsIterator, Gamepad, GamepadId, Gilrs};
use miniquad::{EventHandler, KeyCode, MouseButton};
use std::collections::HashSet;

pub enum Stick {
    Left,
    Right,
}

/// Handles the input.
/// Keeps track of the current gamepads connected as well as provides different input functionality.
pub struct InputHandler {
    gilrs: Gilrs,
    /// The current gamepad ID. Used to get the actual gamepad when needed.
    current_gamepad_id: Option<GamepadId>,
    /// All the unique keys pressed at a given time.
    keys_pressed: HashSet<KeyCode>,
    /// Similar to `keys_pressed`, but for mouse buttons.
    mouse_buttons_pressed: HashSet<MouseButton>,
    /// Stores the last key that was pressed after an event.
    last_key_down: Option<KeyCode>,
    /// Stores the last mouse position recorded after an event.
    last_mouse_position: (usize, usize),
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            ..Default::default()
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

    /// Gets all pressed keys at a given moment.
    pub const fn pressed_keys(&self) -> &HashSet<KeyCode> {
        &self.keys_pressed
    }

    /// Check whether the given key is in the last pressed ones.
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    /// Helper to apply _little_ code after a key is released.
    /// Emphasis on _little_, as this does not allow passing function calls into `func`.
    pub fn on_key_release<F>(&mut self, key: KeyCode, mut func: F)
    where
        F: FnMut(),
    {
        if self.last_key_down == Some(key) {
            func();
        }

        self.last_key_down = None;
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

impl Default for InputHandler {
    fn default() -> Self {
        let gilrs = Gilrs::new().unwrap();
        let current_gamepad_id = None;
        let keys_pressed = HashSet::new();
        let mouse_buttons_pressed = HashSet::new();
        let last_key_down = None;
        let last_mouse_position = (0, 0);

        Self {
            gilrs,
            current_gamepad_id,
            keys_pressed,
            mouse_buttons_pressed,
            last_key_down,
            last_mouse_position,
        }
    }
}

impl EventHandler for InputHandler {
    fn update(&mut self, _ctx: &mut miniquad::Context) {
        self.check_for_gamepad();
    }

    fn draw(&mut self, _ctx: &mut miniquad::Context) {}

    fn key_down_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        keycode: miniquad::KeyCode,
        _keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        self.keys_pressed.insert(keycode);
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        keycode: KeyCode,
        _keymods: miniquad::KeyMods,
    ) {
        // FIXME: when another event is called before letting the key up, this won't be called
        // e.g. setting the window to fullscreen while pressing W.
        self.keys_pressed.remove(&keycode);
        self.last_key_down = Some(keycode);
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        self.mouse_buttons_pressed.insert(button);
        self.last_mouse_position = (x as usize, y as usize);
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut miniquad::Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.mouse_buttons_pressed.remove(&button);
    }
}
