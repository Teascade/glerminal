use super::{InterfaceItem, InterfaceItemBase};
use events::Events;
use text_buffer::{Color, TextBuffer};

/// Represents a TextLabel that simply shows a row of text in the menu
#[derive(Debug, Clone)]
pub struct TextLabel {
    /// Background-color
    pub fg_color: Color,
    /// Background-color
    pub bg_color: Color,
    base: InterfaceItemBase,
    max_width: u32,
    text: String,
}

impl TextLabel {
    /// Creates a new TextLabel with the given text and max width
    ///
    /// Max width determines how wide the text can be maximally before cutting off.
    pub fn new<T: Into<String>>(text: T, max_width: u32) -> TextLabel {
        TextLabel {
            base: InterfaceItemBase::new(false),
            max_width: max_width,
            text: text.into(),
            bg_color: [0.0, 0.0, 0.0, 0.0],
            fg_color: [0.8, 0.8, 0.8, 1.0],
        }
    }

    /// Sets the initial position of the TextLabel
    pub fn with_pos(mut self, pos: (u32, u32)) -> TextLabel {
        let (x, y) = pos;
        self.base.x = x;
        self.base.y = y;
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
        self.base.dirty = true;
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
        self.base.dirty = true;
        text_buffer.change_cursor_bg_color(self.bg_color);
        text_buffer.change_cursor_fg_color(self.fg_color);
        text_buffer.move_cursor(self.base.x as i32, self.base.y as i32);
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
