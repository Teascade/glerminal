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

    pub(crate) fn update_button_press(&mut self, button: T, pressed: bool) {
        if pressed && !self.pressed.contains(&button) {
            self.pressed.push(button);
            self.just_pressed.push(button);
        } else if !pressed && self.pressed.contains(&button) {
            self.just_released.push(button);
            if let Some(idx) = self.find_buttonpress_idx_from_pressed(button) {
                self.pressed.remove(idx);
            }
        }
    }

    /// Returns wether the button is currently pressed. Does not care when it was pressed.
    pub fn is_pressed(&self, button: T) -> bool {
        self.pressed.contains(&button)
    }

    /// Returns wether the button was pressed this frame.
    pub fn was_just_pressed(&self, button: T) -> bool {
        self.just_pressed.contains(&button)
    }

    /// Returns wether the button was released this frame.
    pub fn was_just_released(&self, button: T) -> bool {
        self.just_released.contains(&button)
    }

    /// Returns an entire list of buttons that were just pressed.
    pub fn get_just_pressed_list(&self) -> Vec<T> {
        self.just_pressed.clone()
    }

    /// Returns an entire list of buttons that were just released.
    pub fn get_just_released_list(&self) -> Vec<T> {
        self.just_released.clone()
    }

    /// Returns an entire list of buttons that are pressed right now.
    pub fn get_pressed_list(&self) -> Vec<T> {
        self.pressed.clone()
    }

    fn find_buttonpress_idx_from_pressed(&self, button: T) -> Option<usize> {
        let mut idx: usize = 0;
        let mut found = false;
        for i in 0..self.pressed.len() {
            if button == self.pressed[i] {
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
