use super::InterfaceItem;
use input::Input;
use text_buffer::TextBuffer;
use glutin::VirtualKeyCode;

/// Represents a list of InterfaceItems that is passed to the Menu when updating
///
/// MenuList is used to draw and handle updates in the Menu and will also determines the order of the InferfaceItems in the menu.
pub struct MenuList<'a> {
    pub(crate) items_ref: Vec<Box<&'a mut InterfaceItem>>,
    pub(crate) items: Vec<Box<InterfaceItem>>,
}

impl<'a> MenuList<'a> {
    /// Creates a new empty MenuList
    pub fn new() -> MenuList<'a> {
        MenuList {
            items_ref: Vec::new(),
            items: Vec::new(),
        }
    }

    /// Adds a InterfaceItem to the MenuList
    pub fn with_item<T: 'static + InterfaceItem + Clone>(mut self, item: &'a mut T) -> MenuList<'a> {
        self.items.push(Box::new(item.clone()));
        self.items_ref.push(Box::new(item));
        self
    }

    /// Adds a InterfaceItem to the MenuList
    pub fn add_item<T: 'static + InterfaceItem + Clone>(&mut self, item: &'a mut T) {
        self.items.push(Box::new(item.clone()));
        self.items_ref.push(Box::new(item));
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
    cloned_interface_items: Vec<Box<InterfaceItem>>,
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
            cloned_interface_items: Vec::new(),
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

    /// Set whether the menu is focused
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Update the menu, first handling any input if necessary, checking dirtyness,
    /// saving changes for later drawing and returning whether the menu should be redrawn or not.
    pub fn update(
        &mut self,
        input: &Input,
        children: MenuList,
    ) -> bool {
        if !self.focused {
            return false;
        }

        let items = children.items;
        let mut items_ref = children.items_ref;

        if input.was_just_pressed(VirtualKeyCode::Up) {
            self.select_idx = (((self.select_idx as i32 + items.len() as i32) - 1)
                % items.len() as i32) as u32;
            self.dirty = true;
        }
        if input.was_just_pressed(VirtualKeyCode::Down) {
            self.select_idx = (((self.select_idx as i32) + 1) % items.len() as i32) as u32;
            self.dirty = true;
        }
        let mut idx = 0;
        for item in &mut items_ref {
            item.set_focused(self.select_idx == idx);
            idx += 1;
        }
        if let Some(item) = (&mut items_ref).get_mut(self.select_idx as usize) {
            item.handle_input(input);
        }

        let dirty = self.is_dirty(&items);
        if dirty {
            self.cloned_interface_items = items;
        }
        dirty
    }

    /// Draw the menu and any saved children (see [`update(input, children)`](#method.update))
    pub fn draw(&mut self, text_buffer: &mut TextBuffer) {
        self.dirty = false;
        let mut h_off = 0;
        for item in &mut self.cloned_interface_items {
            item.set_pos((self.x, self.y + h_off));
            h_off += item.get_total_height();
            item.draw(text_buffer);
        }
    }

    fn is_dirty(&self, children: &Vec<Box<InterfaceItem>>) -> bool {
        let mut children_are_dirty = false; // No lewding the dragon loli
        for item in children {
            children_are_dirty = children_are_dirty || item.is_dirty()
        }
        self.dirty || children_are_dirty
    }
}