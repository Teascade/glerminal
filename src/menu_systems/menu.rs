use std::ops::{Deref, DerefMut};

use super::InterfaceItem;
use input::Input;
use text_buffer::TextBuffer;
use glutin::VirtualKeyCode;

/// Represents an InterfaceItem, that can be added in a menu
pub struct MenuItem<T: InterfaceItem> {
    pub(crate) item: T,
}

impl<T: 'static + InterfaceItem> MenuItem<T> {
    /// Creates a new menu item from the given InterfaceItem
    pub fn new(item: T) -> MenuItem<T> {
        MenuItem {
            item
        }
    }
}

impl<T: 'static + InterfaceItem> Deref for MenuItem<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<T: 'static + InterfaceItem> DerefMut for MenuItem<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.item
    }
}

/// Represents a Menu
pub struct Menu {
    x: u32,
    y: u32,
    focused: bool,
    dirty: bool,
    select_idx: u32,
    total_width: u32,
    total_height: u32,
}

impl Menu {
    /// Initializes a new empty menu
    pub fn new() -> Menu {
        Menu {
            x: 0,
            y: 0,
            focused: false,
            dirty: true,
            select_idx: 0,
            total_width: 0,
            total_height: 0,
        }
    }

    /// Sets the position and consumes the Menu, then returns it
    pub fn with_pos(mut self, position: (u32, u32)) -> Menu {
        let (x, y) = position;
        self.x = x;
        self.y = y;
        self
    }

    /// Sets weather the TextInput is focused.
    pub fn with_focus(mut self, focused: bool) -> Menu {
        self.focused = focused;
        self
    }

    /*
        /// Get a reference of the box of the currently selected item, if such exists.
        pub fn get_current_focused_item(&self) -> Option<&Box<InterfaceItem>> {
            if !self.focused || self.interface_items.len() == 0 {
                None
            } else {
                self.interface_items.get(self.select_idx as usize)
            }
        }
    */

    /// Get the position of the Menu
    pub fn get_pos(&self) -> (u32, u32) {
        (self.x, self.y)
    }

    /// Sets the position of the menu
    pub fn set_pos(&mut self, pos: (u32, u32)) {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
    }

    /// Gets the width this Menu should take up when drawn
    pub fn get_total_width(&self) -> u32 {
        self.total_width
    }

    /// Get the height this Menu should take up when drawn
    pub fn get_total_height(&self) -> u32 {
        self.total_height
    }

    /// Is the menu currently focused and is it receiving input.
    /// If the menu is not focused, selection will point to a non-existing item (-1)
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Set weather the menu is focused
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Is the menu dirty and should it be redrawn
    pub fn is_dirty<T: 'static + InterfaceItem>(&self, children: &[&MenuItem<T>]) -> bool {
        let mut children_are_dirty = false; // No lewding the dragon loli
        for item in children {
            children_are_dirty = children_are_dirty || item.is_dirty();
        }
        self.dirty || children_are_dirty
    }

    /// Draw the menu
    pub fn draw<T: 'static + InterfaceItem>(&mut self, text_buffer: &mut TextBuffer, children: &mut [&mut MenuItem<T>]) {
        self.dirty = false;
        let mut h_off = 0;
        let mut idx = 0;
        for item in children {
            item.set_focused(self.select_idx == idx);
            item.set_pos((self.x, self.y + h_off));
            h_off += item.get_total_height();
            item.draw(text_buffer);
            idx += 1;
        }
    }

    /// Handle input for this menu and children if required
    pub fn handle_input<T: 'static + InterfaceItem>(
        &mut self,
        input: &Input,
        children: &mut [&mut MenuItem<T>],
    ) -> bool {
        if !self.focused {
            return false;
        }
        let mut catched_input = false;
        if input.was_just_pressed(VirtualKeyCode::Up) {
            self.select_idx = (((self.select_idx as i32 + children.len() as i32) - 1)
                % children.len() as i32) as u32;
            catched_input = true;
            self.dirty = true;
        }
        if input.was_just_pressed(VirtualKeyCode::Down) {
            self.select_idx = (((self.select_idx as i32) + 1) % children.len() as i32) as u32;
            catched_input = true;
            self.dirty = true;
        }
        if let Some(item) = children.get_mut(self.select_idx as usize) {
            return item.handle_input(input) || catched_input;
        }
        catched_input
    }
}