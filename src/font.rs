use png::{ColorType, Decoder};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::FontFormat;
use bmfont_parser::BMFont;

/// Contains data of a single character in a Font
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterData {
    pub(crate) id: u32,
    pub(crate) x1: f32,
    pub(crate) x2: f32,
    pub(crate) y1: f32,
    pub(crate) y2: f32,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) x_off: i32,
    pub(crate) y_off: i32,
}

/// The Font is used to load fonts that can be used in the [`TextBuffer`](text_buffer/struct.TextBuffer.html)
///
/// The Font can be loaded from an `.sfl` file and then used in the `TextBuffer`, in example:
/// ```
/// use glerminal::{TerminalBuilder, Font, FontFormat};
///
/// let mut terminal = TerminalBuilder::new()
///     .with_title("Hello glerminal::font::Font!")
///     .with_dimensions((1280, 720))
///     .with_font(Font::load(&FontFormat::SFL, "fonts/iosevka.sfl"))
///     .with_headless(true)
///     .build();
/// ```
///
/// Alternatively you can use `load_raw` to load the font straight with `include_str!` and `include_bytes!`, example:
/// ```
/// use glerminal::{TerminalBuilder, Font, FontFormat};
///
/// static IOSEVKA_SFL: &'static str = include_str!("../fonts/iosevka.sfl");
/// static IOSEVKA_PNG: &'static [u8] = include_bytes!("../fonts/iosevka.png");
///
/// let mut terminal = TerminalBuilder::new()
///     .with_title("Hello glerminal::font::Font!")
///     .with_dimensions((1280, 720))
///     .with_font(Font::load_raw(&FontFormat::SFL, IOSEVKA_SFL, IOSEVKA_PNG))
///     .with_headless(true)
///     .build();
/// ```
#[derive(Debug, PartialEq)]
pub struct Font {
    /// The name of the font
    pub name: String,
    pub(crate) image_buffer: Vec<u8>,
    pub(crate) width: u32,
    pub(crate) height: u32,
    /// Line height of the font
    pub line_height: u32,
    /// Size of the font (width)
    pub size: u32,
    pub(crate) min_offset_y: i32,
    pub(crate) characters: HashMap<u16, CharacterData>,
}

impl Font {
    /// Loads the font fron the given .sfl file, for example:
    ///
    /// ```
    /// use glerminal::{Font, FontFormat};
    /// let font = Font::load(&FontFormat::SFL, "fonts/iosevka.sfl");
    /// ```
    pub fn load<T: Into<PathBuf>>(format: &FontFormat, fnt_path: T) -> Font {
        let fnt_path = fnt_path.into();
        if !fnt_path.exists() {
            panic!("Font image or .sfl file missing");
        }
        // Load Font .sfl file
        let bm_font;
        match BMFont::from_path(format, fnt_path) {
            Ok(bmf) => bm_font = bmf,
            Err(error) => panic!("Failed to load .sfl file: {}", error),
        }

        // Load Font image file
        let image_path = &bm_font.pages[0].image_path;
        println!("{:?}", image_path);
        Font::load_with_bmfont_and_image_read(&bm_font, File::open(image_path).unwrap())
    }

    /// Loads the font from the given string (.sfl file contents) and Read (image read)
    ///
    /// ```
    /// use glerminal::{Font, FontFormat};
    /// use std::fs::File;
    ///
    /// let font = Font::load_raw(&FontFormat::SFL, include_str!("../fonts/iosevka.sfl"), File::open("fonts/iosevka.png").unwrap());
    /// ```
    pub fn load_raw<T: Into<String>, R: Read>(
        format: &FontFormat,
        content: T,
        image_read: R,
    ) -> Font {
        let bm_font;
        match BMFont::from_loaded(format, content.into(), &["image.png"]) {
            Ok(bmf) => bm_font = bmf,
            Err(error) => panic!("Failed to load .sfl file: {}", error),
        }

        Font::load_with_bmfont_and_image_read(&bm_font, image_read)
    }

    fn load_with_bmfont_and_image_read<R: Read>(bm_font: &BMFont, read: R) -> Font {
        let decoder = Decoder::new(read);
        let (info, mut reader) = decoder.read_info().unwrap();

        if info.color_type != ColorType::RGBA {
            panic!("Font color type is not RGBA");
        }

        let mut image_buffer = vec![0; info.buffer_size()];

        reader.next_frame(&mut image_buffer).unwrap();

        if image_buffer.len() != (info.width * info.height * 4) as usize {
            panic!("Font image is deformed");
        }

        // Load the font
        let mut characters = HashMap::<u16, CharacterData>::new();
        let width_float = info.width as f32;
        let height_float = info.height as f32;
        let mut min_off_y = 100_000;
        for (key, value) in bm_font.chars.iter() {
            let x1 = value.x as f32 / width_float;
            let x2 = (value.x as f32 + value.width as f32) / width_float;
            let y1 = value.y as f32 / height_float;
            let y2 = (value.y as f32 + value.height as f32) / height_float;
            if value.yoffset < min_off_y {
                min_off_y = value.yoffset;
            }

            characters.insert(
                *key as u16,
                CharacterData {
                    id: value.id,
                    x1,
                    x2,
                    y1,
                    y2,
                    width: value.width,
                    height: value.height,
                    x_off: value.xoffset,
                    y_off: value.yoffset,
                },
            );
        }

        Font {
            name: (&bm_font.font_name).clone(),
            image_buffer: image_buffer,
            width: info.width,
            height: info.height,
            line_height: bm_font.line_height,
            size: bm_font.size,
            min_offset_y: min_off_y,
            characters: characters,
        }
    }
    /// Gets the CharacterData from the Font with the given char, if the charcter exists, otherwise returns an error as a String. Example:
    ///
    /// ```
    /// use glerminal::{Font, FontFormat};
    /// let a_char_data = Font::load(&FontFormat::SFL, "fonts/iosevka.sfl").get_character('a' as u16);
    /// ```
    pub fn get_character(&self, character: u16) -> Result<CharacterData, String> {
        if let Some(character_data) = self.characters.get(&character) {
            Ok(character_data.clone())
        } else {
            Err(format!("Character not found: '{}'", character))
        }
    }
}
