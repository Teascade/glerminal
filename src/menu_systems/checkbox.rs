use std::iter::repeat;

use super::InterfaceItem;
use input::Input;
use text_buffer::{Color, TextBuffer};
use VirtualKeyCode;

/// Represents a Checkbox that can be checked or unchecked, and it's checkedness can be get.
#[derive(Debug, Clone)]
pub struct Checkbox {
    /// Foreground color for when the checkbox is not focused
    pub fg_color_unfocused: Color,
    /// Background color for when the checkbox is not focused
    pub bg_color_unfocused: Color,
    /// Foreground color for when the checkbox is focused
    pub fg_color_focused: Color,
    /// Background color for when the checkbox is focused
    pub bg_color_focused: Color,
    x: u32,
    y: u32,

    text: String,
    prefix: String,
    suffix: String,
    checked_text: String,

    focused: bool,
    dirty: bool,

    checked: bool,
    was_just_pressed: bool,
    button_press_inputs: Vec<VirtualKeyCode>,
}

impl Checkbox {
    /// Intiailizes a Checkbox with the given text and max width
    pub fn new<T: Into<String>>(text: T) -> Checkbox {
        Checkbox {
            bg_color_unfocused: [0.0, 0.0, 0.0, 0.0],
            fg_color_unfocused: [0.8, 0.8, 0.8, 1.0],
            bg_color_focused: [0.8, 0.8, 0.8, 1.0],
            fg_color_focused: [0.2, 0.2, 0.2, 1.0],
            x: 0,
            y: 0,

            text: text.into(),
            prefix: "[".to_owned(),
            suffix: "]".to_owned(),
            checked_text: "X".to_owned(),

            focused: false,
            dirty: true,

            checked: false,
            was_just_pressed: false,
            button_press_inputs: vec![VirtualKeyCode::Return],
        }
    }

    /// Sets the initial position of the Checkbox
    pub fn with_pos(mut self, pos: (u32, u32)) -> Checkbox {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the initial text of the Checkbox
    pub fn with_text<T: Into<String>>(mut self, text: T) -> Checkbox {
        self.text = text.into();
        self
    }

    /// Sets the initial prefix of the Checkbox
    pub fn with_prefix<T: Into<String>>(mut self, prefix: T) -> Checkbox {
        self.prefix = prefix.into();
        self
    }

    /// Sets the initial suffix of the Checkbox
    pub fn with_suffix<T: Into<String>>(mut self, suffix: T) -> Checkbox {
        self.suffix = suffix.into();
        self
    }

    /// Sets the initial checked-text (text shown in between prefix and suffix) of the Checkbox
    pub fn with_checked_text<T: Into<String>>(mut self, checked_text: T) -> Checkbox {
        self.checked_text = checked_text.into();
        self
    }

    /// Set whether the checkbox is initially focused or not
    pub fn with_focused(mut self, focused: bool) -> Checkbox {
        self.focused = focused;
        self
    }

    /// Set whether the checkbox is initially checked or not
    pub fn with_checked(mut self, checked: bool) -> Checkbox {
        self.checked = checked;
        self
    }

    /// Set the initial colors of this Checkbox when it is unfocused
    pub fn with_unfocused_colors(mut self, colors: (Color, Color)) -> Checkbox {
        let (fg, bg) = colors;
        self.fg_color_unfocused = fg;
        self.bg_color_unfocused = bg;
        self
    }

    /// Set the initial colors of this Checkbox when it is focused
    pub fn with_focused_colors(mut self, colors: (Color, Color)) -> Checkbox {
        let (fg, bg) = colors;
        self.fg_color_focused = fg;
        self.bg_color_focused = bg;
        self
    }

    /// Set the buttons from which this checkbox's was_just_pressed triggers
    pub fn with_button_press_inputs(mut self, buttons: Vec<VirtualKeyCode>) -> Checkbox {
        self.button_press_inputs = buttons;
        self
    }

    /// Sets the text of the Checkbox
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
        self.dirty = true;
    }

    /// Sets the prefix of the Checkbox
    pub fn set_prefix<T: Into<String>>(&mut self, prefix: T) {
        self.prefix = prefix.into();
        self.dirty = true;
    }

    /// Sets the suffix of the Checkbox
    pub fn set_suffix<T: Into<String>>(&mut self, suffix: T) {
        self.suffix = suffix.into();
        self.dirty = true;
    }

    /// Sets the checked-text (text shown in between prefix and suffix) of the Checkbox
    pub fn set_checked_text<T: Into<String>>(mut self, checked_text: T) {
        self.checked_text = checked_text.into();
    }

    /// Return the current text of the Checkbox
    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    /// Return the current prefix of the Checkbox
    pub fn get_prefix(&self) -> String {
        self.prefix.clone()
    }

    /// Return the current suffix of the Checkbox
    pub fn get_suffix(&self) -> String {
        self.suffix.clone()
    }

    /// Set the colors of this Checkbox when it is unfocused
    pub fn set_unfocused_colors(&mut self, colors: (Color, Color)) {
        let (fg, bg) = colors;
        self.fg_color_unfocused = fg;
        self.bg_color_unfocused = bg;
    }

    /// Set the colors of this Checkbox when it is focused
    pub fn set_focused_colors(&mut self, colors: (Color, Color)) {
        let (fg, bg) = colors;
        self.fg_color_focused = fg;
        self.bg_color_focused = bg;
    }

    /// Set the buttons from which this checkbox's was_just_pressed triggers
    pub fn set_button_press_inputs(mut self, buttons: Vec<VirtualKeyCode>) {
        self.button_press_inputs = buttons;
    }

    /// Returns whether this checkbox was just pressed.
    pub fn was_just_pressed(&self) -> bool {
        self.was_just_pressed
    }

    /// Sets the checked-status for this checkbox.
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    /// Returns whether this checkbox is checked.
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

impl InterfaceItem for Checkbox {
    fn get_pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, pos: (u32, u32)) {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
    }

    fn get_total_width(&self) -> u32 {
        (self.text.len() + self.prefix.len() + self.checked_text.len() + self.suffix.len()) as u32
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
            text_buffer.change_cursor_fg_color(self.fg_color_focused);
            text_buffer.change_cursor_bg_color(self.bg_color_focused);
        } else {
            text_buffer.change_cursor_fg_color(self.fg_color_unfocused);
            text_buffer.change_cursor_bg_color(self.bg_color_unfocused);
        }
        text_buffer.move_cursor(self.x as i32, self.y as i32);
        let checked_text;
        if self.checked {
            checked_text = (&self.checked_text).to_owned();
        } else {
            checked_text = repeat(" ").take(self.checked_text.len()).collect();
        }
        let text = (&self.text).to_owned() + &self.prefix + &checked_text + &self.suffix;
        text_buffer.write(text);
    }

    fn handle_input(&mut self, input: &Input) -> bool {
        self.was_just_pressed = false;
        for curr in &self.button_press_inputs {
            if input.was_just_pressed(*curr) {
                self.was_just_pressed = true;
                self.checked = !self.checked;
                self.dirty = true;
                return true;
            }
        }
        false
    }

    fn update(&mut self, _: f32) {}
}
