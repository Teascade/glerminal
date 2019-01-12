use super::{
    random_char, random_color, random_text, run_multiple_times, test_setup_text_buffer,
    test_setup_text_buffer_with_terminal,
};
use rand::{thread_rng, Rng};

#[test]
fn aspect_ratio() {
    run_multiple_times(10, || {
        let mut rnd = thread_rng();

        let width = rnd.gen_range(2, 100);
        let height = rnd.gen_range(2, 100);

        let (text_buffer, terminal) = test_setup_text_buffer_with_terminal((width, height));
        let ar_height = height * terminal.font.line_height;
        let ar_width = width * terminal.font.size;
        let ar = ar_width as f32 / ar_height as f32;

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
    assert_eq!(character.get_fg_color(), [0.0; 4]);
    assert_eq!(character.get_bg_color(), [0.0; 4]);
    assert_eq!(character.get_shakiness(), 0.0);
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
        let fg_color = random_color();
        let bg_color = random_color();
        let shakiness = rand::random::<f32>();

        let mut text_buffer = test_setup_text_buffer((2, 2));
        text_buffer.change_cursor_fg_color(fg_color);
        text_buffer.change_cursor_bg_color(bg_color);
        text_buffer.change_cursor_shakiness(shakiness);
        text_buffer.put_char('a');

        let character = text_buffer.get_character(0, 0).unwrap();
        assert_eq!(character.get_fg_color(), fg_color);
        assert_eq!(character.get_bg_color(), bg_color);
        assert_eq!(character.get_shakiness(), shakiness);
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
        text_buffer.move_cursor(x, y);
        //assert_eq!(text_buffer.get_cursor_position(), (x, y));
        text_buffer.move_cursor(width, height);
        //assert_eq!(text_buffer.get_cursor_position(), (x, y));
    });
}

#[test]
fn cursor_styles() {
    run_multiple_times(10, || {
        let mut text_buffer = test_setup_text_buffer((2, 2));
        let fg = random_color();
        let bg = random_color();
        let shakiness = rand::random::<f32>();

        text_buffer.change_cursor_fg_color(fg);
        assert_eq!(text_buffer.get_cursor_fg_color(), fg);
        text_buffer.change_cursor_bg_color(bg);
        assert_eq!(text_buffer.get_cursor_bg_color(), bg);
        text_buffer.change_cursor_shakiness(shakiness);
        assert_eq!(text_buffer.get_cursor_shakiness(), shakiness);
    });
}
