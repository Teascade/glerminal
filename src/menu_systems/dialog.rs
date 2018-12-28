use glutin::VirtualKeyCode;

use super::{InterfaceItem, InterfaceItemBase};

use events::Events;
use text_buffer::{Color, TextBuffer};

use std::iter::repeat;

/// Represents a dialog-window, meaning a window of text that can be scrolled up and down.
#[derive(Debug, Clone)]
pub struct Dialog {
    /// Background-color when the dialog is unfocused
    pub fg_color_unfocused: Color,
    /// Background-color when the dialog is unfocused
    pub bg_color_unfocused: Color,
    /// Background-color when the dialog is focused
    pub fg_color_focused: Color,
    /// Background-color when the dialog is focused
    pub bg_color_focused: Color,

    base: InterfaceItemBase,
    width: u32,

    min_height: Option<u32>,
    max_height: Option<u32>,

    text: String,
    rows: Vec<String>,

    scroll_idx: u32,
}

impl Dialog {
    /// Creates a new dialog input with the given width and optionally given min and max heights.
    pub fn new<T: Into<Option<u32>>, U: Into<Option<u32>>>(
        width: u32,
        min_height: T,
        max_height: U,
    ) -> Dialog {
        Dialog {
            fg_color_unfocused: [0.8, 0.8, 0.8, 1.0],
            bg_color_unfocused: [0.0; 4],
            fg_color_focused: [0.2, 0.2, 0.2, 1.0],
            bg_color_focused: [0.8, 0.8, 0.8, 1.0],

            base: InterfaceItemBase::new(true),
            width: width,
            min_height: min_height.into(),
            max_height: max_height.into(),

            text: String::new(),
            rows: Vec::new(),

            scroll_idx: 0,
        }
    }

    /// Sets the initial position of the dialog window.
    pub fn with_pos(mut self, position: (u32, u32)) -> Dialog {
        let (x, y) = position;
        self.base.x = x;
        self.base.y = y;
        self
    }

    /// Sets the initial width of the dialog window
    pub fn with_width(mut self, width: u32) -> Dialog {
        self.width = width;
        self.update_rows();
        self
    }

    /// Sets the initial minimum height of the dialog window
    pub fn with_min_height(mut self, min_height: Option<u32>) -> Dialog {
        self.min_height = min_height;
        self
    }

    /// Sets the initial maximum height of the dialog window
    pub fn with_max_height(mut self, max_height: Option<u32>) -> Dialog {
        self.max_height = max_height;
        self
    }

    /// Sets the initial colors (fg, bg) of the dialog window when it is unfocused
    pub fn with_unfocused_colors(mut self, fg: Color, bg: Color) -> Dialog {
        self.fg_color_unfocused = fg;
        self.bg_color_unfocused = bg;
        self
    }

    /// Sets the colors (fg, bg) of the dialog window when it is focused
    pub fn with_focused_colors(mut self, fg: Color, bg: Color) -> Dialog {
        self.fg_color_focused = fg;
        self.bg_color_focused = bg;
        self
    }

    /// Sets the initial text of the dialog window
    pub fn with_text<T: Into<String>>(mut self, text: T) -> Dialog {
        self.text = text.into();
        self.update_rows();
        self
    }

    /// Sets whether the dialog window is initially focused or not.
    pub fn with_focused(mut self, focused: bool) -> Dialog {
        self.base.focused = focused;
        self
    }

    /// Sets the width of the dialog window
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.update_rows();
    }

    /// Sets the minimum height of the dialog window
    pub fn set_min_height(&mut self, min_height: Option<u32>) {
        self.min_height = min_height;
    }

    /// Sets the maximum height of the dialog window
    pub fn set_max_height(&mut self, max_height: Option<u32>) {
        self.max_height = max_height;
    }

    /// Sets colors of the dialog window is unfocused
    pub fn set_unfocused_colors(&mut self, fg: Color, bg: Color) {
        self.fg_color_unfocused = fg;
        self.bg_color_unfocused = bg;
    }

    /// Sets colors of the dialog window is focused
    pub fn set_focused_colors(&mut self, fg: Color, bg: Color) {
        self.fg_color_focused = fg;
        self.bg_color_focused = bg;
    }

    /// Sets the text of the dialog window
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
        self.update_rows();
    }

    fn update_rows(&mut self) {
        self.rows = Vec::new();
        let mut curr_row = String::new();

        let mut parts = self.text.split(' ');
        while let Some(word) = parts.next() {
            if (curr_row.len() + word.len() + 1) as u32 <= self.width {
                if curr_row.len() != 0 {
                    curr_row += " ";
                }
                curr_row += word;
            } else {
                self.rows.push(curr_row);
                curr_row = word.to_owned();
            }
        }
        self.rows.push(curr_row);
    }
}

impl InterfaceItem for Dialog {
    fn get_base(&self) -> &InterfaceItemBase {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut InterfaceItemBase {
        &mut self.base
    }

    fn get_total_width(&self) -> u32 {
        self.width
    }

    fn get_total_height(&self) -> u32 {
        if let Some(max_height) = self.max_height {
            max_height
        } else if let Some(min_height) = self.min_height {
            min_height.max(self.rows.len() as u32)
        } else {
            self.rows.len() as u32
        }
    }

    fn draw(&mut self, text_buffer: &mut TextBuffer) {
        self.base.dirty = false;
        if self.base.focused {
            text_buffer.change_cursor_bg_color(self.bg_color_focused);
            text_buffer.change_cursor_fg_color(self.fg_color_focused);
        } else {
            text_buffer.change_cursor_bg_color(self.bg_color_unfocused);
            text_buffer.change_cursor_fg_color(self.fg_color_unfocused);
        }
        for idx in 0..self.get_total_height() {
            let text: String;
            if let Some(row) = self.rows.get((self.scroll_idx + idx) as usize) {
                text = row.to_owned()
                    + &*repeat(' ')
                        .take(self.width as usize - row.len())
                        .collect::<String>();
            } else {
                text = repeat(' ').take(self.width as usize).collect();
            }
            text_buffer.move_cursor(self.base.x as i32, self.base.y as i32 + idx as i32);
            text_buffer.write(text);
        }
    }

    fn handle_events(&mut self, events: &Events) -> bool {
        if events.keyboard.was_just_pressed(VirtualKeyCode::Down) {
            if self.scroll_idx + 1 < self.rows.len() as u32 {
                self.scroll_idx += 1;
                self.base.dirty = true;
                return true;
            }
        } else if events.keyboard.was_just_pressed(VirtualKeyCode::Up) {
            if self.scroll_idx > 0 {
                self.scroll_idx -= 1;
                self.base.dirty = true;
                return true;
            }
        }
        false
    }

    fn update(&mut self, _: f32) {}
}
