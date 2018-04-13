use events::Events;
use glutin::VirtualKeyCode;
use super::test_setup_open_terminal;

#[test]
fn test_input_was_just_pressed() {
    let button = VirtualKeyCode::A;

    let mut events = Events::new();
    events.keyboard.update_virtual_keycode(button, true);
    assert_eq!(events.keyboard.was_just_pressed(button), true);
    events.keyboard.clear_just_lists();
    assert_eq!(events.keyboard.was_just_pressed(button), false);
}

#[test]
fn test_input_was_just_released() {
    let button = VirtualKeyCode::A;

    let mut events = Events::new();
    events.keyboard.update_virtual_keycode(button, true);
    events.keyboard.clear_just_lists();
    events.keyboard.update_virtual_keycode(button, false);
    assert_eq!(events.keyboard.was_just_pressed(button), false);
    assert_eq!(events.keyboard.was_just_released(button), true);
    events.keyboard.clear_just_lists();
    assert_eq!(events.keyboard.was_just_released(button), false);
}

#[test]
fn test_input_is_pressed() {
    let button = VirtualKeyCode::A;

    let mut events = Events::new();
    events.keyboard.update_virtual_keycode(button, true);
    assert_eq!(events.keyboard.is_pressed(button), true);
    events.keyboard.clear_just_lists();
    assert_eq!(events.keyboard.is_pressed(button), true);
}

#[test]
fn test_input_was_just_pressed_with_terminal() {
    let button = VirtualKeyCode::A;
    let mut terminal = test_setup_open_terminal();
    if !terminal.headless {
        terminal.update_virtual_keycode(button, true);
        let mut events = terminal.get_current_events();

        assert_eq!(events.keyboard.was_just_pressed(button), true);
        events.keyboard.clear_just_lists();
        assert_eq!(events.keyboard.was_just_pressed(button), false);
    }
}
