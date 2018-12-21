use glutin::{MouseButton, VirtualKeyCode};
use TextBuffer;

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
    /// Allows getting information related to cursor position
    pub cursor_position: CursorPosition,
}

impl Events {
    pub(crate) fn new() -> Events {
        Events {
            keyboard: Input::new(),
            mouse: Input::new(),
            cursor_position: CursorPosition::new(),
        }
    }

    pub(crate) fn clear_just_lists(&mut self) {
        self.keyboard.clear_just_lists();
        self.mouse.clear_just_lists();
        self.cursor_position.clear_just_moved();
    }
}

#[derive(Clone)]
pub struct CursorPosition {
    location: Option<(f32, f32)>,
    just_moved: bool,
    overflows: (f32, f32),
    relative_dimensions: (f32, f32),
}

impl CursorPosition {
    pub(crate) fn new() -> CursorPosition {
        CursorPosition {
            location: None,
            just_moved: false,
            overflows: (0.0, 0.0),
            relative_dimensions: (0.0, 0.0),
        }
    }

    pub(crate) fn update_overflows(&mut self, dimensions: (f32, f32), aspect_ratio: f32) {
        let (width, height) = dimensions;
        let true_width = height * aspect_ratio;
        let true_height = width / aspect_ratio;

        let mut overflow_width = 0f32;
        let mut overflow_height = 0f32;
        let mut relative_width = 1.0;
        let mut relative_height = 1.0;
        if true_width < width {
            overflow_width = (width - true_width) / width;
            relative_width = width / true_width;
        } else {
            overflow_height = (height - true_height) / height;
            relative_height = height / true_height;
        }

        self.overflows = (overflow_width / 2.0, overflow_height / 2.0);
        self.relative_dimensions = (relative_width, relative_height);
    }

    pub(crate) fn update_location(&mut self, location: (f32, f32)) {
        self.just_moved = true;

        if location.0 > self.overflows.0
            && location.0 < 1.0 - self.overflows.0
            && location.1 > self.overflows.1
            && location.1 < 1.0 - self.overflows.1
        {
            let x = location.0 * self.relative_dimensions.0 - self.overflows.0;
            let y = location.1 * self.relative_dimensions.1 - self.overflows.1;

            self.location = Some((x, y));
        } else {
            self.location = None;
        }
    }

    pub(crate) fn cursor_left(&mut self) {
        self.just_moved = true;
        self.location = None;
    }

    pub(crate) fn clear_just_moved(&mut self) {
        self.just_moved = false;
    }

    pub fn cursor_just_moved(&self) -> bool {
        self.just_moved
    }

    pub fn get_location(&self, text_buffer: &TextBuffer) -> Option<(i32, i32)> {
        if let Some(location) = self.location {
            Some((
                (location.0 * text_buffer.width as f32).floor() as i32,
                (location.1 * text_buffer.height as f32).floor() as i32,
            ))
        } else {
            None
        }
    }
}

/// Input contains the necessary infoamtions to satisfy all your binary input-gathering needs!
///
/// Input is used for keyboard input (VirtualKeyCode) and mouse input (MouseButton).
/// See Events-documentation for usage information.
#[derive(Clone)]
pub struct Input<T: PartialEq + Copy> {
    pressed: Vec<T>,
    just_released: Vec<T>,
    just_pressed: Vec<T>,
}

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
