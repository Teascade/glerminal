use std::iter::repeat;

use super::{InterfaceItem, InterfaceItemBase};
use crate::events::Events;
use crate::text_buffer::{Color, TextBuffer};
use crate::{MouseButton, VirtualKeyCode};

/// Represents a group of checkboxes that can be managed like they were radio buttons.
///
/// This allows you to make a CheckboxGroup of ie. 3 checkboxes and only one of them will be selected at a time,
/// unless force one selected is toggled on, in which case the index given with it will be selected at first, and the group will disallow having none selected.
///
/// Updating this CheckboxGroup with the given Checkboxes will ensure that the functionality explained above will be enforced.
#[derive(Default)]
pub struct CheckboxGroup {
    selected_idx: Option<u32>,
    force_one_checked: Option<u32>,
}

impl CheckboxGroup {
    /// Creates a new CheckboxGroup with no selection.
    pub fn new() -> CheckboxGroup {
        CheckboxGroup {
            selected_idx: None,
            force_one_checked: None,
        }
    }

    /// Some forces something to be always checked, None (default) allows everything to be unchecked.
    /// Put an index that should be checked as default, if given index doesn't exist 0 is used.
    /// Selected index will result to None only if no checkboxes are given.
    pub fn with_force_one_checked(mut self, force_one_checked: Option<u32>) -> CheckboxGroup {
        self.force_one_checked = force_one_checked;
        self
    }

    /// Update this CheckboxGroup with the given Checkboxes, this will ensure that only one of the given Checkboxes will remain checked.
    pub fn update(&mut self, checkboxes: &mut [&mut Checkbox]) {
        let mut selection_changed = false;
        let mut any_selected = false;
        for idx in 0..checkboxes.len() {
            let curr_is_checked;
            if let Some(checkbox) = checkboxes.get(idx) {
                curr_is_checked = checkbox.is_checked();
            } else {
                curr_is_checked = false;
            }

            if curr_is_checked {
                any_selected = true;
            }

            if let Some(selected_idx) = self.selected_idx {
                if curr_is_checked && idx as u32 != selected_idx {
                    if !selection_changed {
                        if let Some(checkbox) = checkboxes.get_mut(selected_idx as usize) {
                            checkbox.set_checked(false);
                        }
                        self.selected_idx = Some(idx as u32);
                        selection_changed = true;
                    } else if let Some(checkbox) = checkboxes.get_mut(idx) {
                        checkbox.set_checked(false);
                    }
                }
            } else if curr_is_checked {
                self.selected_idx = Some(idx as u32);
                selection_changed = true;
            }
        }
        if !any_selected {
            if let Some(forced_idx) = self.force_one_checked {
                if let Some(idx) = self.selected_idx {
                    if let Some(checkbox) = checkboxes.get_mut(idx as usize) {
                        checkbox.set_checked(true);
                    }
                } else {
                    if checkboxes.len() > forced_idx as usize {
                        let checkbox = &mut checkboxes[forced_idx as usize];
                        checkbox.set_checked(true);
                        self.selected_idx = Some(forced_idx);
                    } else if let Some(checkbox) = checkboxes.get_mut(0) {
                        checkbox.set_checked(true);
                        self.selected_idx = Some(0);
                    }
                    self.selected_idx = None;
                }
            } else {
                self.selected_idx = None;
            }
        }
    }

    /// Return the index that is currently selected, if any.
    pub fn get_selection_idx(&self) -> Option<u32> {
        self.selected_idx
    }
}

/// Represents a Checkbox that can be checked or unchecked, and it's checked-status can be get with `is_checked`.
///
/// See [CheckboxGroup](struct.CheckboxGroup.html) to limit how checkboxes can be checked together.
///
/// For example:
/// ```
/// use glerminal::menu_systems::Checkbox;
///
/// Checkbox::new("Generate world: ")
///     .with_prefix("<")
///     .with_suffix(">")
///     .with_checked_text("O");
///
/// // Creates a checbox that looks unchecked:
/// // Generate world: < >
/// // And checked:
/// // Generate world: <O>
/// ```
#[derive(Debug, Clone)]
pub struct Checkbox {
    /// Foreground color for when the checkbox is not focused
    pub fg_color_unfocused: Color,
    /// Background color for when the checkbox is not focused
    pub bg_color_unfocused: Color,
    /// Foreground color for when the checkbox is focused
    pub fg_color_focused: Color,
    /// Background color for when the checkbox is focused
    pub bg_color_focused: Color,

    /// The keyboard inputs that trigger `was_just_pressed`
    pub button_press_inputs: Vec<VirtualKeyCode>,
    /// The mouse inputs that trigger `was_just_pressed`
    pub mouse_button_press_inputs: Vec<MouseButton>,

    base: InterfaceItemBase,

    text: String,
    prefix: String,
    suffix: String,
    checked_text: String,

    checked: bool,
    was_just_pressed: bool,
}

impl Checkbox {
    /// Intiailizes a Checkbox with the given text and max width
    pub fn new<T: Into<String>>(text: T) -> Checkbox {
        Checkbox {
            bg_color_unfocused: [0.0, 0.0, 0.0, 0.0],
            fg_color_unfocused: [0.8, 0.8, 0.8, 1.0],
            bg_color_focused: [0.8, 0.8, 0.8, 1.0],
            fg_color_focused: [0.2, 0.2, 0.2, 1.0],

            base: InterfaceItemBase::new(true),

            text: text.into(),
            prefix: "[".to_owned(),
            suffix: "]".to_owned(),
            checked_text: "X".to_owned(),

            checked: false,
            was_just_pressed: false,
            button_press_inputs: vec![VirtualKeyCode::Return],
            mouse_button_press_inputs: Vec::new(),
        }
    }

    with_base!(Checkbox);
    with_set_pressable!(Checkbox);
    with_set_colors!(Checkbox);

    /// Sets the initial text of the Checkbox
    pub fn with_text<T: Into<String>>(mut self, text: T) -> Checkbox {
        self.text = text.into();
        self
    }

    /// Sets the initial prefix of the Checkbox
    pub fn with_prefix<T: Into<String>>(mut self, prefix: T) -> Checkbox {
        self.prefix = prefix.into();
        self
    }

    /// Sets the initial suffix of the Checkbox
    pub fn with_suffix<T: Into<String>>(mut self, suffix: T) -> Checkbox {
        self.suffix = suffix.into();
        self
    }

    /// Sets the initial checked-text (text shown in between prefix and suffix) of the Checkbox
    pub fn with_checked_text<T: Into<String>>(mut self, checked_text: T) -> Checkbox {
        self.checked_text = checked_text.into();
        self
    }

    /// Set whether the checkbox is initially checked or not
    pub fn with_checked(mut self, checked: bool) -> Checkbox {
        self.checked = checked;
        self
    }

    /// Sets the text of the Checkbox
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
        self.base.dirty = true;
    }

    /// Sets the prefix of the Checkbox
    pub fn set_prefix<T: Into<String>>(&mut self, prefix: T) {
        self.prefix = prefix.into();
        self.base.dirty = true;
    }

    /// Sets the suffix of the Checkbox
    pub fn set_suffix<T: Into<String>>(&mut self, suffix: T) {
        self.suffix = suffix.into();
        self.base.dirty = true;
    }

    /// Sets the checked-text (text shown in between prefix and suffix) of the Checkbox
    pub fn set_checked_text<T: Into<String>>(mut self, checked_text: T) {
        self.checked_text = checked_text.into();
    }

    /// Return the current text of the Checkbox
    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    /// Return the current prefix of the Checkbox
    pub fn get_prefix(&self) -> String {
        self.prefix.clone()
    }

    /// Return the current suffix of the Checkbox
    pub fn get_suffix(&self) -> String {
        self.suffix.clone()
    }

    /// Sets the checked-status for this checkbox.
    pub fn set_checked(&mut self, checked: bool) {
        if self.checked != checked {
            self.base.dirty = true;
        }
        self.checked = checked;
    }

    /// Returns whether this checkbox is checked.
    pub fn is_checked(&self) -> bool {
        self.checked
    }
}

impl InterfaceItem for Checkbox {
    fn get_base(&self) -> &InterfaceItemBase {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut InterfaceItemBase {
        &mut self.base
    }

    fn get_total_width(&self) -> u32 {
        (self.text.len() + self.prefix.len() + self.checked_text.len() + self.suffix.len()) as u32
    }

    fn get_total_height(&self) -> u32 {
        1
    }

    fn draw(&mut self, text_buffer: &mut TextBuffer) {
        self.base.dirty = false;
        if self.base.is_focused() {
            text_buffer.change_cursor_fg_color(self.fg_color_focused);
            text_buffer.change_cursor_bg_color(self.bg_color_focused);
        } else {
            text_buffer.change_cursor_fg_color(self.fg_color_unfocused);
            text_buffer.change_cursor_bg_color(self.bg_color_unfocused);
        }
        text_buffer.move_cursor(self.base.x as i32, self.base.y as i32);
        let checked_text = if self.checked {
            (&self.checked_text).to_owned()
        } else {
            repeat(" ").take(self.checked_text.len()).collect()
        };
        let text = (&self.text).to_owned() + &self.prefix + &checked_text + &self.suffix;
        text_buffer.write(text);
    }

    fn handle_events(&mut self, events: &Events) -> bool {
        self.was_just_pressed = false;
        for curr in &self.button_press_inputs {
            if events.keyboard.was_just_pressed(*curr) {
                self.was_just_pressed = true;
                self.checked = !self.checked;
                self.base.dirty = true;
                return true;
            }
        }
        for curr in &self.mouse_button_press_inputs {
            if events.mouse.was_just_pressed(*curr) {
                self.was_just_pressed = true;
                self.checked = !self.checked;
                self.base.dirty = true;
                return true;
            }
        }
        false
    }

    fn update(&mut self, _: f32) {}
}
