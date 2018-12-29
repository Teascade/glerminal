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
    /// The buttons that make the dialog scroll up when focused
    pub up_buttons: Vec<VirtualKeyCode>,
    /// The buttons that make the dialog scroll down when focused
    pub down_buttons: Vec<VirtualKeyCode>,

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
            up_buttons: vec![VirtualKeyCode::Up],
            down_buttons: vec![VirtualKeyCode::Down],

            base: InterfaceItemBase::new(true),
            width: width,

            min_height: min_height.into(),
            max_height: max_height.into(),

            text: String::new(),
            rows: Vec::new(),

            scroll_idx: 0,
        }
    }

    with_base!(Dialog);
    with_set_colors!(Dialog);

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

    /// Sets the initial text of the dialog window
    pub fn with_text<T: Into<String>>(mut self, text: T) -> Dialog {
        self.text = text.into();
        self.update_rows();
        self
    }

    /// Set the initial buttons that scroll the dialog up.
    pub fn with_up_buttons(mut self, buttons: Vec<VirtualKeyCode>) -> Dialog {
        self.up_buttons = buttons;
        self
    }

    /// Set the initial buttons that scroll the dialog down.
    pub fn with_down_buttons(mut self, buttons: Vec<VirtualKeyCode>) -> Dialog {
        self.down_buttons = buttons;
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

    /// Sets the text of the dialog window
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
        self.update_rows();
    }

    /// Attempt to scroll the dialog up. Returns true if successful, false if not.
    pub fn scroll_up(&mut self) -> bool {
        if self.scroll_idx > 0 {
            self.scroll_idx -= 1;
            self.base.dirty = true;
            true
        } else {
            false
        }
    }

    /// Attempt to scroll the dialog down. Returns true if successful, false if not.
    pub fn scroll_down(&mut self) -> bool {
        if self.scroll_idx + 1 < self.rows.len() as u32 {
            self.scroll_idx += 1;
            self.base.dirty = true;
            true
        } else {
            false
        }
    }

    /// Get the current scroll idx of this dialog
    pub fn get_scroll(&self) -> u32 {
        self.scroll_idx
    }

    /// Sets the current scroll idx of this dialog, assuming `scroll` is within boundaries.
    pub fn set_scroll(&mut self, scroll: u32) {
        if scroll < self.rows.len() as u32 {
            self.scroll_idx = scroll;
        }
    }

    fn update_rows(&mut self) {
        self.rows = Vec::new();
        let mut curr_row = String::new();
        let width = self.width;

        let mut parts = self.text.split(' ').flat_map(|word| {
            let mut word = word.to_owned();
            let mut words = Vec::new();
            while word.len() as u32 > width {
                let part = word.split_off(width as usize);
                words.push(word);
                word = part;
            }
            words.push(word);
            words
        });

        while let Some(word) = parts.next() {
            if (curr_row.len() + word.len() + 1) as u32 <= width {
                if curr_row.len() != 0 {
                    curr_row += " ";
                }
                curr_row += &word;
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
        if self.base.is_focused() {
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
        let mut handled = false;
        for button in events.keyboard.get_just_pressed_list() {
            if self.down_buttons.contains(&button) {
                handled = handled || self.scroll_down();
            } else if self.up_buttons.contains(&button) {
                handled = handled || self.scroll_up();
            }
        }
        handled
    }

    fn update(&mut self, _: f32) {}
}
