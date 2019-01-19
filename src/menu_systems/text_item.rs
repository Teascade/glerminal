use super::{InterfaceItem, InterfaceItemBase};
use crate::text_processing::{ProcessedChar, TextProcessor};
use crate::{Color, Events, MouseButton, TextBuffer, TextStyle, VirtualKeyCode};

#[derive(Debug, Clone)]
/// Represents a simple text item that by default can not be selected,
/// but optionally can be selected and pressed like a button.
///
/// Example:
/// ```
/// use glerminal::menu_systems::TextItem;
///
/// TextItem::new("Just a text label");
/// TextItem::new("A button that can be pressed").with_is_button(true);
/// ```
pub struct TextItem {
    /// Foreground color for when the button is not focused
    pub fg_color_unfocused: Color,
    /// Background color for when the button is not focused
    pub bg_color_unfocused: Color,
    /// Foreground color for when the button is focused
    pub fg_color_focused: Color,
    /// Background color for when the button is focused
    pub bg_color_focused: Color,

    /// The keyboard inputs that trigger `was_just_pressed`
    pub button_press_inputs: Vec<VirtualKeyCode>,
    /// The mouse inputs that trigger `was_just_pressed`
    pub mouse_button_press_inputs: Vec<MouseButton>,
    /// The max width the text maximally cuts at
    pub max_width: u32,

    base: InterfaceItemBase,
    text: String,

    processed_text: Vec<ProcessedChar>,
    needs_processing: bool,

    is_button: bool,
    was_just_pressed: bool,
}

impl TextItem {
    /// Intiailizes a TextItem with the given text and gets the initial max width from the text width
    /// If the text might get bigger, use `with_max_width` or `set_max_width`.
    pub fn new<T: Into<String>>(text: T) -> TextItem {
        let text = text.into();
        TextItem {
            bg_color_unfocused: [0.0, 0.0, 0.0, 0.0],
            fg_color_unfocused: [0.8, 0.8, 0.8, 1.0],
            bg_color_focused: [0.8, 0.8, 0.8, 1.0],
            fg_color_focused: [0.2, 0.2, 0.2, 1.0],

            base: InterfaceItemBase::new(false),
            max_width: text.chars().count() as u32,
            text: text,

            processed_text: Vec::new(),
            needs_processing: true,

            is_button: false,
            was_just_pressed: false,
            button_press_inputs: vec![VirtualKeyCode::Return],
            mouse_button_press_inputs: Vec::new(),
        }
    }

    with_base!(TextItem);
    with_set_pressable!(TextItem);
    with_set_colors!(TextItem);

    /// Sets the initial max width of the TextItem
    pub fn with_max_width(mut self, max_width: u32) -> TextItem {
        self.max_width = max_width;
        self
    }

    /// Sets the initial text of the TextItem
    pub fn with_text<T: Into<String>>(mut self, text: T) -> TextItem {
        self.text = text.into();
        self
    }

    /// Set whether this TextItem can be focused and used as a button
    pub fn with_is_button(mut self, is_button: bool) -> TextItem {
        self.is_button = is_button;
        self.base.can_be_focused = is_button;
        self
    }

    /// Set whether this TextItem can be focused and used as a button
    pub fn set_is_button(&mut self, is_button: bool) {
        self.is_button = is_button;
        self.base.can_be_focused = is_button;
    }

    /// Sets the text of the TextItem
    pub fn set_text<T: Into<String>>(&mut self, text: T) {
        self.text = text.into();
        self.base.dirty = true;
        self.needs_processing = true;
    }

    /// Set the max width of the TextItem. This should ideally not be called, unless necessary.
    pub fn set_max_width(&mut self, max_width: u32) {
        self.max_width = max_width;
    }

    /// Return the current text of the TextItem
    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    /// Can this `TextItem` be focused and pressed like a button
    pub fn is_button(&self) -> bool {
        self.is_button
    }
}

impl InterfaceItem for TextItem {
    fn get_base(&self) -> &InterfaceItemBase {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut InterfaceItemBase {
        &mut self.base
    }

    fn get_total_width(&self) -> u32 {
        self.max_width
    }

    fn get_total_height(&self) -> u32 {
        1
    }

    fn draw(&mut self, text_buffer: &mut TextBuffer) {
        self.base.dirty = false;
        text_buffer.cursor.move_to(self.base.x, self.base.y);
        text_buffer.write_processed(
            &(self
                .processed_text
                .clone()
                .into_iter()
                .take(self.max_width as usize)
                .collect::<Vec<ProcessedChar>>()),
        );
    }

    fn handle_events(&mut self, events: &Events) -> bool {
        self.was_just_pressed = false;
        for curr in &self.button_press_inputs {
            if events.keyboard.was_just_pressed(*curr) {
                self.was_just_pressed = true;
                return true;
            }
        }
        for curr in &self.mouse_button_press_inputs {
            if events.mouse.was_just_pressed(*curr) {
                self.was_just_pressed = true;
                return true;
            }
        }
        false
    }

    fn update(&mut self, _: f32, processor: &TextProcessor) {
        if self.needs_processing || self.base.dirty {
            let (fg, bg) = if self.base.is_focused() {
                (self.fg_color_focused, self.bg_color_focused)
            } else {
                (self.fg_color_unfocused, self.bg_color_unfocused)
            };
            let style = TextStyle {
                fg_color: fg,
                bg_color: bg,
                ..Default::default()
            };
            self.processed_text = processor.process(vec![self.text.clone().into()], style);
            self.needs_processing = false;
        }
    }
}
