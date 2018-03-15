use input::Input;
use glutin::VirtualKeyCode;
use super::test_setup_open_terminal;

#[test]
fn test_input_was_just_pressed() {
    let button = VirtualKeyCode::A;

    let mut input = Input::new();
    input.update_virtual_keycode(button, true);
    assert_eq!(input.was_just_pressed(button), true);
    input.clear_just_lists();
    assert_eq!(input.was_just_pressed(button), false);
}

#[test]
fn test_input_was_just_released() {
    let button = VirtualKeyCode::A;

    let mut input = Input::new();
    input.update_virtual_keycode(button, true);
    input.clear_just_lists();
    input.update_virtual_keycode(button, false);
    assert_eq!(input.was_just_pressed(button), false);
    assert_eq!(input.was_just_released(button), true);
    input.clear_just_lists();
    assert_eq!(input.was_just_released(button), false);
}

#[test]
fn test_input_is_pressed() {
    let button = VirtualKeyCode::A;

    let mut input = Input::new();
    input.update_virtual_keycode(button, true);
    assert_eq!(input.is_pressed(button), true);
    input.clear_just_lists();
    assert_eq!(input.is_pressed(button), true);
}

#[test]
fn test_input_was_just_pressed_with_terminal() {
    let button = VirtualKeyCode::A;
    let mut terminal = test_setup_open_terminal();
    if !terminal.headless {
        terminal.update_virtual_keycode(button, true);
        let mut input = terminal.get_current_input();

        assert_eq!(input.was_just_pressed(button), true);
        input.clear_just_lists();
        assert_eq!(input.was_just_pressed(button), false);
    }
}
