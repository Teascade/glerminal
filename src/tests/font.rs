use crate::font::Font;
use crate::FontFormat;

static SCP_SFL: &'static str = include_str!("../../fonts/source_code_pro.sfl");
static SCP_PNG: &'static [u8] = include_bytes!("../../fonts/source_code_pro.png");

fn test_load_font() -> Font {
    Font::load(&FontFormat::SFL, "fonts/source_code_pro.sfl")
}

fn test_load_font_raw() -> Font {
    Font::load_raw(&FontFormat::SFL, SCP_SFL, SCP_PNG)
}

#[test]
fn name() {
    assert_eq!(test_load_font().name, "Source Code Pro");
}

#[test]
fn load_raw_eq_load() {
    assert_eq!(test_load_font(), test_load_font_raw());
}

#[test]
fn size() {
    let font = test_load_font();
    assert_eq!(font.width, 1024);
    assert_eq!(font.height, 1024);
}

#[test]
fn line_height_and_size() {
    let font = test_load_font();
    assert_eq!(font.line_height, 54);
    assert_eq!(font.size, 32);
}

#[test]
fn character_amount() {
    let font = test_load_font();
    assert_eq!(font.characters.keys().len(), 483);
}

#[test]
fn single_character_width() {
    let font = test_load_font();
    assert_eq!(font.get_character('a' as u16).unwrap().width, 22);
}

#[test]
fn single_character_height() {
    let font = test_load_font();
    assert_eq!(font.get_character('a' as u16).unwrap().height, 23);
}

#[test]
fn single_character_offsets() {
    let font = test_load_font();
    assert_eq!(font.get_character('a' as u16).unwrap().x_off, 2);
    assert_eq!(font.get_character('a' as u16).unwrap().y_off, 20);
}

#[test]
fn single_character_id() {
    let font = test_load_font();
    assert_eq!(font.get_character('a' as u16).unwrap().id, 97);
}
