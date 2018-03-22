use super::{InterfaceItem, Filter};

use std::iter::repeat;

use glutin::VirtualKeyCode;
use text_buffer::TextBuffer;
use input::Input;
use text_buffer::Color;

/// Represents a text-input field, that can be focused, takes in input (text),
/// and it's possible to get the input with get_text
#[derive(Debug, Clone)]
pub struct TextInput {
    /// Background-color for when the field is unfocused
    pub unfocused_bg: Color,
    /// Background-color for when the field is focused
    pub focused_bg: Color,
    /// Foreground-color for when the field is unfocused
    pub unfocused_fg: Color,
    /// Foreground-color for when the field is focused
    pub focused_fg: Color,
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
            unfocused_bg: [0.0, 0.0, 0.0, 0.0],
            unfocused_fg: [0.8, 0.8, 0.8, 1.0],
            focused_bg: [0.8, 0.8, 0.8, 1.0],
            focused_fg: [0.2, 0.2, 0.2, 1.0],
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

    /// Sets whether the TextInput is focused.
    pub fn with_focus(mut self, focused: bool) -> TextInput {
        self.focused = focused;
        self
    }

    /// Sets the filter for the TextInput.
    pub fn with_filter(mut self, filter: Filter) -> TextInput {
        self.filter = filter;
        self
    }

    /// Sets the (fg, bg) colors for when the field is unfocused
    pub fn with_unfocused_colors(mut self, colors: (Color, Color)) -> TextInput {
        let (fg, bg) = colors;
        self.unfocused_fg = fg;
        self.unfocused_bg = bg;
        self
    }

    /// Sets the (fg, bg) colors for when the field is focused
    pub fn with_focused_colors(mut self, colors: (Color, Color)) -> TextInput {
        let (fg, bg) = colors;
        self.focused_fg = fg;
        self.focused_bg = bg;
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
        if focused != self.focused {
            self.dirty = true;
        }
        self.focused = focused;
    }

    fn can_be_focused(&self) -> bool {
        true
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn draw(&mut self, text_buffer: &mut TextBuffer) {
        self.dirty = false;

        if self.focused {
            text_buffer.change_cursor_bg_color(self.focused_bg);
            text_buffer.change_cursor_fg_color(self.focused_fg);
        } else {
            text_buffer.change_cursor_bg_color(self.unfocused_bg);
            text_buffer.change_cursor_fg_color(self.unfocused_fg);
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