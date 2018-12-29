use super::{random_color, random_text, run_multiple_times, test_setup_text_buffer};
use menu_systems::{InterfaceItem, InterfaceItemBase, TextItem};
use MouseButton;
use VirtualKeyCode;

use rand::{thread_rng, Rng};

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

        let fg_focus = random_color();
        let fg_unfocus = random_color();
        let bg_focus = random_color();
        let bg_unfocus = random_color();

        let mut item = TextItem::new("")
            .with_pos((x, y))
            .with_focused(focused)
            .with_focused_colors((fg_focus, bg_focus))
            .with_unfocused_colors((fg_unfocus, bg_unfocus))
            .with_button_press_inputs(buttons.clone())
            .with_mouse_button_press_inputs(mouse_buttons.clone());

        // Test with_x macro-generated functions
        assert_eq!(item.get_base().get_pos(), (x, y));
        assert_eq!(item.get_base().is_focused(), focused);
        assert_eq!(item.fg_color_unfocused, fg_unfocus);
        assert_eq!(item.fg_color_focused, fg_focus);
        assert_eq!(item.bg_color_unfocused, bg_unfocus);
        assert_eq!(item.bg_color_focused, bg_focus);
        assert_eq!(item.button_press_inputs, buttons);
        assert_eq!(item.mouse_button_press_inputs, mouse_buttons);

        let fg_focus = random_color();
        let fg_unfocus = random_color();
        let bg_focus = random_color();
        let bg_unfocus = random_color();

        item.set_focused_colors((fg_focus, bg_focus));
        item.set_unfocused_colors((fg_unfocus, bg_unfocus));

        // Test set_x macrog-generated functions
        assert_eq!(item.fg_color_unfocused, fg_unfocus);
        assert_eq!(item.fg_color_focused, fg_focus);
        assert_eq!(item.bg_color_unfocused, bg_unfocus);
        assert_eq!(item.bg_color_focused, bg_focus);
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
