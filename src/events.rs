use glutin::{MouseButton, VirtualKeyCode};

/// Represents all the events that happen in glerminal, such as keyboard events, mouse events, resize, and close events.
///
/// ### Example usage:
/// ```no_run
/// use glerminal::{TerminalBuilder, VirtualKeyCode};
///
/// let terminal = TerminalBuilder::new()
///     .with_title("Hello GLerminal!")
///     .with_dimensions((1280, 720))
///     .build();
///
/// let events = terminal.get_current_events();
/// println!("Was A just pressed: {}", events.keyboard.was_just_pressed(VirtualKeyCode::A));
/// ```
#[derive(Clone)]
pub struct Events {
    /// Represents keyboard events.
    pub keyboard: Input<VirtualKeyCode>,
    /// Represents mouse events.
    pub mouse: Input<MouseButton>,
}

impl Events {
    pub(crate) fn new() -> Events {
        Events {
            keyboard: Input::new(),
            mouse: Input::new(),
        }
    }

    pub(crate) fn clear_just_lists(&mut self) {
        self.keyboard.clear_just_lists();
        self.mouse.clear_just_lists();
    }
}

/// Input contains the necessary infoamtions to satisfy all your input-gathering needs!
///
/// Input is used for keyboard input (VirtualKeyCode) and mouse input (MouseButton).
/// See Events-documentation for usage information.
#[derive(Clone)]
pub struct Input<T: PartialEq + Copy> {
    pressed: Vec<T>,
    just_released: Vec<T>,
    just_pressed: Vec<T>,
}

#[allow(dead_code)]
impl<T: PartialEq + Copy> Input<T> {
    pub(crate) fn new() -> Input<T> {
        Input {
            pressed: Vec::new(),
            just_pressed: Vec::new(),
            just_released: Vec::new(),
        }
    }

    pub(crate) fn clear_just_lists(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub(crate) fn update_virtual_keycode(&mut self, keycode: T, pressed: bool) {
        if pressed && !self.pressed.contains(&keycode) {
            self.pressed.push(keycode);
            self.just_pressed.push(keycode);
        } else if !pressed && self.pressed.contains(&keycode) {
            self.just_released.push(keycode);
            if let Some(idx) = self.find_keycode_idx_from_pressed(keycode) {
                self.pressed.remove(idx);
            }
        }
    }

    /// Returns wether the keycode is currently pressed. Does not care when it was pressed.
    pub fn is_pressed(&self, keycode: T) -> bool {
        self.pressed.contains(&keycode)
    }

    /// Returns wether the keycode was pressed this frame.
    pub fn was_just_pressed(&self, keycode: T) -> bool {
        self.just_pressed.contains(&keycode)
    }

    /// Returns wether the keycode was released this frame.
    pub fn was_just_released(&self, keycode: T) -> bool {
        self.just_released.contains(&keycode)
    }

    /// Returns an entire list of keys that were just pressed.
    pub fn get_just_pressed_list(&self) -> Vec<T> {
        self.just_pressed.clone()
    }

    /// Returns an entire list of keys that were just released.
    pub fn get_just_released_list(&self) -> Vec<T> {
        self.just_released.clone()
    }

    /// Returns an entire list of keys that are pressed right now.
    pub fn get_pressed_list(&self) -> Vec<T> {
        self.pressed.clone()
    }

    fn find_keycode_idx_from_pressed(&self, keycode: T) -> Option<usize> {
        let mut idx: usize = 0;
        let mut found = false;
        for i in 0..self.pressed.len() {
            if keycode == self.pressed[i] {
                idx = i;
                found = true;
            }
        }
        if found {
            Some(idx)
        } else {
            None
        }
    }
}
