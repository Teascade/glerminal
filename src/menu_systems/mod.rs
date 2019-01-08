//! Menu systems is a module that allows easy creation and usage of [`Menu`](struct.Menu.html)s.
//! Examples of what a menu can be, is ncurses.
//!
//! Simply lists of [`InterfaceItem`](trait.InterfaceItem.html)s, that the user can browse through, press buttons,
//! input text, do whatever you want with GUI items generally. You can even make your own `InterfaceItem`s if you want
//!
//! Current pre-implemented items to use in Menus are
//! - [TextItem](struct.TextItem.html), functions as a text label and a button.
//! - [TextInput](struct.TextInput.html), can accept text input that can be get with `get_text`.
//! - [Dialog](struct.Dialog.html), can be used to display large volumes of text compactly.
//! - [Checkbox](struct.Checkbox.html), can be checked (and unchecked), like a check- or radiobox (if using [CheckboxGroup](struct.CheckboxGroup.html)).
//!
//! **Note:** This module requires _menu_systems_ feature to be enabled.
//!
//! Example usage of menu-systems:
//! (Same example can be found in [`Menu`](struct.Menu.html))
//! ```no_run
//! use glerminal::menu_systems::{Filter, Menu, MenuList, MenuPosition, TextInput, TextItem};
//! use glerminal::{TerminalBuilder, TextBuffer};
//!
//! // Initialize terminal and text buffer
//! let terminal = TerminalBuilder::new().build();
//! let mut text_buffer;
//! match TextBuffer::new(&terminal, (80, 24)) {
//!     Ok(buffer) => text_buffer = buffer,
//!     Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
//! }
//!
//! // Create three example InterfaceItems to use
//! let mut label = TextItem::new("Text label");
//! let mut button = TextItem::new("Press me!").with_is_button(true);
//! let mut input = TextInput::new(None, 10)
//!     .with_filter(Filter::empty_filter().with_basic_latin_characters())
//!     .with_prefix("Name: [")
//!     .with_suffix("]");
//!
//! // Create the actual menu
//! let mut menu = Menu::new().with_focus(true);
//!
//! while terminal.refresh() {
//!
//!     // Update the menu. Update returns weather it should be redrawn.
//!     if menu.update(
//!         &terminal.get_current_events(),
//!         terminal.delta_time(),
//!         &text_buffer,
//!         &mut MenuList::new()
//!             .with_item(&mut label, None)
//!             .with_item(&mut button, MenuPosition::RelativeToLast(0, 1))
//!             // Use MenuPosition to make a gap between label and button
//!             .with_item(&mut input, None),
//!     ) {
//!         text_buffer.clear();              // Clear the screen
//!         menu.draw(&mut text_buffer);      // Draw the menu
//!         terminal.flush(&mut text_buffer); // Apply changes; flush
//!     }
//!
//!     terminal.draw(&text_buffer);
//! }
//! ```

macro_rules! with_set_colors {
    ($name:ident) => {
        /// Set the initial colors when it is unfocused
        pub fn with_unfocused_colors(mut self, colors: (Color, Color)) -> $name {
            let (fg, bg) = colors;
            self.fg_color_unfocused = fg;
            self.bg_color_unfocused = bg;
            self
        }

        /// Set the initial colors when it is focused
        pub fn with_focused_colors(mut self, colors: (Color, Color)) -> $name {
            let (fg, bg) = colors;
            self.fg_color_focused = fg;
            self.bg_color_focused = bg;
            self
        }

        /// Set the colors when it is unfocused
        pub fn set_unfocused_colors(&mut self, colors: (Color, Color)) {
            let (fg, bg) = colors;
            self.fg_color_unfocused = fg;
            self.bg_color_unfocused = bg;
        }

        /// Set the colors when it is focused
        pub fn set_focused_colors(&mut self, colors: (Color, Color)) {
            let (fg, bg) = colors;
            self.fg_color_focused = fg;
            self.bg_color_focused = bg;
        }
    };
}

macro_rules! with_set_pressable {
    ($name:ident) => {
        /// Set the buttons which trigger `was_just_pressed`
        pub fn with_button_press_inputs(mut self, buttons: Vec<VirtualKeyCode>) -> $name {
            self.button_press_inputs = buttons;
            self
        }

        /// Set the mouse buttons which trigger `was_just_pressed`
        pub fn with_mouse_button_press_inputs(mut self, buttons: Vec<MouseButton>) -> $name {
            self.mouse_button_press_inputs = buttons;
            self
        }

        /// Returns whether buttons from `mouse_button_press_inputs` or `button_press_inputs` were pressed just now.
        pub fn was_just_pressed(&self) -> bool {
            self.was_just_pressed
        }
    };
}

/// If you're creating a struct that has `InterfaceItemBase`, calling `with_base!(Struct)` within
/// the impl-call can be useful. This will add two useful functions for setting initial values for `InterfaceItemBase`
/// - `with_pos(mut self, pos: (u32, u32)) -> Struct`
/// - `with_focused(mut self, focused: bool) -> Struct`
#[macro_export]
macro_rules! with_base {
    ($name:ident) => {
        /// Sets the initial position
        pub fn with_pos(mut self, pos: (u32, u32)) -> $name {
            let (x, y) = pos;
            self.base.x = x;
            self.base.y = y;
            self
        }

        /// Set whether the checkbox is initially focused or not
        pub fn with_focused(mut self, focused: bool) -> $name {
            self.base.set_focused(focused);
            self
        }
    };
}

// /// Sets the text of the TextInput.
// pub fn with_text<T: Into<String>>(mut self, text: T) -> TextInput {
//     self.text = text.into();
//     self
// }

mod checkbox;
mod dialog;
mod menu;
mod text_input;
mod text_item;
mod window;

pub use self::checkbox::{Checkbox, CheckboxGroup};
pub use self::dialog::Dialog;
pub use self::menu::{FocusSelection, GrowthDirection, Menu, MenuList, MenuPosition};
pub use self::text_input::TextInput;
pub use self::text_item::TextItem;
pub use self::window::Window;

use std::collections::HashMap;

use events::Events;
use glutin::VirtualKeyCode;
use text_buffer::TextBuffer;

/// Represents a single menu item: an item that is somewhere, can handle events and can be drawn.
///
/// Current pre-implemented items to use in Menus are
/// - [TextItem](struct.TextItem.html), functions as a text label and a button.
/// - [TextInput](struct.TextInput.html), can accept text input that can be get with `get_text`.
/// - [Dialog](struct.Dialog.html), can be used to display large volumes of text compactly.
/// - [Checkbox](struct.Checkbox.html), can be checked (and unchecked), like a check- or radiobox (if using [CheckboxGroup](struct.CheckboxGroup.html)).
///
/// You can make your own InterfaceItems that you can create, draw, and use for Menus by implementing InterfaceItem.
/// To implement InterfaceItem, you need to derive Clone too though.
///
/// A simple example of how to make an InterfaceItem that you can use for Menus
/// ```
/// use glerminal::menu_systems::{InterfaceItem, InterfaceItemBase};
/// use glerminal::{Events, TextBuffer};
///
/// #[derive(Clone)]
/// struct TextLabel {
///     base: InterfaceItemBase,
///     text: String,
/// }
///
/// impl TextLabel {
///     pub fn new(text: String) -> TextLabel {
///         TextLabel {
///             base: InterfaceItemBase::new(false),
///             text: text,
///         }
///     }
/// }
///
/// impl InterfaceItem for TextLabel {
///     fn get_base(&self) -> &InterfaceItemBase {
///         &self.base
///     }
///
///     fn get_mut_base(&mut self) -> &mut InterfaceItemBase {
///         &mut self.base
///     }
///
///     fn get_total_width(&self) -> u32 {
///         self.text.len() as u32
///     }
///
///     fn get_total_height(&self) -> u32 {
///         1
///     }
///
///     fn draw(&mut self, text_buffer: &mut TextBuffer) {
///         self.base.dirty = false;
///         let pos = self.base.get_pos();
///
///         text_buffer.change_cursor_fg_color([0.2, 0.2, 0.2, 1.0]);
///         text_buffer.change_cursor_bg_color([0.0; 4]);
///         text_buffer.move_cursor(pos.0 as i32, pos.1 as i32);
///         text_buffer.write(self.text.clone());
///     }
///
///     fn handle_events(&mut self, _: &Events) -> bool {
///         false
///     }
///
///     fn update(&mut self, _: f32) {}
/// }
/// ```
pub trait InterfaceItem: InterfaceItemClone {
    /// Get the `InterfaceItemBase`
    fn get_base(&self) -> &InterfaceItemBase;
    /// Get the `InterfaceItemBase` as mutable
    fn get_mut_base(&mut self) -> &mut InterfaceItemBase;
    /// Get the width this InterfaceItem can take up
    ///
    /// This should ideally never change
    fn get_total_width(&self) -> u32;
    /// Get the height this InterfaceItem can take up
    ///
    /// This should ideally never change
    fn get_total_height(&self) -> u32;
    /// Draw the InterfaceItem
    fn draw(&mut self, text_buffer: &mut TextBuffer);
    /// Handle events for this InterfaceItem.
    ///
    /// Returns whether it handled any events.
    fn handle_events(&mut self, events: &Events) -> bool;
    /// Update this InterfaceItem; delta is given in seconds. (see [Terminal.delta_time()](../terminal/struct.Terminal.html))
    fn update(&mut self, delta: f32);
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

/// The base for all `interaceItem`s. Contains metadata that is handled similarily in each `InterfaceItem`
#[derive(Debug, Clone)]
pub struct InterfaceItemBase {
    /// Whether this `InterfaceItem` can be (or should be) focused or not
    pub can_be_focused: bool,
    /// Whether this `InterfaceItem` is dirty so it needs redrawing
    /// This should be called every time something changes
    pub dirty: bool,
    x: u32,
    y: u32,
    focused: bool,
}

impl InterfaceItemBase {
    /// Create a new InterfaceItemBase
    ///
    /// (use this if you're making a new InterfaceItem)
    pub fn new(can_be_focused: bool) -> InterfaceItemBase {
        InterfaceItemBase {
            can_be_focused: can_be_focused,
            dirty: false,
            x: 0,
            y: 0,
            focused: false,
        }
    }

    /// Get the position of the `InterfaceItem`
    pub fn get_pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    /// Set the absolute position of the `InterfaceItem`
    pub fn set_pos(&mut self, pos: (u32, u32)) {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
    }

    /// Whether the `InterfaceItem` is focused or not (and should it receive inputs or not)
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Un/Focus the `InterfaceItem`
    pub fn set_focused(&mut self, focused: bool) {
        if focused != self.focused {
            self.dirty = true;
        }
        self.focused = focused;
    }
}

/// Represents a HashMap from VirtualKeyCode to character. Used to filter out which characters get registered by the textinput.
/// Use `Filter::empty_filter()` to create a new filter and for example `.with_basic_latin_characters` to add basic latin characters to the filter.
///
/// Use `.with_pair` or `insert` to create your own filters.
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
    /// Includes `'`, `\`, `:`, `.`, `;`, `,`, `=`, `-`, `*`, `_`, `/`, `[`, `]`
    ///
    /// Unfortunately VirtualKeyCode doesn't seem to support other special characters,
    /// using scancodes instead of VirtualKeyCodes is a planned feature though.
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
