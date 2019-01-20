use super::{random_text, run_multiple_times, test_setup_text_buffer};
use crate::menu_systems::{Dialog, InterfaceItem};
use crate::text_processing::DefaultProcessor;
use crate::VirtualKeyCode::{Down, Up, L, O};
use crate::{Events, VirtualKeyCode};

use rand::{thread_rng, Rng};

#[test]
fn draw() {
    run_multiple_times(20, || {
        let mut rand = thread_rng();

        let width = rand.gen_range(10, 15);
        let height = rand.gen_range(3, 5);

        let mut text_buffer = test_setup_text_buffer((width, height));

        let word_amount = rand.gen_range(5, 20);

        let mut text = String::new();
        let mut row_text = String::new();
        let mut rows = Vec::new();
        for _ in 0..word_amount {
            if !text.is_empty() {
                text += " ";
            }
            text += &random_text(rand.gen_range(2, 5));
        }
        for word in text.split(' ') {
            if (row_text.len() + word.len() + 1) as u32 <= width {
                if !row_text.is_empty() {
                    row_text += " ";
                }
                row_text += &word;
            } else {
                rows.push(row_text);
                row_text = word.to_owned();
            }
        }
        rows.push(row_text);

        let scroll = rand.gen_range(0, rows.len() as u32 - 1);

        let mut dialog = Dialog::new(width, height, height).with_text(text);
        dialog.update(0.0, &DefaultProcessor);

        dialog.set_scroll(scroll);
        dialog.draw(&mut text_buffer);

        let expected: Vec<String> = rows
            .into_iter()
            .skip(scroll as usize)
            .take(height as usize)
            .collect();
        for (idx, row) in expected.iter().enumerate() {
            for (char_idx, c) in row.chars().enumerate() {
                assert_eq!(
                    c,
                    text_buffer
                        .get_character(char_idx as u32, idx as u32)
                        .unwrap()
                        .get_char()
                );
            }
        }
    });
}

#[test]
fn scroll() {
    let mut rand = thread_rng();

    let width = rand.gen_range(10, 15);
    let height = rand.gen_range(3, 5);
    let rows = rand.gen_range(2, 7);

    let scroll_amount_test = rows - 1;

    let mut text = String::new();
    for _ in 0..rows {
        if text.is_empty() {
            text += " ";
        }
        text += &random_text(width);
    }

    let mut dialog = Dialog::new(width, height, height).with_text(text);
    dialog.update(0.0, &DefaultProcessor);

    assert_eq!(dialog.get_scroll(), 0);

    for _ in 0..scroll_amount_test {
        dialog.scroll_down();
    }
    assert_eq!(dialog.get_scroll(), scroll_amount_test);

    for _ in 0..scroll_amount_test {
        dialog.scroll_up();
    }
    assert_eq!(dialog.get_scroll(), 0);

    dialog.set_scroll(scroll_amount_test);
    assert_eq!(dialog.get_scroll(), scroll_amount_test);
}

#[test]
fn handle_input() {
    run_multiple_times(50, || {
        let mut events = Events::new(false);

        let mut rand = thread_rng();

        let width = rand.gen_range(10, 15);
        let height = rand.gen_range(3, 5);

        let change_up_button = rand.gen();
        let change_down_button = rand.gen();

        let mut text = String::new();
        for _ in 0..3 {
            if text.is_empty() {
                text += " ";
            }
            text += &random_text(width);
        }

        let mut dialog = Dialog::new(width, height, height).with_text(text);
        dialog.update(0.0, &DefaultProcessor);

        if change_up_button {
            dialog.up_buttons = vec![O];
        }
        if change_down_button {
            dialog.down_buttons = vec![L];
        }

        let mut check = |initial: u32, key: VirtualKeyCode, expect: u32| {
            dialog.set_scroll(initial);
            events.keyboard.update_button_press(key, true);
            dialog.handle_events(&events);
            events.keyboard.update_button_press(key, false);
            events.clear_just_lists();
            assert_eq!(dialog.get_scroll(), expect)
        };

        let expect = if change_up_button { 1 } else { 0 };
        check(1, Up, expect);
        let expect = if change_down_button { 0 } else { 1 };
        check(0, Down, expect);
        let expect = if change_up_button { 0 } else { 1 };
        check(1, O, expect);
        let expect = if change_down_button { 1 } else { 0 };
        check(0, L, expect);
    });
}
