use super::{InterfaceItem, InterfaceItemBase};
use text_buffer::{Color, TextBuffer};
use Events;
use MouseButton;
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

    base: InterfaceItemBase,
    max_width: u32,
    text: String,

    was_just_pressed: bool,
    button_press_inputs: Vec<VirtualKeyCode>,
    mouse_button_press_inputs: Vec<MouseButton>,
}

impl Button {
    /// Intiailizes a Button with the given text and max width
    pub fn new<T: Into<String>>(text: T, max_width: u32) -> Button {
        Button {
            bg_color_unfocused: [0.0, 0.0, 0.0, 0.0],
            fg_color_unfocused: [0.8, 0.8, 0.8, 1.0],
            bg_color_focused: [0.8, 0.8, 0.8, 1.0],
            fg_color_focused: [0.2, 0.2, 0.2, 1.0],

            base: InterfaceItemBase::new(true),
            max_width: max_width,
            text: text.into(),

            was_just_pressed: false,
            button_press_inputs: vec![VirtualKeyCode::Return],
            mouse_button_press_inputs: Vec::new(),
        }
    }

    /// Sets the initial position of the Button
    pub fn with_pos(mut self, pos: (u32, u32)) -> Button {
        let (x, y) = pos;
        self.base.x = x;
        self.base.y = y;
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
        self.base.focused = focused;
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
    pub fn with_button_press_inputs(mut self, buttons: Vec<VirtualKeyCode>) -> Button {
        self.button_press_inputs = buttons;
        self
    }

    /// Set the mouse buttons from which this button triggers
    pub fn with_mouse_button_press_inputs(mut self, buttons: Vec<MouseButton>) -> Button {
        self.mouse_button_press_inputs = buttons;
        self
    }

    /// Sets the max width of the Button
    pub fn set_max_width(&mut self, max_width: u32) {
        self.max_width = max_width;
    }

    /// Sets the text of the Button
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
        self.base.dirty = true;
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
    pub fn set_button_press_inputs(mut self, buttons: Vec<VirtualKeyCode>) {
        self.button_press_inputs = buttons;
    }

    /// Set the mouse buttons from which this button triggers
    pub fn set_mouse_button_press_inputs(mut self, buttons: Vec<MouseButton>) {
        self.mouse_button_press_inputs = buttons;
    }

    /// Returns whether this button was just pressed.
    pub fn was_just_pressed(&self) -> bool {
        self.was_just_pressed
    }
}

impl InterfaceItem for Button {
    fn get_base(&self) -> &InterfaceItemBase {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut InterfaceItemBase {
        &mut self.base
    }

    fn get_total_width(&self) -> u32 {
        self.max_width
    }

    fn get_total_height(&self) -> u32 {
        1
    }

    fn draw(&mut self, text_buffer: &mut TextBuffer) {
        self.base.dirty = false;
        if self.base.focused {
            text_buffer.change_cursor_fg_color(self.fg_color_focused);
            text_buffer.change_cursor_bg_color(self.bg_color_focused);
        } else {
            text_buffer.change_cursor_fg_color(self.fg_color_unfocused);
            text_buffer.change_cursor_bg_color(self.bg_color_unfocused);
        }
        text_buffer.move_cursor(self.base.x as i32, self.base.y as i32);
        text_buffer.write(
            self.text
                .chars()
                .take(self.max_width as usize)
                .collect::<String>(),
        );
    }

    fn handle_events(&mut self, events: &Events) -> bool {
        self.was_just_pressed = false;
        for curr in &self.button_press_inputs {
            if events.keyboard.was_just_pressed(*curr) {
                self.was_just_pressed = true;
                return true;
            }
        }
        for curr in &self.mouse_button_press_inputs {
            if events.mouse.was_just_pressed(*curr) {
                self.was_just_pressed = true;
                return true;
            }
        }
        false
    }

    fn update(&mut self, _: f32) {}
}
