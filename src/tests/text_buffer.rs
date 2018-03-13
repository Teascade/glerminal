use super::{run_multiple_times, test_setup_text_buffer};
use text_buffer::TextBuffer;
use rand;
use rand::distributions::{Range, Sample};

#[test]
fn test_text_buffer_size() {
    run_multiple_times(|| {
        let mut range = Range::new(2i32, 100);
        let mut rnd = rand::thread_rng();

        let width = range.sample(&mut rnd);
        let height = range.sample(&mut rnd);

        let text_buffer = test_setup_text_buffer((width, height));
        assert_eq!(text_buffer.chars.len(), (width * height) as usize);
        assert_eq!(text_buffer.width, width);
        assert_eq!(text_buffer.height, height);
    });
}

#[test]
fn test_text_buffer_chars_len() {
    run_multiple_times(|| {
        let mut range = Range::new(2i32, 100);
        let mut rnd = rand::thread_rng();

        let width = range.sample(&mut rnd);
        let height = range.sample(&mut rnd);

        let text_buffer = test_setup_text_buffer((width, height));
        assert_eq!(text_buffer.chars.len(), (width * height) as usize);
    });
}

#[test]
fn test_text_buffer_chars_get_char_empty() {
    let text_buffer = test_setup_text_buffer((2, 2));
    let character = text_buffer.get_character(0, 0);
    assert_eq!(character.get_char(), ' ');
    assert_eq!(character.get_fg_color(), [0.0; 4]);
    assert_eq!(character.get_bg_color(), [0.0; 4]);
    assert_eq!(character.get_shakiness(), 0.0);
}

#[test]
fn test_text_buffer_chars_put_single_character() {
    run_multiple_times(|| {
        let character = rand::random::<char>();
        let mut text_buffer = test_setup_text_buffer((2, 2));
        text_buffer.put_char(character);

        let termcharacter = text_buffer.get_character(0, 0);
        assert_eq!(termcharacter.get_char(), character);
    });
}

#[test]
fn test_text_buffer_chars_write_three_characters() {
    run_multiple_times(|| {
        let mut text = String::new();
        for _ in 0..3 {
            text.push(rand::random::<char>());
        }

        let mut text_buffer = test_setup_text_buffer((2, 2));
        text_buffer.write(text.clone());

        let character = text_buffer.get_character(0, 0);
        assert_eq!(character.get_char(), text.remove(0));
        let character = text_buffer.get_character(1, 0);
        assert_eq!(character.get_char(), text.remove(0));
        let character = text_buffer.get_character(0, 1);
        assert_eq!(character.get_char(), text.remove(0));
    })
}

#[test]
fn test_text_buffer_chars_put_single_styled_character() {
    run_multiple_times(|| {
        let fg_color = [
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        ];
        let bg_color = [
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
            rand::random::<f32>(),
        ];
        let shakiness = rand::random::<f32>();

        let mut text_buffer = test_setup_text_buffer((2, 2));
        text_buffer.change_cursor_fg_color(fg_color);
        text_buffer.change_cursor_bg_color(bg_color);
        text_buffer.change_cursor_shakiness(shakiness);
        text_buffer.put_char('a');

        let character = text_buffer.get_character(0, 0);
        assert_eq!(character.get_fg_color(), fg_color);
        assert_eq!(character.get_bg_color(), bg_color);
        assert_eq!(character.get_shakiness(), shakiness);
    });
}
