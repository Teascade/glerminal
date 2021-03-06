use glutin::VirtualKeyCode;

use super::{InterfaceItem, InterfaceItemBase};

use crate::text_processing::{OptTextStyle, ProcessedChar, TextProcessor};
use crate::{Events, TextBuffer, TextStyle};

use std::iter::repeat;

/// Represents a dialog-window, meaning a window of text that can be scrolled up and down.
///
/// For example
/// ```
/// use glerminal::menu_systems::Dialog;
///
/// Dialog::new(12, None, 3)
///     .with_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis faucibus.");
///
/// // Looks like:
///
/// // Lorem ipsum
/// // dolor sit
/// // amet,
///
/// // And can be scrolled down to reveal more text.
/// ```
#[derive(Debug, Clone)]
pub struct Dialog {
    /// Style of this Dialog when it is unfocused
    pub unfocused_style: TextStyle,
    /// Style of this Dialog when it is focused
    pub focused_style: TextStyle,
    /// The buttons that make the dialog scroll up when focused
    pub up_buttons: Vec<VirtualKeyCode>,
    /// The buttons that make the dialog scroll down when focused
    pub down_buttons: Vec<VirtualKeyCode>,

    base: InterfaceItemBase,
    width: u32,

    min_height: Option<u32>,
    max_height: Option<u32>,

    text: String,
    rows: Vec<Vec<ProcessedChar>>,

    processed_text: Vec<ProcessedChar>,
    needs_processing: bool,

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
            unfocused_style: TextStyle {
                bg_color: [0.0, 0.0, 0.0, 0.0],
                fg_color: [0.8, 0.8, 0.8, 1.0],
                ..Default::default()
            },
            focused_style: TextStyle {
                bg_color: [0.8, 0.8, 0.8, 1.0],
                fg_color: [0.2, 0.2, 0.2, 1.0],
                ..Default::default()
            },
            up_buttons: vec![VirtualKeyCode::Up],
            down_buttons: vec![VirtualKeyCode::Down],

            base: InterfaceItemBase::new(true),
            width: width,

            min_height: min_height.into(),
            max_height: max_height.into(),

            text: String::new(),
            rows: Vec::new(),

            processed_text: Vec::new(),
            needs_processing: true,

            scroll_idx: 0,
        }
    }

    with_base!(Dialog);
    with_style!(Dialog);

    /// Sets the initial width of the dialog window
    pub fn with_width(mut self, width: u32) -> Dialog {
        self.width = width;
        self.needs_processing = true;
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
        self.needs_processing = true;
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
        self.needs_processing = true;
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
        self.needs_processing = true;
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
        let width = self.width;

        let mut curr_word = Vec::new();

        let mut last_style = OptTextStyle {
            fg_color: None,
            bg_color: None,
            shakiness: None,
        };
        self.rows = Vec::new();
        let mut curr_row = Vec::new();

        // Add an empty character at the end, so that every word is added processed properly.
        let empty = ProcessedChar {
            character: ' ',
            style: last_style.clone(),
        };
        let mut text = self.processed_text.clone();
        text.push(empty);

        // Process each letter
        for c in text {
            if ((c.character == ' ' || c.character == '\n') && !curr_word.is_empty())
                || curr_word.len() as u32 >= width
            {
                // A word breaker found, this is a separate word now.
                if ((curr_row.len() + curr_word.len() + 1) as u32) <= self.width {
                    if !curr_row.is_empty() {
                        curr_row.push(ProcessedChar {
                            character: ' ',
                            style: last_style.clone(),
                        });
                    }
                    curr_row.append(&mut curr_word.clone());
                } else {
                    self.rows.push(curr_row);
                    curr_row = curr_word.clone();
                }
                if let Some(last) = curr_word.last() {
                    last_style = last.style.clone()
                }
                // Found \n => make new row
                if c.character == '\n' {
                    self.rows.push(curr_row.clone());
                    curr_row.clear();
                }
                curr_word = Vec::new();
            } else {
                curr_word.push(c);
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

        text_buffer.cursor.style = if self.base.is_focused() {
            self.focused_style
        } else {
            self.unfocused_style
        };
        let none_style = OptTextStyle {
            fg_color: None,
            bg_color: None,
            shakiness: None,
        };
        for idx in 0..self.get_total_height() {
            let text: Vec<ProcessedChar>;
            if let Some(row) = self.rows.get((self.scroll_idx + idx) as usize) {
                text = row
                    .iter()
                    .cloned()
                    .chain(
                        repeat(ProcessedChar {
                            character: ' ',
                            style: none_style.clone(),
                        })
                        .take(self.width as usize - row.len()),
                    )
                    .collect();
            } else {
                text = repeat(ProcessedChar {
                    character: ' ',
                    style: none_style.clone(),
                })
                .take(self.width as usize)
                .collect();
            }
            text_buffer.cursor.move_to(self.base.x, self.base.y + idx);
            text_buffer.write_processed(&text);
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

    fn update(&mut self, _: f32, processor: &TextProcessor) {
        if self.needs_processing {
            self.processed_text = processor.process(vec![self.text.clone().into()]);
            self.update_rows();
            self.needs_processing = false;
        }
    }
}
