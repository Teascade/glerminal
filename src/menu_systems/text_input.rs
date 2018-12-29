use super::{Filter, InterfaceItem, InterfaceItemBase};

use std::iter::repeat;

use text_buffer::Color;
use text_buffer::TextBuffer;
use Events;
use MouseButton;
use VirtualKeyCode;

/// Represents a text-input field, that can be focused, takes in events (.keyboard, text),
/// and it's possible to get the input text with get_text
#[derive(Debug, Clone)]
pub struct TextInput {
    /// Background-color for when the field is unfocused
    pub bg_color_unfocused: Color,
    /// Background-color for when the field is focused
    pub bg_color_focused: Color,
    /// Foreground-color for when the field is unfocused
    pub fg_color_unfocused: Color,
    /// Foreground-color for when the field is focused
    pub fg_color_focused: Color,

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
    /// Initializes a new TextInput with the given position and width
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
            bg_color_unfocused: [0.0, 0.0, 0.0, 0.0],
            bg_color_focused: [0.8, 0.8, 0.8, 1.0],
            fg_color_unfocused: [0.8, 0.8, 0.8, 1.0],
            fg_color_focused: [0.2, 0.2, 0.2, 1.0],

            base: InterfaceItemBase::new(true),
            min_width: actual_min_width,
            max_width: actual_max_width,

            character_limit: None,
            text: String::new(),
            prefix: String::new(),
            suffix: String::new(),
            filter: Filter::empty_filter(),

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
    with_set_colors!(TextInput);

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
    }

    /// Limtis the amount of characters that the TextInput will accept.
    pub fn set_character_limit<T: Into<Option<u32>>>(mut self, char_limit: T) {
        self.character_limit = char_limit.into();
    }

    /// Set the current text
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
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
            text_width = self.text.len().max(min_width as usize) as u32;
        } else {
            text_width = self.text.len() as u32;
        }
        (self.prefix.len() + self.suffix.len()) as u32 + text_width
    }

    fn get_total_height(&self) -> u32 {
        1
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
        text_buffer.move_cursor(self.base.x as i32, self.base.y as i32);

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
            text_width = ((max_width - text_w_offset) as usize).min(self.text.len());
            field_width = min_width.max(self.text.len() as u32).min(max_width);
        } else if let Some(min_width) = self.min_width {
            // Only min width
            text_width = self.text.len();
            field_width = min_width.max(self.text.len() as u32 + text_w_offset);
        } else if let Some(max_width) = self.max_width {
            // Only max width
            text_width = ((max_width - text_w_offset) as usize).min(self.text.len());
            field_width = max_width.min(self.text.len() as u32 + 1);
        } else {
            // Neither
            text_width = self.text.len();
            field_width = (self.text.len() as u32 + text_w_offset).max(1);
        }

        let mut text: String = self.text[(self.text.len() - text_width)..].to_string();
        if self.caret_showing {
            text.push('_');
        }

        let spaces: String = repeat(" ")
            .take(field_width as usize - text_width - space_offset)
            .collect();
        let text = text + &*spaces;

        text_buffer.write(format!("{}{}{}", self.prefix, text, self.suffix));
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
            if events.keyboard.was_just_pressed(VirtualKeyCode::Back) {
                self.text.pop();
                self.base.dirty = true;
                handled = true;
            }
            for keycode in events.keyboard.get_just_pressed_list() {
                if self.character_limit.is_none()
                    || self.character_limit.unwrap() > self.text.len() as u32
                {
                    if let Some(mut character) = self.filter.get(&keycode) {
                        let mut text = String::new();
                        if events.keyboard.is_pressed(VirtualKeyCode::LShift)
                            || events.keyboard.is_pressed(VirtualKeyCode::RShift)
                        {
                            text.push_str(&*character.to_uppercase().to_string());
                        } else {
                            text.push(*character);
                        }
                        self.text.push_str(&*text);
                        self.base.dirty = true;
                        handled = true;
                    }
                }
            }
        }
        handled
    }

    fn update(&mut self, delta: f32) {
        if !self.base.is_focused() || self.caret == 0.0 {
            self.caret_timer = 0.0;
            self.caret_showing = false;
        } else {
            self.caret_timer += delta;
            if self.caret_timer >= self.caret {
                self.caret_timer -= self.caret;
                self.caret_showing = !self.caret_showing;
                self.base.dirty = true;
            }
        }
    }
}
