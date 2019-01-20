//! Menu systems is a module that allows easy creation and usage of [`Menu`](struct.Menu.html)s.
//! Examples of what a menu can be, is ncurses.
//!
//! Simply lists of [`InterfaceItem`](trait.InterfaceItem.html)s, that the user can browse through, press buttons,
//! input text, do whatever you want with GUI items generally. You can even make your own `InterfaceItem`s if you want.  
//! Selection in [`Menu`](struct.Menu.html)s works with keyboard and mouse, changeable with [`FocusSelection`](enum.FocusSelection.html).
//!
//! To add a [`TextProcessor`](../text_processing/struct.TextProcessor.html) to the menu, such as the Parser, use [`with_text_processor`](struct.Menu.html/#method.with_text_processor)
//!
//! If you wish to use [`InterfaceItem`](trait.InterfaceItem.html)s without the Menu struct, it is required to call `update` and then `draw` for them, in that order.
//!
//! Current pre-implemented items to use are
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
//! match TextBuffer::create(&terminal, (80, 24)) {
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
            self.base.set_pos(pos);
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
mod menu_switcher;
mod text_input;
mod text_item;
mod window;

pub use self::checkbox::{Checkbox, CheckboxGroup};
pub use self::dialog::Dialog;
pub use self::menu::{FocusSelection, GrowthDirection, Menu, MenuList, MenuPosition};
pub use self::menu_switcher::{MenuSelectionMethod, MenuSwitcher};
pub use self::text_input::TextInput;
pub use self::text_item::TextItem;
pub use self::window::Window;

use crate::events::Events;
use crate::text_buffer::TextBuffer;
use crate::text_processing::TextProcessor;

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
/// use glerminal::{with_base, Events, TextBuffer, TextStyle};
/// use glerminal::text_processing::TextProcessor;
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
///
///     with_base!(TextLabel);
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
///         text_buffer.cursor.style = TextStyle {
///             fg_color: [0.8, 0.8, 0.8, 1.0],
///             bg_color: [0.0, 0.0, 0.0, 0.0],
///             ..Default::default()
///         };
///         text_buffer.cursor.move_to(pos.0, pos.1);
///         text_buffer.write(self.text.clone());
///     }
///
///     fn handle_events(&mut self, _: &Events) -> bool {
///         false
///     }
///
///     // If you want TextProcessor support, ie. Parser support, implement the processing here.
///     // Also remember to not process every frame, but only when necessary.
///     fn update(&mut self, _: f32, processor: &TextProcessor) {}
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
    /// Update this InterfaceItem; delta is given in seconds. (see [Terminal.delta_time()](../terminal/struct.Terminal.html)).
    /// Also process any text that has changed since last update.
    fn update(&mut self, delta: f32, processor: &TextProcessor);
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
    fn clone_box(&self) -> Box<dyn InterfaceItem>;
}

impl<T: 'static + InterfaceItem + Clone> InterfaceItemClone for T {
    fn clone_box(&self) -> Box<dyn InterfaceItem> {
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

/// Represents a list of characters that is used to filter which character are registered in a [`TextInput`](struct.TextInput.html).
///
/// Use `Filter::empty_filter()` to create a new filter and for example `.with_basic_latin_characters` to add basic latin characters to the filter.  
/// Use `.with_char` or `add` to create your own filters, or `remove` to remove characters from pre-existing filters.
#[derive(Clone, Debug)]
pub struct Filter {
    chars: Vec<char>,
}

impl Filter {
    /// Create an empty filter, where other filters can be added, such as basic_latin_keycode_filter, or specific characters, with `with_char` or `add`
    pub fn empty_filter() -> Filter {
        Filter { chars: Vec::new() }
    }

    /// Creates a Filter with basic latin characters
    ///
    /// Includes characters `A-z` and spacebar (upper and lowercase).
    pub fn with_basic_latin_characters(mut self) -> Filter {
        let chars = "abcdefghijklmnopqrstuvwxyz";
        self.add_all(&chars.to_uppercase());
        self.add_all(chars);
        self.add(' ');
        self
    }

    /// Creates a Filter with basic numerals
    ///
    /// Includes numerals from `0-9`
    pub fn with_basic_numerals(mut self) -> Filter {
        self.add_all("0123456789");
        self
    }

    /// Creates a Filter with all special symbols from
    /// "basic latin", "latin-1 supplement" and "currency symbols" unicode blocks
    ///
    /// Includes the following characters:  
    /// From basic latin:  
    /// `!` `"` `#` `$` `%` `&` `'` `(` `)` `*` `+` `,` `-` `.` `/`  
    /// `@` `:` `;` `<` `=` `>` `?`  
    /// `` ` `` `[` `\` `]` `^` `_`   
    /// `{` `|` `}` `~`
    ///
    /// From latin-1 supplement:  
    /// `¡` `¢` `£` `¤` `¥` `¦` `§` `¨` `©` `ª` `«` `¬` `®` `¯`  
    /// `°` `±` `²` `³` `´` `µ` `¶` `·` `¸` `¹` `º` `»` `¼` `½` `¾` `¿`  
    /// `×` `÷`
    ///
    ///
    /// From currency symbols:  
    /// `₠` `₡` `₢` `₣` `₤` `₥` `₦` `₧` `₨` `₩` `₪` `₫` `€` `₭` `₮` `₯`  
    /// `₰` `₱` `₲` `₳` `₴` `₵` `₶` `₷` `₸` `₹` `₺` `₻` `₼` `₽` `₾` `₿`
    pub fn with_basic_special_symbols(mut self) -> Filter {
        // Basic latin -block
        self.add_all("!\"#$%&'()*+,-./");
        self.add_all("@:;<=>?");
        self.add_all("`[\\]^_");
        self.add_all("{|}~");

        // latin-1 supplement -block
        self.add_all("¡¢£¤¥¦§¨©ª«¬®¯");
        self.add_all("°±²³´µ¶·¸¹º»¼½¾¿");
        self.add_all("×÷");

        // Currency symbols -block
        self.add_all("₠₡₢₣₤₥₦₧₨₩₪₫€₭₮₯");
        self.add_all("₰₱₲₳₴₵₶₷₸₹₺₻₼₽₾₿");
        self
    }

    /// Creates a filter with all letter-like symbols from the latin-1 supplement unicode block
    ///
    /// Includes the following characters (upper and lowercase):  
    /// `à` `á` `â` `ã` `ä` `å` `æ`  
    /// `ç` `è` `é` `ê` `ë`  
    /// `ì` `í` `î` `ï`  
    /// `ð` `ñ` `ò` `ó` `ô` `õ` `ö` `ø`  
    /// `ù` `ú` `û` `ü` `ý` `þ` `ß` `ÿ`
    pub fn with_latin_1_supplement(mut self) -> Filter {
        let mut chars = String::new();
        chars += "àáâãäåæ";
        chars += "çèéêë";
        chars += "ìíîï";
        chars += "ðñòóôõöø";
        chars += "ùúûüýþßÿ";

        self.add_all(&chars.to_uppercase());
        self.add_all(&chars);
        self
    }

    /// Creates a filter with all letter-like symbols from the latin extended A unicode block
    ///
    /// Includes the following characters (upper and lowercase):  
    /// `ā` `ă` `ą`  
    /// `ć` `ĉ` `ċ` `č`  
    /// `ď` `đ` `ē` `ĕ` `ė` `ę` `ě`  
    /// `ĝ` `ğ` `ġ` `ģ` `ĥ` `ħ`  
    /// `ĩ` `ī` `ĭ` `į` `i` `̇ı`  
    /// `ĳ` `ĵ` `ķ` `ĸ`  
    /// `ĺ` `ļ` `ľ` `ŀ` `ł`  
    /// `ń` `ņ` `ň` `ŉ` `ŋ`  
    /// `ō` `ŏ` `ő` `œ`  
    /// `ŕ` `ŗ` `ř`  
    /// `ś` `ŝ` `ş` `š` `ţ` `ť` `ŧ`  
    /// `ũ` `ū` `ŭ` `ů` `ű` `ų`  
    /// `ŵ` `ŷ` `ÿ` `ź` `ż` `ž` `ſ`
    pub fn with_latin_extended_a(mut self) -> Filter {
        let mut chars = String::new();
        chars += "āăą";
        chars += "ćĉċč";
        chars += "ďđēĕėęě";
        chars += "ĝğġģĥħ";
        chars += "ĩīĭįi̇ı";
        chars += "ĳĵķĸ";
        chars += "ĺļľŀł";
        chars += "ńņňŉŋ";
        chars += "ōŏőœ";
        chars += "ŕŗř";
        chars += "śŝşšţťŧ";
        chars += "ũūŭůűų";
        chars += "ŵŷÿźżžſ";

        self.add_all(&chars.to_uppercase());
        self.add_all(&chars);
        self
    }

    /// Add a specific character to be accepted in this filter, and return the filter.
    pub fn with_char(mut self, character: char) -> Filter {
        self.add(character);
        self
    }

    /// Add a specific characters to be accepted in this filter, and return the filter.
    /// The function will go through each character in the string, and add them seperately.
    pub fn with_chars(mut self, characters: &str) -> Filter {
        self.add_all(characters);
        self
    }

    /// Insert a specific character to be accepted in this filter.
    ///
    /// Works similarly to with_pair
    ///
    /// Returns whether the character was added.
    pub fn add(&mut self, character: char) -> bool {
        if !self.has(character) {
            self.chars.push(character);
            true
        } else {
            false
        }
    }

    /// Insert a string of characters to be accepted in this filter.
    /// The function will go through each character in the string, and add them seperately.
    ///
    /// Works similarly to with_pair
    pub fn add_all(&mut self, characters: &str) {
        for character in characters.chars() {
            self.add(character);
        }
    }

    /// Remove the given character from the filter, if it exists.
    /// Returns whether the character was removed.
    pub fn remove(&mut self, character: char) -> bool {
        if !self.has(character) {
            false
        } else {
            let mut idx = 0;
            for c in self.chars.clone() {
                if c == character {
                    break;
                }
                idx += 1;
            }
            self.chars.remove(idx);
            true
        }
    }

    /// Remove all the characters in the given string of characters, if they exist in the filter.
    pub fn remove_all(&mut self, characters: &str) {
        for character in characters.chars() {
            self.remove(character);
        }
    }

    /// Whether the character specified exists in this filter or not.
    pub fn has(&self, character: char) -> bool {
        self.chars.contains(&character)
    }
}
