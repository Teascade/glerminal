//! Module doc
use std::collections::HashMap;
use std::iter;

use text_buffer::TextBuffer;
use input::Input;
use glutin::VirtualKeyCode;

/// Represents a single menu item: an item that is somewhere, can take input and can be drawn.
pub trait InterfaceItem {
    /// Get the position of this InterfaceItem
    fn get_pos(&self) -> (u32, u32);
    /// Set the (absolute) position of this InterfaceItem
    fn set_pos(&mut self, pos: (u32, u32));
    /// Get the width this InterfaceItem can take up
    ///
    /// This should ideally never change
    fn get_total_width(&self) -> u32;
    /// Get the height this InterfaceItem can take up
    ///
    /// This should ideally never change
    fn get_total_height(&self) -> u32;
    /// Is the InterfaceItem currently focused
    fn is_focused(&self) -> bool;
    /// Un/Focus the InterfaceItem
    fn set_focused(&mut self, focused: bool);
    /// Should the InterfaceItem be redrawn (has changes happened, that mean it should be redrawn)
    fn is_dirty(&self) -> bool;
    /// Draw the InterfaceItem
    fn draw(&mut self, text_buffer: &mut TextBuffer);
    /// Handle input for this InterfaceItem. Returns weather it handled any input.
    fn handle_input(&mut self, input: &Input, filter: &Filter) -> bool;
}

/// Represents a text-input field, that can be focused, takes in input (text),
/// and it's possible to get the input with get_text
pub struct TextInput {
    x: u32,
    y: u32,
    width: u32,
    text: String,
    prefix: String,
    suffix: String,
    focused: bool,
    dirty: bool,
}

impl InterfaceItem for TextInput {
    fn get_pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    fn set_pos(&mut self, pos: (u32, u32)) {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
    }

    fn get_total_width(&self) -> u32 {
        (self.prefix.len() + self.suffix.len()) as u32 + self.width
    }

    fn get_total_height(&self) -> u32 {
        1
    }

    fn is_focused(&self) -> bool {
        self.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn draw(&mut self, text_buffer: &mut TextBuffer) {
        self.dirty = false;

        if self.focused {
            text_buffer.change_cursor_bg_color([0.8, 0.8, 0.8, 1.0]);
            text_buffer.change_cursor_fg_color([0.2, 0.2, 0.2, 1.0]);
        } else {
            text_buffer.change_cursor_bg_color([0.0, 0.0, 0.0, 0.0]);
            text_buffer.change_cursor_fg_color([0.8, 0.8, 0.8, 1.0]);
        }
        text_buffer.move_cursor(self.x as i32, self.y as i32);
        let text_width = (self.width as usize).min(self.text.len());
        let text: String = self.text[(self.text.len() - text_width)..].to_string();
        let spaces: String = iter::repeat(" ")
            .take(self.width as usize - text_width)
            .collect();
        let text = text + &*spaces;
        text_buffer.write(format!("{}{}{}", self.prefix, text, self.suffix));
    }

    fn handle_input(&mut self, input: &Input, filter: &Filter) -> bool {
        let mut handled = false;
        if self.focused {
            if input.was_just_pressed(VirtualKeyCode::Back) {
                self.text.pop();
                self.dirty = true;
                handled = true;
            }
            for keycode in input.get_just_pressed_list() {
                if let Some(mut character) = filter.get(&keycode) {
                    let mut text = String::new();
                    if input.is_pressed(VirtualKeyCode::LShift)
                        || input.is_pressed(VirtualKeyCode::RShift)
                        {
                            text.push_str(&*character.to_uppercase().to_string());
                        } else {
                        text.push(*character);
                    }
                    self.text.push_str(&*text);
                    self.dirty = true;
                    handled = true;
                }
            }
        }
        handled
    }
}

impl TextInput {
    /// Initializes a new TextInput with the given position and width
    pub fn new(width: u32) -> TextInput {
        TextInput {
            x: 0,
            y: 0,
            width: width,
            text: String::new(),
            prefix: String::new(),
            suffix: String::new(),
            focused: false,
            dirty: true,
        }
    }

    /// Sets the position of the TextInput
    pub fn with_pos(mut self, position: (u32, u32)) -> TextInput {
        let (x, y) = position;
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the width of the TextInput.
    pub fn with_width(mut self, width: u32) -> TextInput {
        self.width = width;
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

    /// Sets weather the TextInput is focused.
    pub fn with_focus(mut self, focused: bool) -> TextInput {
        self.focused = focused;
        self
    }
}

/// Represents a HashMap from VirtualKeyCode to character. Used to filter out which characters get registered by the textinput.
pub struct Filter {
    map: HashMap<VirtualKeyCode, char>,
}

impl Filter {
    /// Create an empty filter, where other filters can be added, such as basic_latin_keycode_filter
    pub fn empty_filter() -> Filter {
        Filter {
            map: HashMap::new(),
        }
    }

    /// Creates a Filter with basic latin characters
    ///
    /// Includes characters a-z and spacebar.
    pub fn with_basic_latin_characters(mut self) -> Filter {
        self.map.insert(VirtualKeyCode::A, 'a');
        self.map.insert(VirtualKeyCode::B, 'b');
        self.map.insert(VirtualKeyCode::C, 'c');
        self.map.insert(VirtualKeyCode::D, 'd');
        self.map.insert(VirtualKeyCode::E, 'e');
        self.map.insert(VirtualKeyCode::F, 'f');
        self.map.insert(VirtualKeyCode::G, 'g');
        self.map.insert(VirtualKeyCode::H, 'h');
        self.map.insert(VirtualKeyCode::I, 'i');
        self.map.insert(VirtualKeyCode::J, 'j');
        self.map.insert(VirtualKeyCode::K, 'k');
        self.map.insert(VirtualKeyCode::L, 'l');
        self.map.insert(VirtualKeyCode::M, 'm');
        self.map.insert(VirtualKeyCode::N, 'n');
        self.map.insert(VirtualKeyCode::O, 'o');
        self.map.insert(VirtualKeyCode::P, 'p');
        self.map.insert(VirtualKeyCode::Q, 'q');
        self.map.insert(VirtualKeyCode::R, 'r');
        self.map.insert(VirtualKeyCode::S, 's');
        self.map.insert(VirtualKeyCode::T, 't');
        self.map.insert(VirtualKeyCode::U, 'u');
        self.map.insert(VirtualKeyCode::V, 'v');
        self.map.insert(VirtualKeyCode::W, 'w');
        self.map.insert(VirtualKeyCode::X, 'x');
        self.map.insert(VirtualKeyCode::Y, 'y');
        self.map.insert(VirtualKeyCode::Z, 'z');
        self.map.insert(VirtualKeyCode::Space, ' ');
        self
    }

    /// Creates a Filter with basic numerals
    ///
    /// Includes numerals from 0-9
    pub fn with_basic_numerals(mut self) -> Filter {
        self.map.insert(VirtualKeyCode::Key0, '0');
        self.map.insert(VirtualKeyCode::Key1, '1');
        self.map.insert(VirtualKeyCode::Key2, '2');
        self.map.insert(VirtualKeyCode::Key3, '3');
        self.map.insert(VirtualKeyCode::Key4, '4');
        self.map.insert(VirtualKeyCode::Key5, '5');
        self.map.insert(VirtualKeyCode::Key6, '6');
        self.map.insert(VirtualKeyCode::Key7, '7');
        self.map.insert(VirtualKeyCode::Key8, '8');
        self.map.insert(VirtualKeyCode::Key9, '9');

        self
    }

    /// Creates a Filter with basic special symbols
    ///
    /// Includes numerals from 0-9
    pub fn with_basic_special_symbols(mut self) -> Filter {
        self.map.insert(VirtualKeyCode::Apostrophe, '\'');
        self.map.insert(VirtualKeyCode::Backslash, '\\');
        self.map.insert(VirtualKeyCode::Colon, ':');
        self.map.insert(VirtualKeyCode::Period, '.');
        self.map.insert(VirtualKeyCode::Semicolon, ';');
        self.map.insert(VirtualKeyCode::Comma, ',');
        self.map.insert(VirtualKeyCode::Equals, '=');
        self.map.insert(VirtualKeyCode::Subtract, '-');
        self.map.insert(VirtualKeyCode::Multiply, '*');
        self.map.insert(VirtualKeyCode::Underline, '_');
        self.map.insert(VirtualKeyCode::Slash, '/');
        self.map.insert(VirtualKeyCode::LBracket, '[');
        self.map.insert(VirtualKeyCode::RBracket, ']');

        self
    }

    /// Add a specific VirtualKeyCode: char pair to this filter and return the current filter.
    ///
    /// This would mean that when you press the keycode, the character specified will be typed.
    pub fn with_pair(mut self, keycode: VirtualKeyCode, character: char) -> Filter {
        self.map.insert(keycode, character);
        self
    }

    /// Insert a specific VirtualKeyCode: char pair to this filter.
    ///
    /// Works similarly to with_pair
    pub fn insert(&mut self, keycode: VirtualKeyCode, character: char) {
        self.map.insert(keycode, character);
    }

    /// Get the character from the specified VirtualKeyCode, None if it doesn't exist.
    pub fn get(&self, keycode: &VirtualKeyCode) -> Option<&char> {
        self.map.get(keycode)
    }
}
