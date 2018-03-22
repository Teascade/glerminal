//! Menu systems enable easy interface creation

mod text_input;
mod text_label;
mod menu;

pub use self::text_input::TextInput;
pub use self::text_label::TextLabel;
pub use self::menu::Menu;
pub use self::menu::MenuList;

use std::collections::HashMap;

use text_buffer::TextBuffer;
use input::Input;
use glutin::VirtualKeyCode;

/// Represents a single menu item: an item that is somewhere, can take input and can be drawn.
pub trait InterfaceItem: InterfaceItemClone {
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
    /// Returns whether this InterfaceItem can be (or should be) focused at all.
    fn can_be_focused(&self) -> bool;
    /// Should the InterfaceItem be redrawn (has changes happened, that mean it should be redrawn)
    fn is_dirty(&self) -> bool;
    /// Set the dirtiness, this should only be called if something else does the drawing.
    fn set_dirty(&mut self, dirty: bool);
    /// Draw the InterfaceItem
    fn draw(&mut self, text_buffer: &mut TextBuffer);
    /// Handle input for this InterfaceItem. Returns whether it handled any input.
    fn handle_input(&mut self, input: &Input) -> bool;
}

/// Represents a cloneable InterfaceItem; You should never implement this yourself, but instead
/// derive Clone for all InterfaceItems.
///
/// E.g.
/// ```
/// #[derive(Clone)]
/// pub struct Test {}
///
/// // Now you can implement only InterfaceItem safely to Test
/// ```
pub trait InterfaceItemClone {
    /// Make a box of the cloned InterfaceItem
    fn clone_box(&self) -> Box<InterfaceItem>;
}

impl<T: 'static + InterfaceItem + Clone> InterfaceItemClone for T {
    fn clone_box(&self) -> Box<InterfaceItem> {
        Box::new(self.clone())
    }
}

/// Represents a HashMap from VirtualKeyCode to character. Used to filter out which characters get registered by the textinput.
#[derive(Clone, Debug)]
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
