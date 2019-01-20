use super::{
    random_char, random_color, random_text, run_multiple_times, test_setup_text_buffer,
    test_setup_text_buffer_with_terminal,
};
use crate::TextStyle;
use rand::{thread_rng, Rng};

#[test]
fn aspect_ratio() {
    run_multiple_times(10, || {
        let mut rnd = thread_rng();

        let width = rnd.gen_range(2, 100);
        let height = rnd.gen_range(2, 100);

        let (text_buffer, terminal) = test_setup_text_buffer_with_terminal((width, height));
        let ar_height = height * terminal.font.line_height;
        let ar_width = width as f32 * terminal.font.average_xadvance;
        let ar = ar_width / ar_height as f32;

        assert_eq!(text_buffer.aspect_ratio, ar);
    });
}

#[test]
fn size() {
    run_multiple_times(10, || {
        let mut rnd = thread_rng();

        let width = rnd.gen_range(2, 100);
        let height = rnd.gen_range(2, 100);

        let text_buffer = test_setup_text_buffer((width, height));
        assert_eq!(text_buffer.chars.len(), (width * height) as usize);
        assert_eq!(text_buffer.width, width);
        assert_eq!(text_buffer.height, height);
    });
}

#[test]
fn get_char_empty() {
    let text_buffer = test_setup_text_buffer((2, 2));
    let character = text_buffer.get_character(0, 0).unwrap();
    assert_eq!(character.get_char(), ' ');
    assert_eq!(character.style.fg_color, [1.0; 4]);
    assert_eq!(character.style.bg_color, [0.0; 4]);
    assert_eq!(character.style.shakiness, 0.0);
}

#[test]
fn get_raw_char() {
    let text_buffer = test_setup_text_buffer((2, 2));
    let character = text_buffer.get_character(0, 0).unwrap();
    assert_eq!(character.get_raw_char(), 32);
}

#[test]
fn put_single_character() {
    run_multiple_times(10, || {
        let mut character = ' ';
        while character == ' ' {
            character = random_char();
        }
        let mut text_buffer = test_setup_text_buffer((2, 2));
        text_buffer.put_char(character);

        let termcharacter = text_buffer.get_character(0, 0).unwrap();
        assert_eq!(termcharacter.get_char(), character);
    });
}

#[test]
fn write_three_characters() {
    run_multiple_times(10, || {
        let mut text = random_text(3);

        let mut text_buffer = test_setup_text_buffer((2, 2));
        text_buffer.write(text.clone());

        let character = text_buffer.get_character(0, 0).unwrap();
        assert_eq!(character.get_char(), text.remove(0));
        let character = text_buffer.get_character(1, 0).unwrap();
        assert_eq!(character.get_char(), text.remove(0));
        let character = text_buffer.get_character(0, 1).unwrap();
        assert_eq!(character.get_char(), text.remove(0));
    })
}

#[test]
fn put_single_styled_character() {
    run_multiple_times(10, || {
        let style = TextStyle {
            fg_color: random_color(),
            bg_color: random_color(),
            shakiness: rand::random::<f32>(),
        };

        let mut text_buffer = test_setup_text_buffer((2, 2));
        text_buffer.cursor.style = style;
        text_buffer.put_char('a');

        let character = text_buffer.get_character(0, 0).unwrap();
        assert_eq!(character.style, style);
    });
}

#[test]
fn cursor_move() {
    run_multiple_times(10, || {
        let mut rnd = thread_rng();

        let width = rnd.gen_range(3, 100);
        let height = rnd.gen_range(3, 100);

        let mut text_buffer = test_setup_text_buffer((width, height));
        let x = rnd.gen_range(0, width - 2);
        let y = rnd.gen_range(0, height - 2);
        text_buffer.cursor.move_to(x, y);
        assert_eq!(text_buffer.get_cursor_position(), (x, y));
        text_buffer.cursor.move_to(width, height);
        assert_eq!(text_buffer.get_cursor_position(), (width - 1, height - 1));
    });
}
