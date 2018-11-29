use super::{random_color, run_multiple_times, test_setup_text_buffer};
use rand;
use text_buffer::parser::Parser;

#[test]
fn add_color() {
    run_multiple_times(1000, || {
        let mut parser = Parser::new();

        let color = random_color();
        parser.add_color("test", color);
        assert_eq!(*parser.get_color("test".to_owned()).unwrap(), color);
    });
}

#[test]
fn write() {
    run_multiple_times(10, || {
        let mut text_buffer = test_setup_text_buffer((2, 2));

        let mut parser = Parser::new();
        let color = random_color();
        let shake = rand::random::<f32>();
        parser.add_color("test", color);
        parser.write(
            &mut text_buffer,
            format!("[fg=test]a[/fg][bg=test]a[/bg][shake={}]a[/shake]", shake),
        );

        let character = text_buffer.get_character(0, 0);
        assert_eq!(character.get_fg_color(), color);
        assert_eq!(character.get_bg_color(), [0.0; 4]);
        assert_eq!(character.get_shakiness(), 0.0);
        let character = text_buffer.get_character(1, 0);
        assert_eq!(character.get_fg_color(), [1.0; 4]);
        assert_eq!(character.get_bg_color(), color);
        assert_eq!(character.get_shakiness(), 0.0);
        let character = text_buffer.get_character(0, 1);
        assert_eq!(character.get_fg_color(), [1.0; 4]);
        assert_eq!(character.get_bg_color(), [0.0; 4]);
        assert_eq!(character.get_shakiness(), shake);
    });
}
