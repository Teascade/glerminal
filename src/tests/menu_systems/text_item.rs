use super::{random_text, run_multiple_times, test_setup_text_buffer};
use menu_systems::{InterfaceItem, TextItem};

use rand::{thread_rng, Rng};
use std::iter::repeat;

#[test]
fn default_max_width() {
    run_multiple_times(50, || {
        let mut rng = thread_rng();
        let len: u32 = rng.gen_range(0, 500);
        let item = TextItem::new(repeat('a').take(len as usize).collect::<String>());
        assert_eq!(item.get_total_width(), len);
    });
}

#[test]
fn with_functions() {
    run_multiple_times(50, || {
        let mut rng = thread_rng();
        let max_width = rng.gen();
        let is_button = rng.gen();
        let text = random_text(15);
        let item = TextItem::new("")
            .with_text(text.clone())
            .with_max_width(max_width)
            .with_is_button(is_button);

        assert_eq!(item.get_text(), text);
        assert_eq!(item.is_button(), is_button);
        assert_eq!(item.get_total_width(), max_width);

        // Check that height doesn't change just to be sure
        assert_eq!(item.get_total_height(), 1);
    });
}

#[test]
fn set_functions() {
    run_multiple_times(50, || {
        let mut item = TextItem::new("");

        let mut rng = thread_rng();

        let max_width = rng.gen();
        let is_button = rng.gen();
        let text = random_text(15);

        item.set_is_button(is_button);
        item.set_max_width(max_width);
        item.set_text(text.clone());

        assert_eq!(item.get_total_width(), max_width);
        assert_eq!(item.get_text(), text);
        assert_eq!(item.is_button(), is_button);

        // Check that height doesn't change just to be sure
        assert_eq!(item.get_total_height(), 1);
    });
}

#[test]
fn name() {
    run_multiple_times(50, || {
        let mut text_buffer = test_setup_text_buffer((15, 1));
        let text = random_text(15);
        let mut item = TextItem::new(text.clone());
        item.draw(&mut text_buffer);
        for (idx, c) in text.chars().enumerate() {
            assert_eq!(
                text_buffer.get_character(idx as i32, 0).unwrap().get_char(),
                c
            );
        }
    });
}
