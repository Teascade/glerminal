use super::{random_color, random_text, run_multiple_times, test_setup_text_buffer};
use crate::menu_systems::{InterfaceItem, InterfaceItemBase, TextItem};
use crate::{MouseButton, VirtualKeyCode, TextStyle};

use rand::{thread_rng, Rng};

mod checkbox;
mod dialog;
mod menu;
mod text_input;
mod text_item;

#[test]
fn with_set_macros() {
    run_multiple_times(50, || {
        let mut rng = thread_rng();

        let x = rng.gen_range(0, 15);
        let y = rng.gen_range(0, 15);
        let focused = rng.gen();

        let buttons = vec![VirtualKeyCode::At, VirtualKeyCode::F];
        let mouse_buttons = vec![MouseButton::Middle];


        let unfocus_style = TextStyle {
            fg_color: random_color(),
            bg_color: random_color(),
            shakiness: rng.gen(),
        };

        let focus_style = TextStyle {
            fg_color: random_color(),
            bg_color: random_color(),
            shakiness: rng.gen(),
        };

        let item = TextItem::new("")
            .with_pos((x, y))
            .with_focused(focused)
            .with_focused_style(focus_style)
            .with_unfocused_style(unfocus_style)
            .with_button_press_inputs(buttons.clone())
            .with_mouse_button_press_inputs(mouse_buttons.clone());

        // Test with_x macro-generated functions
        assert_eq!(item.get_base().get_pos(), (x, y));
        assert_eq!(item.get_base().is_focused(), focused);
        assert_eq!(item.unfocused_style, unfocus_style);
        assert_eq!(item.focused_style, focus_style);
        assert_eq!(item.button_press_inputs, buttons);
        assert_eq!(item.mouse_button_press_inputs, mouse_buttons);
    });
}

#[test]
fn inteface_item_base() {
    run_multiple_times(50, || {
        let mut rng = thread_rng();
        let can_be_focused = rng.gen();
        let mut base = InterfaceItemBase::new(can_be_focused);

        // Test initial values
        assert_eq!(base.get_pos(), (0, 0));
        assert_eq!(base.can_be_focused, can_be_focused);
        assert_eq!(base.dirty, false);
        assert_eq!(base.is_focused(), false);

        let x = rng.gen_range(0, 15);
        let y = rng.gen_range(0, 15);
        let focused = rng.gen();
        base.set_pos((x, y));
        base.set_focused(focused);

        // Test functions
        assert_eq!(base.get_pos(), (x, y));
        assert_eq!(base.is_focused(), focused);
    });
}
