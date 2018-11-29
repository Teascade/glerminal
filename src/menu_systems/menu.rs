use super::InterfaceItem;
use events::Events;
use glutin::VirtualKeyCode;
use text_buffer::TextBuffer;

/// Represents a list of InterfaceItems that is passed to the Menu when updating
///
/// MenuList is used to draw and handle updates in the Menu and will also determines the order of the InferfaceItems in the menu.
pub struct MenuList<'a> {
    items_ref: Vec<Box<&'a mut InterfaceItem>>,
    positions: Vec<MenuPosition>,
}

impl<'a> MenuList<'a> {
    /// Creates a new empty MenuList
    pub fn new() -> MenuList<'a> {
        MenuList {
            items_ref: Vec::new(),
            positions: Vec::new(),
        }
    }

    /// Adds a InterfaceItem to the MenuList
    pub fn with_item<T: 'static + InterfaceItem + Clone, U: Into<Option<MenuPosition>>>(
        mut self,
        item: &'a mut T,
        position: U,
    ) -> MenuList<'a> {
        self.items_ref.push(Box::new(item));
        if let Some(position) = position.into() {
            self.positions.push(position);
        } else {
            self.positions.push(MenuPosition::RelativeToLast(0, 0));
        }
        self
    }

    /// Adds a InterfaceItem to the MenuList
    pub fn add_item<T: 'static + InterfaceItem + Clone, U: Into<Option<MenuPosition>>>(
        &mut self,
        item: &'a mut T,
        position: U,
    ) {
        self.items_ref.push(Box::new(item));
        if let Some(position) = position.into() {
            self.positions.push(position);
        } else {
            self.positions.push(MenuPosition::RelativeToLast(0, 0));
        }
    }

    pub(crate) fn get_cloned_list(&self) -> Vec<Box<InterfaceItem>> {
        let mut list = Vec::new();
        for item in &self.items_ref {
            list.push(item.clone_box());
        }
        list
    }
}

/// Determines the direction where the Menu will expand/grow from it's position
pub enum GrowthDirection {
    /// Expand it lownwards
    Down,
    /// Expand it upwards
    Up,
    /// Expand it rightwards
    Right,
    /// Expand it leftwards
    Left,
}

/// Represents the position of a menu item in the menu
pub enum MenuPosition {
    /// Relative to the menu's default positioning. (Relative(0, 0) = default position)
    Relative(i32, i32),
    /// (Default) Relative to the last item in the menu (RelativeToLast(0, 0) = next to last item)
    RelativeToLast(i32, i32),
    /// Absolute position for the menu item (Absolute(0, 0) = top-left corner of the screen)
    Absolute(u32, u32),
}

/// Represents a Menu
pub struct Menu {
    x: u32,
    y: u32,
    focused: bool,
    is_dirty: bool,
    select_idx: u32,
    total_width: u32,
    total_height: u32,
    cloned_interface_items: Vec<Box<InterfaceItem>>,

    growth_direction: GrowthDirection,
    previous_button: Option<VirtualKeyCode>,
    next_button: Option<VirtualKeyCode>,
}

impl Menu {
    /// Initializes a new empty menu
    pub fn new() -> Menu {
        Menu {
            x: 0,
            y: 0,
            focused: false,
            is_dirty: false,
            select_idx: 0,
            total_width: 0,
            total_height: 0,
            cloned_interface_items: Vec::new(),

            growth_direction: GrowthDirection::Down,
            previous_button: None,
            next_button: None,
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

    /// Sets the initial growth direction of the Menu
    pub fn with_growth_direction(mut self, growth_direction: GrowthDirection) -> Menu {
        self.growth_direction = growth_direction;
        self
    }

    /// Sets the initial previous button (selects previous item in the menu) for the Menu
    pub fn with_previous_button<T: Into<Option<VirtualKeyCode>>>(mut self, button: T) -> Menu {
        self.previous_button = button.into();
        self
    }

    /// Sets the initial next button (selects next item in the menu) for the Menu
    pub fn with_next_button<T: Into<Option<VirtualKeyCode>>>(mut self, button: T) -> Menu {
        self.next_button = button.into();
        self
    }

    /// Sets the position of the menu
    pub fn set_pos(&mut self, pos: (u32, u32)) {
        let (x, y) = pos;
        self.x = x;
        self.y = y;
    }

    /// Set whether the menu is focused
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Sets the growth direction of the Menu
    pub fn set_growth_direction(&mut self, growth_direction: GrowthDirection) {
        self.growth_direction = growth_direction;
    }

    /// Sets the previous button (selects previous item in the menu) for the Menu
    pub fn set_previous_button<T: Into<Option<VirtualKeyCode>>>(mut self, button: T) {
        self.previous_button = button.into();
    }

    /// Sets the next button (selects next item in the menu) for the Menu
    pub fn set_next_button<T: Into<Option<VirtualKeyCode>>>(mut self, button: T) {
        self.next_button = button.into();
    }

    /// Get the position of the Menu
    pub fn get_pos(&self) -> (u32, u32) {
        (self.x, self.y)
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

    /// Returns the button that must be pressed in order to select the previous menu item.
    pub fn get_previous_button(&self) -> VirtualKeyCode {
        if let Some(button) = self.previous_button {
            button
        } else {
            match self.growth_direction {
                GrowthDirection::Up => VirtualKeyCode::Down,
                GrowthDirection::Down => VirtualKeyCode::Up,
                GrowthDirection::Left => VirtualKeyCode::Right,
                GrowthDirection::Right => VirtualKeyCode::Left,
            }
        }
    }

    /// Returns the button that must be pressed in order to select the next menu item.
    pub fn get_next_button(&self) -> VirtualKeyCode {
        if let Some(button) = self.next_button {
            button
        } else {
            match self.growth_direction {
                GrowthDirection::Up => VirtualKeyCode::Up,
                GrowthDirection::Down => VirtualKeyCode::Down,
                GrowthDirection::Left => VirtualKeyCode::Left,
                GrowthDirection::Right => VirtualKeyCode::Right,
            }
        }
    }

    /// Update the menu, first handling any events if necessary, checking dirtyness,
    /// saving changes for later drawing and returning whether the menu should be redrawn or not.
    pub fn update(&mut self, events: &Events, delta: f32, list: &mut MenuList) -> bool {
        if !self.focused {
            return false;
        }

        // Handle input for focused child and consume input if necessary.
        let mut focused_handled_input = false;
        if let Some(item) = (&mut list.items_ref).get_mut(self.select_idx as usize) {
            focused_handled_input = item.handle_events(events);
        }

        let length = list.items_ref.len();

        // Handle input for the menu (selecting), if focused child didn't consume the last inpout
        if !focused_handled_input {
            if events.keyboard.was_just_pressed(self.get_previous_button()) {
                self.select_idx =
                    (((self.select_idx as i32 + length as i32) - 1) % length as i32) as u32;

                let start_idx = self.select_idx.min(length as u32 - 1).max(0);
                while {
                    !list
                        .items_ref
                        .get(self.select_idx as usize)
                        .unwrap()
                        .can_be_focused()
                } {
                    self.select_idx =
                        (((self.select_idx as i32 + length as i32) - 1) % length as i32) as u32;
                    if self.select_idx == start_idx {
                        break;
                    }
                }
            }
            if events.keyboard.was_just_pressed(self.get_next_button()) {
                self.select_idx = (((self.select_idx as i32) + 1) % length as i32) as u32;
            }
        }

        // Ensure that any unselectable menu items aren't selected. If none are found, c'est la vie
        let start_idx = self.select_idx.min(length as u32 - 1).max(0);
        while {
            !list
                .items_ref
                .get(self.select_idx as usize)
                .unwrap()
                .can_be_focused()
        } {
            self.select_idx = (((self.select_idx as i32) + 1) % length as i32) as u32;
            if self.select_idx == start_idx {
                break;
            }
        }

        // Update children and focus the focused child.
        let mut idx = 0;
        for item in &mut list.items_ref {
            item.set_focused(self.select_idx == idx);
            item.update(delta);
            idx += 1;
        }

        // Check if the children are dirty, if they are then update them to be drawn
        self.is_dirty = self.children_are_dirty(&mut list.items_ref);
        if self.is_dirty {
            self.cloned_interface_items = list.get_cloned_list();

            let mut off: (i32, i32) = (0, 0);
            let mut last_off: (i32, i32) = (0, 0);
            let mut last_pos: (i32, i32) = (self.x as i32, self.y as i32);

            // Set the positions of the children relative to the growth direction and their own positions
            match self.growth_direction {
                GrowthDirection::Down => {
                    for (idx, item) in (&mut self.cloned_interface_items).iter_mut().enumerate() {
                        let position = list.positions.get(idx).unwrap();

                        last_pos = Menu::calc_new_pos(
                            self.x as i32,
                            self.y as i32,
                            position,
                            off,
                            last_off,
                            last_pos,
                        );
                        item.set_pos((last_pos.0 as u32, last_pos.1 as u32));

                        last_off = (0, item.get_total_height() as i32);
                        off.1 += last_off.1;
                    }
                }
                GrowthDirection::Up => {
                    for (idx, item) in (&mut self.cloned_interface_items).iter_mut().enumerate() {
                        let position = list.positions.get(idx).unwrap();

                        last_off = (0, -(item.get_total_height() as i32));

                        last_pos = Menu::calc_new_pos(
                            self.x as i32,
                            self.y as i32,
                            position,
                            off,
                            last_off,
                            last_pos,
                        );
                        item.set_pos((last_pos.0 as u32, last_pos.1 as u32));

                        off.1 += last_off.1;
                    }
                }
                GrowthDirection::Right => {
                    for (idx, item) in (&mut self.cloned_interface_items).iter_mut().enumerate() {
                        let position = list.positions.get(idx).unwrap();

                        last_pos = Menu::calc_new_pos(
                            self.x as i32,
                            self.y as i32,
                            position,
                            off,
                            last_off,
                            last_pos,
                        );
                        item.set_pos((last_pos.0 as u32, last_pos.1 as u32));

                        last_off = (item.get_total_width() as i32, 0);
                        off.0 += last_off.0;
                    }
                }
                GrowthDirection::Left => {
                    for (idx, item) in (&mut self.cloned_interface_items).iter_mut().enumerate() {
                        let position = list.positions.get(idx).unwrap();

                        last_off = (-(item.get_total_width() as i32), 0);

                        last_pos = Menu::calc_new_pos(
                            self.x as i32,
                            self.y as i32,
                            position,
                            off,
                            last_off,
                            last_pos,
                        );
                        item.set_pos((last_pos.0 as u32, last_pos.1 as u32));

                        off.0 += last_off.0;
                    }
                }
            }
        }
        self.is_dirty
    }

    /// Draw the menu and any saved children (see [`update(input, children)`](#method.update))
    pub fn draw(&mut self, text_buffer: &mut TextBuffer) {
        for item in &mut self.cloned_interface_items {
            item.draw(text_buffer);
        }
    }

    /// Check if any of the given children are dirty; meaning they should be redrawn
    fn children_are_dirty(&self, children: &mut Vec<Box<&mut InterfaceItem>>) -> bool {
        let mut children_are_dirty = false; // No lewding the dragon loli
        for item in children {
            children_are_dirty = children_are_dirty || item.is_dirty();
            item.set_dirty(false);
        }
        children_are_dirty
    }

    /// Gets the correct position of the menu-item relative to the
    /// 1. Growth direction of the menu
    /// 2. Position of the menu item itself
    /// 3. Other offset data related to the menu growth.
    fn calc_new_pos(
        root_x: i32,
        root_y: i32,
        position: &MenuPosition,
        off: (i32, i32),
        last_off: (i32, i32),
        last_pos: (i32, i32),
    ) -> (i32, i32) {
        match position {
            &MenuPosition::Relative(x, y) => (root_x + x + off.0, root_y + y + off.1),
            &MenuPosition::RelativeToLast(x, y) => {
                (last_pos.0 + x + last_off.0, last_pos.1 + y + last_off.1)
            }
            &MenuPosition::Absolute(x, y) => (x as i32, y as i32),
        }
    }
}
