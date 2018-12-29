use super::{random_text, run_multiple_times, test_setup_text_buffer};
use menu_systems::{Filter, InterfaceItem, TextInput};
use Events;
use VirtualKeyCode::{Back, Key1, Return, A};

use rand::{thread_rng, Rng};

#[test]
fn set_get_test() {
    let text = random_text(10);
    let mut item = TextInput::new(None, None).with_text(text.clone());

    assert_eq!(item.get_text(), text);

    let text = random_text(10);
    item.set_text(text.clone());

    assert_eq!(item.get_text(), text);
}

#[test]
fn back_removing() {
    let mut rand = thread_rng();
    let mut events = Events::new(false);
    let text = random_text(10);
    let removed = rand.gen_range(1, 9);
    let mut item = TextInput::new(None, None)
        .with_text(text.clone())
        .with_focused(true);

    let expected: String = text.chars().take(10 - removed).collect();

    events.keyboard.update_button_press(Back, true);
    for _ in 0..removed {
        item.handle_events(&events);
    }
    assert_eq!(item.get_text(), expected);
}

#[test]
fn input_handling_and_filters() {
    run_multiple_times(50, || {
        let mut rand = thread_rng();
        let mut events = Events::new(false);
        let mut item = TextInput::new(None, None).with_focused(true);
        let test_a = rand.gen();
        let test_key1 = rand.gen();

        events.keyboard.update_button_press(Return, true);
        item.handle_events(&events);
        events.clear_just_lists();
        assert_eq!(item.was_just_pressed(), true);

        let mut filter = Filter::empty_filter();
        let mut expected = String::new();
        if test_a {
            filter = filter.with_basic_latin_characters();
            expected += "a";
        }
        if test_key1 {
            filter = filter.with_basic_numerals();
            expected += "1";
        }
        item.filter = filter;

        events.keyboard.update_button_press(A, true);
        item.handle_events(&events);
        events.clear_just_lists();
        events.keyboard.update_button_press(A, false);

        events.keyboard.update_button_press(Key1, true);
        item.handle_events(&events);

        assert_eq!(expected, item.get_text());
    });
}

#[test]
fn input_handling_no_focus() {
    run_multiple_times(50, || {
        let mut events = Events::new(false);
        let mut item = TextInput::new(None, None);

        events.keyboard.update_button_press(Return, true);
        item.handle_events(&events);
        events.clear_just_lists();
        assert_eq!(item.was_just_pressed(), false);
    });
}

#[test]
fn caret() {
    run_multiple_times(20, || {
        let caret_time = thread_rng().gen_range(0.05, 0.2);

        let mut item = TextInput::new(None, None)
            .with_focused(true)
            .with_caret(caret_time);
        assert_eq!(item.caret_showing(), false);
        item.update(caret_time);
        assert_eq!(item.caret_showing(), true);
        item.update(caret_time);
        assert_eq!(item.caret_showing(), false);
    });
}

#[test]
fn draw() {
    run_multiple_times(20, || {
        let mut rng = thread_rng();

        let mut text_buffer = test_setup_text_buffer((20, 1));

        let text = random_text(5);
        let prefix = random_text(5);
        let suffix = random_text(1);
        let caret = rng.gen_range(0.05, 0.2);

        let mut item = TextInput::new(None, None)
            .with_prefix(prefix.clone())
            .with_suffix(suffix.clone())
            .with_caret(caret)
            .with_text(text.clone())
            .with_focused(true);

        item.draw(&mut text_buffer);

        let mut caret_should_show = false;
        for _ in 0..4 {
            let caret_text = if caret_should_show { "_" } else { " " };
            let whole = format!("{}{}{}{}", prefix, text, caret_text, suffix);
            item.draw(&mut text_buffer);
            for (idx, c) in whole.chars().enumerate() {
                assert_eq!(
                    text_buffer.get_character(idx as i32, 0).unwrap().get_char(),
                    c
                )
            }
            caret_should_show = !caret_should_show;
            item.update(caret);
        }
    });
}

#[test]
fn min_max_width() {
    run_multiple_times(50, || {
        let mut rng = thread_rng();

        let text = random_text(5);
        let prefix = random_text(5);
        let suffix = random_text(1);

        let min_width = rng.gen_range(1, 6);
        let max_width = min_width + 2;

        let item = TextInput::new(min_width, max_width)
            .with_prefix(prefix.clone())
            .with_suffix(suffix.clone())
            .with_text(text.clone())
            .with_focused(true);

        let len = prefix.len()
            + suffix.len()
            + text.len().min(min_width as usize).max(max_width as usize);
        assert_eq!(item.get_total_width(), len as u32);
    });
}
