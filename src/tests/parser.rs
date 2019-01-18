use super::{random_color, run_multiple_times, test_setup_text_buffer};
use crate::{Parser, TextStyle};

#[test]
fn add_color() {
    run_multiple_times(1000, || {
        let mut parser = Parser::new();

        let color = random_color();
        parser.add_color("test", color);
        assert_eq!(*parser.get_color("test").unwrap(), color);
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

        let character = text_buffer.get_character(0, 0).unwrap();
        assert_eq!(
            character.style,
            TextStyle {
                fg_color: color,
                ..Default::default()
            }
        );
        let character = text_buffer.get_character(1, 0).unwrap();
        assert_eq!(
            character.style,
            TextStyle {
                bg_color: color,
                ..Default::default()
            }
        );
        let character = text_buffer.get_character(0, 1).unwrap();
        assert_eq!(
            character.style,
            TextStyle {
                shakiness: shake,
                ..Default::default()
            }
        );
    });
}
