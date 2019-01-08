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
#[derive(Clone, Copy)]
pub enum GrowthDirection {
    /// Expand it downwards
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

/// Represents the way in which focus is selected in the menu
pub enum FocusSelection {
    /// With keyboard (previous key, next key)
    Keyboard(Option<VirtualKeyCode>, Option<VirtualKeyCode>),
    /// With mouse (point which item should be focused)
    Mouse(),
    /// With mouse or keyboard (see mouse and keyboard individually)
    MouseAndKeyboard(Option<VirtualKeyCode>, Option<VirtualKeyCode>),
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
    focus_selection: FocusSelection,
}

impl Menu {
    /// Initializes a new empty menu
    pub fn new() -> Menu {
        Menu {
            x: 0,
            y: 0,
            focused: false,
            is_dirty: true,
            select_idx: 0,
            total_width: 0,
            total_height: 0,
            cloned_interface_items: Vec::new(),

            growth_direction: GrowthDirection::Down,
            focus_selection: FocusSelection::Keyboard(None, None),
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

    /// Sets the way the menu is browsed
    pub fn with_focus_selection(mut self, focus_selection: FocusSelection) -> Menu {
        self.focus_selection = focus_selection;
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

    /// Sets the way the menu is browsed
    pub fn set_focus_selection(&mut self, focus_selection: FocusSelection) {
        self.focus_selection = focus_selection;
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

    /// Return the current item that is selected.
    pub fn get_select_idx(&self) -> u32 {
        self.select_idx
    }

    /// Returns the button that must be pressed in order to select the previous menu item.
    pub fn get_previous_button(&self) -> VirtualKeyCode {
        let previous_button;
        match self.focus_selection {
            FocusSelection::Keyboard(previous, _) => previous_button = previous,
            FocusSelection::Mouse() => previous_button = None,
            FocusSelection::MouseAndKeyboard(previous, _) => previous_button = previous,
        }
        if let Some(button) = previous_button {
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
        let next_button;
        match self.focus_selection {
            FocusSelection::Keyboard(_, next) => next_button = next,
            FocusSelection::Mouse() => next_button = None,
            FocusSelection::MouseAndKeyboard(_, next) => next_button = next,
        }
        if let Some(button) = next_button {
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
    pub fn update(
        &mut self,
        events: &Events,
        delta: f32,
        text_buffer: &TextBuffer,
        list: &mut MenuList,
    ) -> bool {
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
            let keyboard_focus = match self.focus_selection {
                FocusSelection::Keyboard(..) => true,
                FocusSelection::MouseAndKeyboard(..) => true,
                _ => false,
            };
            if keyboard_focus {
                if events.keyboard.was_just_pressed(self.get_previous_button()) {
                    self.select_idx =
                        (((self.select_idx as i32 + length as i32) - 1) % length as i32) as u32;

                    let start_idx = self.select_idx.min(length as u32 - 1).max(0);
                    while {
                        !list
                            .items_ref
                            .get(self.select_idx as usize)
                            .unwrap()
                            .get_base()
                            .can_be_focused
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

            // Do any selection with mouse
            let mouse_focus = match self.focus_selection {
                FocusSelection::Mouse() => true,
                FocusSelection::MouseAndKeyboard(..) => true,
                _ => false,
            };

            if mouse_focus {
                let grow_right = match self.growth_direction {
                    GrowthDirection::Left => false,
                    _ => true,
                };
                let grow_down = match self.growth_direction {
                    GrowthDirection::Up => false,
                    _ => true,
                };
                if let Some(loc) = events.cursor.get_location(&text_buffer) {
                    for idx in 0..self.cloned_interface_items.len() {
                        let item = self.cloned_interface_items.get(idx).unwrap();
                        let base = item.get_base();
                        let idx = idx as u32;

                        if !base.can_be_focused {
                            continue;
                        }
                        let (x, y) = (base.get_pos().0 as i32, base.get_pos().1 as i32);
                        let width = item.get_total_width() as i32;
                        let height = item.get_total_height() as i32;

                        let (x0, x1);
                        if grow_right {
                            x0 = x;
                            x1 = x + width - 1;
                        } else {
                            x0 = x - width + 1;
                            x1 = x;
                        }

                        let (y0, y1);
                        if grow_down {
                            y0 = y;
                            y1 = y + height - 1;
                        } else {
                            y0 = y - height + 1;
                            y1 = y;
                        }

                        if loc.0 >= x0 && loc.0 <= x1 && loc.1 >= y0 && loc.1 <= y1 {
                            self.select_idx = idx;
                            break;
                        }
                    }
                }
            }
        }

        // Ensure that any unselectable menu items aren't selected. If none are found, c'est la vie
        let start_idx = self.select_idx.min(length as u32 - 1).max(0);
        while {
            !list
                .items_ref
                .get(self.select_idx as usize)
                .unwrap()
                .get_base()
                .can_be_focused
        } {
            self.select_idx = (((self.select_idx as i32) + 1) % length as i32) as u32;
            if self.select_idx == start_idx {
                break;
            }
        }

        // Update children and focus the focused child.
        let mut idx = 0;
        for item in &mut list.items_ref {
            item.get_mut_base().set_focused(self.select_idx == idx);
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
                        item.get_mut_base()
                            .set_pos((last_pos.0 as u32, last_pos.1 as u32));

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
                        item.get_mut_base()
                            .set_pos((last_pos.0 as u32, last_pos.1 as u32));

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
                        item.get_mut_base()
                            .set_pos((last_pos.0 as u32, last_pos.1 as u32));

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
                        item.get_mut_base()
                            .set_pos((last_pos.0 as u32, last_pos.1 as u32));

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
            let mut base = item.get_mut_base();
            children_are_dirty = children_are_dirty || base.dirty;
            base.dirty = false;
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
