use super::test_setup_open_terminal;
use events::Events;
use glutin::VirtualKeyCode;

#[test]
fn was_just_pressed() {
    let button = VirtualKeyCode::A;

    let mut events = Events::new(true);
    events.keyboard.update_button_press(button, true);
    assert_eq!(events.keyboard.was_just_pressed(button), true);
    events.keyboard.clear_just_lists();
    assert_eq!(events.keyboard.was_just_pressed(button), false);
}

#[test]
fn was_just_released() {
    let button = VirtualKeyCode::A;

    let mut events = Events::new(true);
    events.keyboard.update_button_press(button, true);
    events.keyboard.clear_just_lists();
    events.keyboard.update_button_press(button, false);
    assert_eq!(events.keyboard.was_just_pressed(button), false);
    assert_eq!(events.keyboard.was_just_released(button), true);
    events.keyboard.clear_just_lists();
    assert_eq!(events.keyboard.was_just_released(button), false);
}

#[test]
fn is_pressed() {
    let button = VirtualKeyCode::A;

    let mut events = Events::new(true);
    events.keyboard.update_button_press(button, true);
    assert_eq!(events.keyboard.is_pressed(button), true);
    events.keyboard.clear_just_lists();
    assert_eq!(events.keyboard.is_pressed(button), true);
}

#[test]
fn was_just_pressed_with_terminal() {
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
