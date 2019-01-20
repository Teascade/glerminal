use crate::text_buffer::{Color, TextBuffer, TextStyle};

pub struct BorderChars {
    pub left_top_corner: char,
    pub left_bottom_corner: char,
    pub right_top_corner: char,
    pub right_bottom_corner: char,

    pub vertical_line: char,
    pub horizontal_line: char,

    pub top_split: char,
    pub bottom_split: char,
    pub left_split: char,
    pub right_split: char,
    pub middle_split: char,
}

impl BorderChars {
    /// Make BorderChars where every field is a spacebar.
    pub fn empty() -> BorderChars {
        BorderChars {
            left_top_corner: ' ',
            left_bottom_corner: ' ',
            right_top_corner: ' ',
            right_bottom_corner: ' ',

            vertical_line: ' ',
            horizontal_line: ' ',

            top_split: ' ',
            bottom_split: ' ',
            left_split: ' ',
            right_split: ' ',
            middle_split: ' ',
        }
    }
}

impl Default for BorderChars {
    fn default() -> BorderChars {
        BorderChars {
            left_top_corner: '╔',
            left_bottom_corner: '╚',
            right_top_corner: '╗',
            right_bottom_corner: '╝',

            vertical_line: '═',
            horizontal_line: '║',

            top_split: '╦',
            bottom_split: '╩',
            left_split: '╠',
            right_split: '╣',
            middle_split: '╬',
        }
    }
}

/// Represents a window that clears everything in it's way and is able to limit the cursor within it's bounds with `set_limits`.
///
/// Windows API and usage is still subject to change in future updates.
///
/// For example a 2x2 window creates a 2x2 area and surrounds it with borders.
/// ```
/// use glerminal::menu_systems::Window;
///
/// Window::new(2, 2);
///
/// // Creates a window that looks like (box-drawing characters used in places of borders)
/// // ████
/// // █  █
/// // █  █
/// // ████
/// ```
pub struct Window {
    x: u32,
    y: u32,
    /// The width of the window
    pub width: u32,
    /// The height of the window
    pub height: u32,
    /// Sets the title of the window
    pub title: String,
    /// The border style for the window
    pub border_style: TextStyle,
    /// The characters used for determining borders
    pub border_chars: BorderChars,
    /// The background (inside the frame) color of the window.
    pub background_color: Color,
}

impl Window {
    /// Creates a new window with the given width and height
    pub fn new(width: u32, height: u32) -> Window {
        Window {
            x: 0,
            y: 0,
            width: width.max(1),
            height: height.max(1),
            title: String::new(),
            border_style: Default::default(),
            border_chars: Default::default(),
            background_color: [0.0; 4],
        }
    }

    /// Sets the initial position of the window.
    pub fn with_pos(mut self, position: (u32, u32)) -> Window {
        let (x, y) = position;
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the initial width of the window
    pub fn with_width(mut self, width: u32) -> Window {
        self.width = width;
        self
    }

    /// Sets the initial height of the window
    pub fn with_height(mut self, height: u32) -> Window {
        self.height = height;
        self
    }

    /// Sets the initial title of the window
    pub fn with_title<T: Into<String>>(mut self, title: T) -> Window {
        self.title = title.into();
        self
    }

    /// Sets the initial border style for the window
    pub fn with_border_color(mut self, style: TextStyle) -> Window {
        self.border_style = style;
        self
    }

    /// Set the background (inside the frame) color of the window.
    pub fn with_background_color(mut self, color: Color) -> Window {
        self.background_color = color;
        self
    }

    /// Sets the position of the window.
    pub fn set_pos(&mut self, position: (u32, u32)) {
        let (x, y) = position;
        self.x = x;
        self.y = y;
    }

    /// Draws the window
    pub fn draw(&self, text_buffer: &mut TextBuffer) {
        for y in 0..(self.height + 2) {
            text_buffer.cursor.move_to(self.x, self.y + y);
            for x in 0..(self.width + 2) {
                if x == 0 || y == 0 || x == self.width + 1 || y == self.height + 1 {
                    text_buffer.cursor.style = self.border_style;
                } else {
                    text_buffer.cursor.style = TextStyle {
                        bg_color: self.background_color,
                        ..Default::default()
                    };
                }
                text_buffer.put_char(' ');
            }
        }
        text_buffer.cursor.move_to(self.x + 1, self.y);
        text_buffer.cursor.style = self.border_style;
        text_buffer.write(
            self.title
                .chars()
                .take(self.width as usize)
                .collect::<String>(),
        );
    }

    /// Set limits for the TextBuffer so that nothing can be written outside the window.
    pub fn set_limits(&self, text_buffer: &mut TextBuffer) {
        text_buffer.cursor.set_limits(
            Some(self.x),
            Some(self.x + self.width + 1),
            Some(self.y),
            Some(self.y + self.height + 1),
        );
    }
}
