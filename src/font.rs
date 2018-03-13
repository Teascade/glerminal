//! This module is used to load fonts that can be used in the [`TextBuffer`](text_buffer/struct.TextBuffer.html)
//!
//! The [`Font`](struct.Font.html) can be loaded from an `.sfl` file and then used in the `TextBuffer`, in example:
//! ```no_run
//! use glerminal::terminal::TerminalBuilder;
//! use glerminal::font::Font;
//!
//! let mut terminal = TerminalBuilder::new()
//!     .with_title("Hello glerminal::font::Font!")
//!     .with_dimensions((1280, 720))
//!     .with_font(Font::load("fonts/iosevka.sfl"))
//!     .build();
//! ```

use png::{ColorType, Decoder};
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;

use sfl_parser::BMFont;

/// Contains data of a single character in a Font
#[derive(Clone)]
pub struct CharacterData {
    pub(crate) x1: f32,
    pub(crate) x2: f32,
    pub(crate) y1: f32,
    pub(crate) y2: f32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) x_off: i32,
    pub(crate) y_off: i32,
}

/// Represents the font when it's loaded.
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
    pub(crate) max_offset_y: i32,
    pub(crate) min_offset_y: i32,
    characters: HashMap<u8, CharacterData>,
}

impl Font {
    /// Loads the font fron the given .sfl file, for example:
    ///
    /// ```
    /// use glerminal::font::Font;
    /// let font = Font::load("fonts/iosevka.sfl");
    /// ```
    pub fn load<T: Into<PathBuf>>(fnt_path: T) -> Font {
        let fnt_path = fnt_path.into();
        if !fnt_path.exists() {
            panic!("Font image or .sfl file missing");
        }

        // Load Font .sfl file
        let bm_font;
        match BMFont::from_path(fnt_path) {
            Ok(bmf) => bm_font = bmf,
            Err(error) => panic!("Failed to load .sfl file: {}", error),
        }

        // Load Font image file
        let decoder = Decoder::new(File::open(&bm_font.image_path).unwrap());
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
        let mut characters = HashMap::<u8, CharacterData>::new();
        let width_float = info.width as f32;
        let height_float = info.height as f32;
        let mut min_off_y = 100_000;
        let mut max_off_y = 0;
        for (key, value) in bm_font.chars.iter() {
            let x1 = value.x as f32 / width_float;
            let x2 = (value.x as f32 + value.width as f32) / width_float;
            let y1 = value.y as f32 / height_float;
            let y2 = (value.y as f32 + value.height as f32) / height_float;
            if value.yoffset > max_off_y {
                max_off_y = value.yoffset;
            }
            if value.yoffset < min_off_y {
                min_off_y = value.yoffset;
            }

            characters.insert(
                *key as u8,
                CharacterData {
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
            name: bm_font.font_name,
            image_buffer: image_buffer,
            width: info.width,
            height: info.height,
            line_height: bm_font.line_height,
            size: bm_font.size,
            min_offset_y: min_off_y,
            max_offset_y: max_off_y,
            characters: characters,
        }
    }

    /// Gets the CharacterData from the Font with the given char, if the charcter exists, otherwise returns an error as a String. Example:
    ///
    /// ```
    /// use glerminal::font::Font;
    /// let a_char_data = Font::load("fonts/iosevka.sfl").get_character('a');
    /// ```
    pub fn get_character(&self, character: char) -> Result<CharacterData, String> {
        let character_code = character as u8;
        if let Some(character_data) = self.characters.get(&character_code) {
            Ok(character_data.clone())
        } else {
            Err(format!("Character not found: '{}'", character_code))
        }
    }
}
