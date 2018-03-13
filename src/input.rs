//! This module includes the Input that can be retrieved via Terminal

use glutin::VirtualKeyCode;

/// Input contains the necessary infoamtions to satisfy all your input-gathering needs!
///
/// It's usage is quite simple, but an Input must be retrieved via a Terminal. Example:
///
/// ```
/// use glerminal::terminal::TerminalBuilder;
/// use glerminal::VirtualKeyCode;
///
/// let terminal = TerminalBuilder::new()
///     .with_title("Hello GLerminal!")
///     .with_dimensions((1280, 720))
///     .build();
///
/// let current_input = terminal.get_current_input();
/// if current_input.just_pressed(VirtualKeyCode::E) {
///     println!("E was just pressed!");
/// }
/// ```
#[derive(Clone)]
pub struct Input {
    pressed: Vec<VirtualKeyCode>,
    just_released: Vec<VirtualKeyCode>,
    just_pressed: Vec<VirtualKeyCode>,
}

#[allow(dead_code)]
impl Input {
    pub(crate) fn new() -> Input {
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

    pub(crate) fn update_virtual_keycode(&mut self, keycode: VirtualKeyCode, pressed: bool) {
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
    pub fn is_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.pressed.contains(&keycode)
    }

    /// Returns wether the keycode was pressed this frame.
    pub fn was_just_pressed(&self, keycode: VirtualKeyCode) -> bool {
        self.just_pressed.contains(&keycode)
    }

    /// Returns wether the keycode was released this frame.
    pub fn was_just_released(&self, keycode: VirtualKeyCode) -> bool {
        self.just_released.contains(&keycode)
    }

    fn find_keycode_idx_from_pressed(&self, keycode: VirtualKeyCode) -> Option<usize> {
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
