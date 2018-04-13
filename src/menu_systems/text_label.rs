use super::InterfaceItem;
use events::Events;
use text_buffer::{Color, TextBuffer};

/// Represents a TextLabel that simply shows a row of text in the menu
#[derive(Debug, Clone)]
pub struct TextLabel {
    /// Background-color
    pub fg_color: Color,
    /// Background-color
    pub bg_color: Color,
    x: u32,
    y: u32,
    dirty: bool,
    max_width: u32,
    text: String,
}

impl TextLabel {
    /// Creates a new TextLabel with the given text and max width
    ///
    /// Max width determines how wide the text can be maximally before cutting off.
    pub fn new<T: Into<String>>(text: T, max_width: u32) -> TextLabel {
        TextLabel {
            x: 0,
            y: 0,
            dirty: true,
            max_width: max_width,
            text: text.into(),
            bg_color: [0.0, 0.0, 0.0, 0.0],
            fg_color: [0.8, 0.8, 0.8, 1.0],
        }
    }

    /// Sets the initial position of the TextLabel
    pub fn with_pos(mut self, pos: (u32, u32)) -> TextLabel {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the initial max width of the TextLabel
    pub fn with_max_width(mut self, max_width: u32) -> TextLabel {
        self.max_width = max_width;
        self
    }

    /// Sets the initial text of the TextLabel
    pub fn with_text<T: Into<String>>(mut self, text: T) -> TextLabel {
        self.text = text.into();
        self
    }

    /// Set the initial colors of this TextLabel
    pub fn with_colors(mut self, colors: (Color, Color)) -> TextLabel {
        let (fg, bg) = colors;
        self.fg_color = fg;
        self.bg_color = bg;
        self
    }

    /// Sets the max width of the TextLabel
    pub fn set_max_width(&mut self, max_width: u32) {
        self.max_width = max_width;
    }

    /// Sets the text of the TextLabel
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
        self.dirty = true;
    }

    /// Return the current text of the TextLabel
    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    /// Set the colors of this TextLabel
    pub fn set_colors(&mut self, colors: (Color, Color)) {
        let (fg, bg) = colors;
        self.fg_color = fg;
        self.bg_color = bg;
    }
}

impl InterfaceItem for TextLabel {
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
        false
    }

    fn set_focused(&mut self, _: bool) {}

    fn can_be_focused(&self) -> bool {
        false
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn draw(&mut self, text_buffer: &mut TextBuffer) {
        self.dirty = false;
        text_buffer.change_cursor_bg_color(self.bg_color);
        text_buffer.change_cursor_fg_color(self.fg_color);
        text_buffer.move_cursor(self.x as i32, self.y as i32);
        text_buffer.write(
            self.text
                .chars()
                .take(self.max_width as usize)
                .collect::<String>(),
        );
    }

    fn handle_events(&mut self, _: &Events) -> bool {
        false
    }

    fn update(&mut self, _: f32) {}
}
