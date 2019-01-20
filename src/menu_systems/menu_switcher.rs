use crate::menu_systems::Menu;
use crate::{Events, VirtualKeyCode};

/// Te way that is used to select the menu when switching menus
///
/// Default is ByLocation(true, true)
pub enum MenuSelectionMethod {
    /// Select by the closest point where the selection were when the menu got switched.
    /// Including/Excluding x or y will mean that those distance values will be discarded when comparing distances.
    ///
    /// (include_x, include_y)
    ByLocation(bool, bool),
    /// Select by the current select_idx. This is only rarely what you want.
    ByIndex(),
}

impl Default for MenuSelectionMethod {
    fn default() -> MenuSelectionMethod {
        MenuSelectionMethod::ByLocation(true, true)
    }
}

/// A very neat tool designed to be used in making ie. button grids
///
/// Allows the navigation between menus. Very useful if you have multiple rows of menus.
pub struct MenuSwitcher {
    previous_selection_idx: u32,
    select_idx: u32,
    selection_method: MenuSelectionMethod,
    focused: bool,

    previous_button: VirtualKeyCode,
    next_button: VirtualKeyCode,
}

impl Default for MenuSwitcher {
    fn default() -> MenuSwitcher {
        MenuSwitcher {
            previous_selection_idx: 0,
            select_idx: 0,
            selection_method: Default::default(),
            focused: true,

            previous_button: VirtualKeyCode::Left,
            next_button: VirtualKeyCode::Right,
        }
    }
}

impl MenuSwitcher {
    /// Create a new MenuSwitcher
    pub fn new(previous: VirtualKeyCode, next: VirtualKeyCode) -> MenuSwitcher {
        MenuSwitcher {
            previous_selection_idx: 0,
            select_idx: 0,
            selection_method: Default::default(),
            focused: true,

            previous_button: previous,
            next_button: next,
        }
    }

    /// Change the [`MenuSelectionMethod`](struct.MenuSelectionMethod.html). Default is ByLocation(true, true)
    pub fn with_selection_method(mut self, selection_method: MenuSelectionMethod) -> MenuSwitcher {
        self.selection_method = selection_method;
        self
    }

    /// Change whether this switcher is focused, meaning if any of it's menus should be focused.
    ///
    /// Default is true.
    pub fn with_focus(mut self, focused: bool) -> MenuSwitcher {
        self.focused = focused;
        self
    }

    /// Change the [`MenuSelectionMethod`](struct.MenuSelectionMethod.html). Default is ByLocation(true, true)
    pub fn set_selection_method(&mut self, selection_method: MenuSelectionMethod) {
        self.selection_method = selection_method;
    }

    /// Change whether this switcher is focused, meaning if any of it's menus should be focused.
    ///
    /// Default is true.
    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Set the index of the Menu that is selected. At update, the idx will clamp between 0 and the max idx.
    pub fn set_select_idx(&mut self, idx: u32) {
        self.previous_selection_idx = self.select_idx;
        self.select_idx = idx;
    }

    /// Update the menu switcher, change select indexes if necessary and handle events.
    pub fn update(&mut self, events: &Events, list: &mut [&mut Menu]) {
        let length = list.len() as u32;

        // Nothing to update
        if length == 0 {
            self.select_idx = 0;
            return;
        }

        self.select_idx = (self.select_idx as i32).min(length as i32 - 1).max(0) as u32;

        if self.focused {
            if events.keyboard.was_just_pressed(self.previous_button) {
                self.previous_selection_idx = self.select_idx;
                self.select_idx = (self.select_idx + length - 1) % length;
            }
            if events.keyboard.was_just_pressed(self.next_button) {
                self.previous_selection_idx = self.select_idx;
                self.select_idx = (self.select_idx + 1) % length;
            }
        }

        for (idx, menu) in list.iter_mut().enumerate() {
            menu.set_focused(idx as u32 == self.select_idx && self.focused);
        }

        if self.previous_selection_idx != self.select_idx {
            let p_select_idx = list[self.previous_selection_idx as usize].get_select_idx();

            let prev_menu = &list[self.previous_selection_idx as usize];
            let curr_pos = prev_menu.get_cloned_list()[prev_menu.get_select_idx() as usize]
                .get_base()
                .get_pos();

            match self.selection_method {
                MenuSelectionMethod::ByLocation(include_x, include_y) => {
                    let mut closest_idx = 0;
                    let mut closest_distance = 100_000;
                    for (idx, item) in list[self.select_idx as usize]
                        .get_cloned_list()
                        .into_iter()
                        .enumerate()
                    {
                        let distance_x = if include_x {
                            (curr_pos.0 as i32 - item.get_base().get_pos().0 as i32).abs() as u32
                        } else {
                            0
                        };
                        let distance_y = if include_y {
                            (curr_pos.1 as i32 - item.get_base().get_pos().1 as i32).abs() as u32
                        } else {
                            0
                        };
                        let distance = distance_x + distance_y;
                        if distance < closest_distance {
                            closest_idx = idx as u32;
                            closest_distance = distance;
                        }
                    }
                    list[self.select_idx as usize].set_select_idx(closest_idx);
                }
                MenuSelectionMethod::ByIndex() => {
                    list[self.select_idx as usize].set_select_idx(p_select_idx);
                }
            }
        }
        self.previous_selection_idx = self.select_idx;
    }
}
