use crate::display::TextBufferDisplayData;
use crate::TextBuffer;
use glutin::{MouseButton, VirtualKeyCode};
use std::collections::HashMap;

/// Represents all the events that happen in glerminal, such as keyboard events, mouse events, resize, and close events.
///
/// ### Example usage:
/// ```no_run
/// use glerminal::{TerminalBuilder, VirtualKeyCode, TextBuffer};
///
/// let terminal = TerminalBuilder::new()
///     .with_title("Hello GLerminal!")
///     .with_dimensions((1280, 720))
///     .build();
///
/// let mut text_buffer;
/// match TextBuffer::create(&terminal, (80, 24)) {
///     Ok(buffer) => text_buffer = buffer,
///     Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
/// }
///
/// let events = terminal.get_current_events();
/// println!("Was A just pressed: {}", events.keyboard.was_just_pressed(VirtualKeyCode::A));
/// println!("Cursor position: {:?}", events.cursor.get_location(&text_buffer));
/// ```
#[derive(Clone)]
pub struct Events {
    /// Represents keyboard events.
    pub keyboard: Input<VirtualKeyCode>,
    /// Represents mouse events.
    pub mouse: Input<MouseButton>,
    /// Allows getting information related to cursor position
    pub cursor: Cursor,
}

impl Events {
    pub(crate) fn new(text_buffer_aspect_ratio: bool) -> Events {
        Events {
            keyboard: Input::new(),
            mouse: Input::new(),
            cursor: Cursor::new(text_buffer_aspect_ratio),
        }
    }

    pub(crate) fn clear_just_lists(&mut self) {
        self.keyboard.clear_just_lists();
        self.mouse.clear_just_lists();
        self.cursor.clear_just_moved();
    }
}

/// Cursor has the ability to get the position in the text buffer where the cursor currently is.
///
/// Just call `events.cursor.get_location(&text_buffer);`
/// See Events-documentation for usage information.
#[derive(Clone)]
pub struct Cursor {
    location: Option<(f32, f32)>,
    just_moved: bool,
    use_text_buffer_overflows: bool,
    display_overflows: (f32, f32),
    display_relative_dimensions: (f32, f32),
    text_buffer_datas: HashMap<u32, TextBufferDisplayData>,
}

impl Cursor {
    pub(crate) fn new(use_text_buffer_overflows: bool) -> Cursor {
        Cursor {
            location: None,
            just_moved: false,
            use_text_buffer_overflows: use_text_buffer_overflows,
            display_overflows: (0.0, 0.0),
            display_relative_dimensions: (0.0, 0.0),
            text_buffer_datas: HashMap::new(),
        }
    }

    pub(crate) fn update_display_datas(
        &mut self,
        display_overflows: (f32, f32),
        display_relative_dimensions: (f32, f32),
        datas: HashMap<u32, TextBufferDisplayData>,
    ) {
        self.display_overflows = display_overflows;
        self.display_relative_dimensions = display_relative_dimensions;
        self.text_buffer_datas = datas;
    }

    pub(crate) fn update_location(&mut self, location: (f32, f32)) {
        self.just_moved = true;

        self.location = Some((location.0, location.1));
    }

    pub(crate) fn cursor_left(&mut self) {
        self.just_moved = true;
        self.location = None;
    }

    pub(crate) fn clear_just_moved(&mut self) {
        self.just_moved = false;
    }

    /// Weather the cursor has moved within the last frame
    pub fn cursor_just_moved(&self) -> bool {
        self.just_moved
    }

    /// Returns the current position of the cursor (the coordinates on the text buffer).
    pub fn get_location(&self, text_buffer: &TextBuffer) -> Option<(i32, i32)> {
        if let Some(location) = self.location {
            let mut overflows = self.display_overflows;
            let mut relative_dimensions = self.display_relative_dimensions;
            if self.use_text_buffer_overflows {
                if let Some(data) = self.text_buffer_datas.get(&text_buffer.get_idx()) {
                    overflows = data.overflows;
                    relative_dimensions = data.relative_dimensions;
                }
            }

            if location.0 > overflows.0
                && location.0 < 1.0 - overflows.0
                && location.1 > overflows.1
                && location.1 < 1.0 - overflows.1
            {
                let x = (location.0 - overflows.0) * relative_dimensions.0;
                let y = (location.1 - overflows.1) * relative_dimensions.1;

                Some((
                    (x * text_buffer.width as f32).floor() as i32,
                    (y * text_buffer.height as f32).floor() as i32,
                ))
            } else {
                None
            }
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
