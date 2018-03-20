use super::{InterfaceItem, Filter};

use std::iter::repeat;

use glutin::VirtualKeyCode;
use text_buffer::TextBuffer;
use input::Input;

/// Represents a text-input field, that can be focused, takes in input (text),
/// and it's possible to get the input with get_text
#[derive(Debug, Clone)]
pub struct TextInput {
    x: u32,
    y: u32,
    width: u32,
    text: String,
    prefix: String,
    suffix: String,
    filter: Filter,
    focused: bool,
    dirty: bool,
}

impl TextInput {
    /// Initializes a new TextInput with the given position and width
    pub fn new(width: u32) -> TextInput {
        TextInput {
            x: 0,
            y: 0,
            width: width,
            text: String::new(),
            prefix: String::new(),
            suffix: String::new(),
            filter: Filter::empty_filter(),
            focused: false,
            dirty: true,
        }
    }

    /// Sets the position of the TextInput
    pub fn with_pos(mut self, position: (u32, u32)) -> TextInput {
        let (x, y) = position;
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the width of the TextInput.
    pub fn with_width(mut self, width: u32) -> TextInput {
        self.width = width;
        self
    }

    /// Sets the text of the TextInput.
    pub fn with_text<T: Into<String>>(mut self, text: T) -> TextInput {
        self.text = text.into();
        self
    }

    /// Sets the prefix text of the TextInput.
    pub fn with_prefix<T: Into<String>>(mut self, prefix: T) -> TextInput {
        self.prefix = prefix.into();
        self
    }

    /// Sets the suffix text of the TextInput.
    pub fn with_suffix<T: Into<String>>(mut self, suffix: T) -> TextInput {
        self.suffix = suffix.into();
        self
    }

    /// Sets weather the TextInput is focused.
    pub fn with_focus(mut self, focused: bool) -> TextInput {
        self.focused = focused;
        self
    }

    /// Sets the filter for the TextInput.
    pub fn with_filter(mut self, filter: Filter) -> TextInput {
        self.filter = filter;
        self
    }

    /// Sets the filter that will be used when taking inputs to write into the TextInput
    pub fn set_filter(&mut self, filter: Filter) {
        self.filter = filter;
    }

    /// Gets the filter
    pub fn get_filter(&self) -> &Filter {
        &self.filter
    }

    /// Gets a mutable reference of the filter.
    pub fn get_filter_mut(&mut self) -> &mut Filter {
        &mut self.filter
    }

    /// Returns the current text in the input
    pub fn get_text(&self) -> String {
        self.text.clone()
    }
}

impl InterfaceItem for TextInput {
    fn get_pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, pos: (u32, u32)) {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
    }

    fn get_total_width(&self) -> u32 {
        (self.prefix.len() + self.suffix.len()) as u32 + self.width
    }

    fn get_total_height(&self) -> u32 {
        1
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn draw(&mut self, text_buffer: &mut TextBuffer) {
        self.dirty = false;

        if self.focused {
            text_buffer.change_cursor_bg_color([0.8, 0.8, 0.8, 1.0]);
            text_buffer.change_cursor_fg_color([0.2, 0.2, 0.2, 1.0]);
        } else {
            text_buffer.change_cursor_bg_color([0.0, 0.0, 0.0, 0.0]);
            text_buffer.change_cursor_fg_color([0.8, 0.8, 0.8, 1.0]);
        }
        text_buffer.move_cursor(self.x as i32, self.y as i32);
        let text_width = (self.width as usize).min(self.text.len());
        let text: String = self.text[(self.text.len() - text_width)..].to_string();
        let spaces: String = repeat(" ")
            .take(self.width as usize - text_width)
            .collect();
        let text = text + &*spaces;
        text_buffer.write(format!("{}{}{}", self.prefix, text, self.suffix));
    }

    fn handle_input(&mut self, input: &Input) -> bool {
        let mut handled = false;
        if self.focused {
            if input.was_just_pressed(VirtualKeyCode::Back) {
                self.text.pop();
                self.dirty = true;
                handled = true;
            }
            for keycode in input.get_just_pressed_list() {
                if let Some(mut character) = self.filter.get(&keycode) {
                    let mut text = String::new();
                    if input.is_pressed(VirtualKeyCode::LShift)
                        || input.is_pressed(VirtualKeyCode::RShift)
                        {
                            text.push_str(&*character.to_uppercase().to_string());
                        } else {
                        text.push(*character);
                    }
                    self.text.push_str(&*text);
                    self.dirty = true;
                    handled = true;
                }
            }
        }
        handled
    }
}