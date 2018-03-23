use super::InterfaceItem;
use input::Input;
use text_buffer::{Color, TextBuffer};
use VirtualKeyCode;

#[derive(Debug, Clone)]
/// Represents a button as an InterfaceItem
pub struct Button {
    /// Foreground color for when the button is not focused
    pub fg_color_unfocused: Color,
    /// Background color for when the button is not focused
    pub bg_color_unfocused: Color,
    /// Foreground color for when the button is focused
    pub fg_color_focused: Color,
    /// Background color for when the button is focused
    pub bg_color_focused: Color,
    x: u32,
    y: u32,
    max_width: u32,
    text: String,
    focused: bool,
    dirty: bool,

    was_just_pressed: bool,
    accepted_inputs: Vec<VirtualKeyCode>,
}

impl Button {
    /// Intiailizes a Button with the given text and max width
    pub fn new<T: Into<String>>(text: T, max_width: u32) -> Button {
        Button {
            bg_color_unfocused: [0.0, 0.0, 0.0, 0.0],
            fg_color_unfocused: [0.8, 0.8, 0.8, 1.0],
            bg_color_focused: [0.8, 0.8, 0.8, 1.0],
            fg_color_focused: [0.2, 0.2, 0.2, 1.0],
            x: 0,
            y: 0,
            max_width: max_width,
            text: text.into(),
            focused: false,
            dirty: true,

            was_just_pressed: false,
            accepted_inputs: vec![VirtualKeyCode::Return],
        }
    }

    /// Sets the initial position of the Button
    pub fn with_pos(mut self, pos: (u32, u32)) -> Button {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the initial max width of the Button
    pub fn with_max_width(mut self, max_width: u32) -> Button {
        self.max_width = max_width;
        self
    }

    /// Sets the initial text of the Button
    pub fn with_text<T: Into<String>>(mut self, text: T) -> Button {
        self.text = text.into();
        self
    }

    /// Set whether the button is initially focused or not
    pub fn with_focused(mut self, focused: bool) -> Button {
        self.focused = focused;
        self
    }

    /// Set the initial colors of this Button when it is unfocused
    pub fn with_unfocused_colors(mut self, colors: (Color, Color)) -> Button {
        let (fg, bg) = colors;
        self.fg_color_unfocused = fg;
        self.bg_color_unfocused = bg;
        self
    }

    /// Set the initial colors of this Button when it is focused
    pub fn with_focused_colors(mut self, colors: (Color, Color)) -> Button {
        let (fg, bg) = colors;
        self.fg_color_focused = fg;
        self.bg_color_focused = bg;
        self
    }

    /// Set the buttons from which this button triggers
    pub fn with_accepted_inputs(mut self, buttons: Vec<VirtualKeyCode>) -> Button {
        self.accepted_inputs = buttons;
        self
    }

    /// Sets the max width of the Button
    pub fn set_max_width(&mut self, max_width: u32) {
        self.max_width = max_width;
    }

    /// Sets the text of the Button
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
        self.dirty = true;
    }

    /// Return the current text of the Button
    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    /// Set the colors of this Button when it is unfocused
    pub fn set_unfocused_colors(&mut self, colors: (Color, Color)) {
        let (fg, bg) = colors;
        self.fg_color_unfocused = fg;
        self.bg_color_unfocused = bg;
    }

    /// Set the colors of this Button when it is focused
    pub fn set_focused_colors(&mut self, colors: (Color, Color)) {
        let (fg, bg) = colors;
        self.fg_color_focused = fg;
        self.bg_color_focused = bg;
    }

    /// Set the buttons from which this button triggers
    pub fn set_accepted_inputs(mut self, buttons: Vec<VirtualKeyCode>) {
        self.accepted_inputs = buttons;
    }

    /// Returns whether this button was just pressed.
    pub fn was_just_pressed(&self) -> bool {
        self.was_just_pressed
    }
}

impl InterfaceItem for Button {
    fn get_pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, pos: (u32, u32)) {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
    }

    fn get_total_width(&self) -> u32 {
        self.max_width
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
        text_buffer.write(
            self.text
                .chars()
                .take(self.max_width as usize)
                .collect::<String>(),
        );
    }

    fn handle_input(&mut self, input: &Input) -> bool {
        self.was_just_pressed = false;
        for curr in &self.accepted_inputs {
            if input.was_just_pressed(*curr) {
                self.was_just_pressed = true;
                return true;
            }
        }
        false
    }

    fn update(&mut self, _: f32) {}
}
