use super::{random_text, run_multiple_times, test_setup_text_buffer};
use crate::menu_systems::{Checkbox, CheckboxGroup, InterfaceItem};
use crate::text_processing::DefaultProcessor;
use crate::Events;
use crate::VirtualKeyCode::{Return, A};

use rand::{thread_rng, Rng};
use std::iter::repeat;

#[test]
fn group() {
    run_multiple_times(50, || {
        let mut rand = thread_rng();
        let force_one_checked: bool = rand.gen();

        let mut group = if force_one_checked {
            CheckboxGroup::new().with_force_one_checked(Some(0))
        } else {
            CheckboxGroup::new()
        };

        let mut cb1 = Checkbox::new("");
        let mut cb2 = Checkbox::new("");
        let mut cb3 = Checkbox::new("");

        cb1.set_checked(true);
        group.update(&mut [&mut cb1, &mut cb2, &mut cb3]);
        assert_eq!(group.get_selection_idx().unwrap(), 0);

        cb2.set_checked(true);
        group.update(&mut [&mut cb1, &mut cb2, &mut cb3]);
        assert_eq!(group.get_selection_idx().unwrap(), 1);

        cb3.set_checked(true);
        group.update(&mut [&mut cb1, &mut cb2, &mut cb3]);
        assert_eq!(group.get_selection_idx().unwrap(), 2);

        cb3.set_checked(false);
        group.update(&mut [&mut cb1, &mut cb2, &mut cb3]);
        assert_eq!(group.get_selection_idx().is_some(), force_one_checked);
    })
}

#[test]
fn checked() {
    run_multiple_times(50, || {
        let mut rand = thread_rng();
        let mut item = Checkbox::new("");

        let checked = rand.gen();
        item.set_checked(checked);
        assert_eq!(item.is_checked(), checked);
    });
}

#[test]
fn draw() {
    run_multiple_times(20, || {
        let mut rand = thread_rng();
        let mut text_buffer = test_setup_text_buffer((20, 1));

        let checked = rand.gen();
        let text = random_text(5);
        let prefix = random_text(5);
        let suffix = random_text(5);
        let checked_text = random_text(5);
        let nonchecked_text = repeat(' ').take(5).collect();

        let mut item = Checkbox::new(text.clone())
            .with_checked(checked)
            .with_prefix(prefix.clone())
            .with_suffix(suffix.clone())
            .with_checked_text(checked_text.clone());

        item.update(0.0, &DefaultProcessor);
        item.draw(&mut text_buffer);

        let is_checked_text = if checked {
            checked_text
        } else {
            nonchecked_text
        };
        let expected = format!("{}{}{}{}", text, prefix, is_checked_text, suffix);

        for (idx, c) in expected.chars().enumerate() {
            assert_eq!(
                c,
                text_buffer.get_character(idx as u32, 0).unwrap().get_char()
            );
        }
    });
}

#[test]
fn handle_input() {
    run_multiple_times(50, || {
        let mut rand = thread_rng();
        let mut events = Events::new(false);

        let change_button = rand.gen();

        let mut item = Checkbox::new("");

        if change_button {
            item.button_press_inputs = vec![A];
        }

        events.keyboard.update_button_press(Return, true);
        item.handle_events(&events);
        events.keyboard.update_button_press(Return, false);
        events.clear_just_lists();
        assert_eq!(item.was_just_pressed(), !change_button);

        events.keyboard.update_button_press(A, true);
        item.handle_events(&events);
        events.keyboard.update_button_press(A, false);
        events.clear_just_lists();
        assert_eq!(item.was_just_pressed(), change_button);
    });
}
