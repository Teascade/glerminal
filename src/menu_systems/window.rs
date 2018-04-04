use text_buffer::{Color, TextBuffer};

/// Represents a window that clears everything in it's way and is able to limit the cursor within it's bounds.
pub struct Window {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    title: String,
    border_color: Color,
    border_title_color: Color,
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
            border_color: [0.8, 0.8, 0.8, 1.0],
            border_title_color: [0.2, 0.2, 0.2, 1.0],
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

    /// Sets the initial border color for the window
    pub fn with_border_color(mut self, color: Color) -> Window {
        self.border_color = color;
        self
    }

    /// Sets the initial border title color for the window
    pub fn with_border_title_color(mut self, color: Color) -> Window {
        self.border_title_color = color;
        self
    }

    /// Sets the position of the window.
    pub fn set_pos(&mut self, position: (u32, u32)) {
        let (x, y) = position;
        self.x = x;
        self.y = y;
    }

    /// Sets the width of the window
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    /// Sets the height of the window
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    /// Sets the title of the window
    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        self.title = title.into();
    }

    /// Sets the border color for the window
    pub fn set_border_color(&mut self, color: Color) {
        self.border_color = color;
    }

    /// Sets the border title color for the window
    pub fn set_border_title_color(&mut self, color: Color) {
        self.border_title_color = color;
    }

    /// Draws the window
    pub fn draw(&self, text_buffer: &mut TextBuffer) {
        for y in 0..(self.height + 2) {
            text_buffer.move_cursor(self.x as i32, (self.y + y) as i32);
            for x in 0..(self.width + 2) {
                if x == 0 || y == 0 || x == self.width + 1 || y == self.height + 1 {
                    text_buffer.change_cursor_bg_color(self.border_color);
                } else {
                    text_buffer.change_cursor_bg_color([0.0; 4]);
                }
                text_buffer.put_char(' ');
            }
        }
        text_buffer.move_cursor((self.x + 1) as i32, self.y as i32);
        text_buffer.change_cursor_bg_color(self.border_color);
        text_buffer.change_cursor_fg_color(self.border_title_color);
        text_buffer.write(
            self.title
                .chars()
                .take(self.width as usize)
                .collect::<String>(),
        );
    }

    /// Set limits for the TextBuffer so that nothing can be written outside the window.
    pub fn set_limits(&self, text_buffer: &mut TextBuffer) {
        text_buffer.set_limits(
            Some(self.x),
            Some(self.x + self.width + 2),
            Some(self.y),
            Some(self.y + self.height + 2),
        );
    }
}
