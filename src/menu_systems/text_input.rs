use super::{Filter, InterfaceItem, InterfaceItemBase};

use std::iter::repeat;

use crate::text_processing::{Processable, ProcessedChar, TextProcessor};
use crate::{Events, MouseButton, TextBuffer, TextStyle, VirtualKeyCode};

/// Represents a text-input field, that can be focused, takes in events (keyboard events as text),
/// and it's possible to get the input text with get_text
///
/// TextInput.filter is used to determine which symbols are registered and then written.
/// Example of a filter that will take in latin alphabetic letters and numerals:
/// ```
/// use glerminal::menu_systems::{Filter, TextInput};
/// use glerminal::VirtualKeyCode::Equals;
///
/// let filter = Filter::empty_filter()
///     .with_basic_latin_characters() // Add basic latin letters
///     .with_basic_numerals()         // Add basic numerals
///     .with_char('=');       // Add custom character
///
/// TextInput::new(None, None).with_filter(filter);
/// ```
#[derive(Debug, Clone)]
pub struct TextInput {
    /// Style of this TextInput when it is unfocused
    pub unfocused_style: TextStyle,
    /// Style of this TextInput when it is focused
    pub focused_style: TextStyle,

    /// The keyboard inputs that trigger `was_just_pressed`
    pub button_press_inputs: Vec<VirtualKeyCode>,
    /// The mouse inputs that trigger `was_just_pressed`
    pub mouse_button_press_inputs: Vec<MouseButton>,

    base: InterfaceItemBase,

    min_width: Option<u32>,
    max_width: Option<u32>,
    character_limit: Option<u32>,

    text: String,
    prefix: String,
    suffix: String,

    processed_text: Vec<ProcessedChar>,
    needs_processing: bool,

    // Cache text.chars().count() for optimization
    text_width: u32,

    /// The filter used to determine which button presses are registered for writing
    pub filter: Filter,

    was_just_pressed: bool,

    /// Determines how often (in seconds) the caret's status should update.
    ///
    /// Set 0.0 for no caret.
    pub caret: f32,
    caret_timer: f32,
    caret_showing: bool,
}

impl TextInput {
    /// Initializes a new TextInput with the given min and max width
    pub fn new<T: Into<Option<u32>>, U: Into<Option<u32>>>(
        min_width: T,
        max_width: U,
    ) -> TextInput {
        let mut actual_min_width = None;
        let mut actual_max_width = None;
        if let Some(min_w) = min_width.into() {
            actual_min_width = Some(min_w.max(1));
        }
        if let Some(max_w) = max_width.into() {
            actual_max_width = Some(max_w.max(1));
        }
        TextInput {
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

            base: InterfaceItemBase::new(true),
            min_width: actual_min_width,
            max_width: actual_max_width,

            character_limit: None,
            text: String::new(),
            prefix: String::new(),
            suffix: String::new(),
            filter: Filter::empty_filter(),

            processed_text: Vec::new(),
            needs_processing: true,

            text_width: 0,

            button_press_inputs: vec![VirtualKeyCode::Return],
            mouse_button_press_inputs: Vec::new(),
            was_just_pressed: false,

            caret: 0.5,
            caret_timer: 0.0,
            caret_showing: false,
        }
    }

    with_base!(TextInput);
    with_set_pressable!(TextInput);
    with_style!(TextInput);

    /// Sets the width of the TextInput.
    pub fn with_width<T: Into<Option<u32>>, U: Into<Option<u32>>>(
        mut self,
        min_width: T,
        max_width: U,
    ) -> TextInput {
        let mut actual_min_width = None;
        let mut actual_max_width = None;
        if let Some(min_w) = min_width.into() {
            actual_min_width = Some(min_w.max(1));
        }
        if let Some(max_w) = max_width.into() {
            actual_max_width = Some(max_w.max(1));
        }
        self.min_width = actual_min_width;
        self.max_width = actual_max_width;
        self
    }

    /// Sets the text of the TextInput.
    pub fn with_text<T: Into<String>>(mut self, text: T) -> TextInput {
        self.text = text.into();
        self.text_width = self.text.chars().count() as u32;
        self
    }

    /// Sets the prefix text of the TextInput.
    pub fn with_prefix<T: Into<String>>(mut self, prefix: T) -> TextInput {
        self.prefix = prefix.into();
        self
    }

    /// Sets the suffix text of the TextInput.
    pub fn with_suffix<T: Into<String>>(mut self, suffix: T) -> TextInput {
        self.suffix = suffix.into();
        self
    }

    /// Sets the filter for the TextInput.
    pub fn with_filter(mut self, filter: Filter) -> TextInput {
        self.filter = filter;
        self
    }

    /// Determines how often (in seconds) the caret's status should update.
    ///
    /// Set 0.0 for no caret.
    pub fn with_caret(mut self, delay: f32) -> TextInput {
        self.caret = delay;
        self
    }

    /// Limtis the amount of characters that the TextInput will accept.
    pub fn with_character_limit<T: Into<Option<u32>>>(mut self, char_limit: T) -> TextInput {
        self.character_limit = char_limit.into();
        self
    }

    /// Sets the width of the TextInput.
    pub fn set_width<T: Into<Option<u32>>, U: Into<Option<u32>>>(
        mut self,
        min_width: T,
        max_width: U,
    ) {
        let mut actual_min_width = None;
        let mut actual_max_width = None;
        if let Some(min_w) = min_width.into() {
            actual_min_width = Some(min_w.max(1));
        }
        if let Some(max_w) = max_width.into() {
            actual_max_width = Some(max_w.max(1));
        }
        self.min_width = actual_min_width;
        self.max_width = actual_max_width;

        self.needs_processing = true;
        self.base.dirty = true;
    }

    /// Limtis the amount of characters that the TextInput will accept.
    pub fn set_character_limit<T: Into<Option<u32>>>(mut self, char_limit: T) {
        self.character_limit = char_limit.into();
    }

    /// Set the current text
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
        self.text_width = self.text.chars().count() as u32;
        self.needs_processing = true;
        self.base.dirty = true;
    }

    /// Returns the current text in the input
    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    #[cfg(test)]
    pub fn caret_showing(&self) -> bool {
        self.caret_showing
    }
}

impl InterfaceItem for TextInput {
    fn get_base(&self) -> &InterfaceItemBase {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut InterfaceItemBase {
        &mut self.base
    }

    fn get_total_width(&self) -> u32 {
        let text_width;
        if let Some(max_width) = self.max_width {
            text_width = max_width
        } else if let Some(min_width) = self.min_width {
            text_width = self.text_width.max(min_width);
        } else {
            text_width = self.text_width as u32;
        }
        (self.prefix.chars().count() + self.suffix.chars().count()) as u32 + text_width
    }

    fn get_total_height(&self) -> u32 {
        1
    }

    fn draw(&mut self, text_buffer: &mut TextBuffer) {
        self.base.dirty = false;

        text_buffer.cursor.style = if self.base.is_focused() {
            self.focused_style
        } else {
            self.unfocused_style
        };
        text_buffer.cursor.move_to(self.base.x, self.base.y);
        text_buffer.write_processed(&self.processed_text);
    }

    fn handle_events(&mut self, events: &Events) -> bool {
        self.was_just_pressed = false;

        let mut handled = false;
        if self.base.is_focused() {
            for curr in &self.button_press_inputs {
                if events.keyboard.was_just_pressed(*curr) {
                    self.was_just_pressed = true;
                    break;
                }
            }
            for curr in &self.mouse_button_press_inputs {
                if events.mouse.was_just_pressed(*curr) {
                    self.was_just_pressed = true;
                    break;
                }
            }
            for character in events.chars.get_chars() {
                if character == '\u{8}' {
                    // Backspace
                    self.text.pop();
                }

                if (self.character_limit.is_none()
                    || self.character_limit.unwrap() > self.text_width)
                    && self.filter.has(character)
                {
                    self.text.push(character);
                }

                self.base.dirty = true;
                self.needs_processing = true;
                handled = true;

                self.text_width = self.text.chars().count() as u32;
            }
        }
        handled
    }

    fn update(&mut self, delta: f32, processor: &TextProcessor) {
        if !self.base.is_focused() || self.caret == 0.0 {
            if self.caret_showing {
                self.needs_processing = true;
            }
            self.caret_timer = 0.0;
            self.caret_showing = false;
        } else {
            self.caret_timer += delta;
            if self.caret_timer >= self.caret {
                self.caret_timer -= self.caret;
                self.caret_showing = !self.caret_showing;
                self.base.dirty = true;
                self.needs_processing = true;
            }
        }

        if self.needs_processing {
            let text_w_offset: u32;
            if self.base.is_focused() && self.caret != 0.0 {
                text_w_offset = 1
            } else {
                text_w_offset = 0
            }
            let space_offset = if self.caret_showing { 1 } else { 0 };

            let text_width;
            let field_width;
            if let (Some(min_width), Some(max_width)) = (self.min_width, self.max_width) {
                // Max width and min width
                text_width = (max_width - text_w_offset).min(self.text_width);
                field_width = min_width.max(self.text_width).min(max_width);
            } else if let Some(min_width) = self.min_width {
                // Only min width
                text_width = self.text_width;
                field_width = min_width.max(self.text_width + text_w_offset);
            } else if let Some(max_width) = self.max_width {
                // Only max width
                text_width = (max_width - text_w_offset).min(self.text_width);
                field_width = max_width.min(self.text_width + 1);
            } else {
                // Neither
                text_width = self.text_width;
                field_width = (self.text_width + text_w_offset).max(1);
            }

            let mut text: String = self.text.chars().take(text_width as usize).collect();
            if self.caret_showing {
                text.push('_');
            }

            let spaces: String = repeat(" ")
                .take((field_width - text_width - space_offset) as usize)
                .collect();
            let text = text + &*spaces;

            self.processed_text = processor.process(vec![
                self.prefix.clone().into(),
                Processable::NoProcess(text),
                self.suffix.clone().into(),
            ]);

            self.needs_processing = false;
        }
    }
}
