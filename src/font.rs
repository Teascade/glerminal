use png::{ColorType, Decoder};
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;

use sfl_parser::BMFont;

#[derive(Clone)]
pub struct CharacterData {
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
    pub width: u32,
    pub height: u32,
    pub x_off: u32,
    pub y_off: u32,
}

pub struct Font {
    pub name: String,
    pub image_buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub line_height: u32,
    pub size: u32,
    pub max_offset_y: u32,
    pub max_offset_x: u32,
    characters: HashMap<u8, CharacterData>,
}

impl Font {
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
        let mut max_off_x = 0;
        let mut max_off_y = 0;
        for (key, value) in bm_font.chars.iter() {
            let x1 = value.x as f32 / width_float;
            let x2 = (value.x as f32 + value.width as f32) / width_float;
            let y1 = value.y as f32 / height_float;
            let y2 = (value.y as f32 + value.height as f32) / height_float;
            if value.xoffset > max_off_x {
                max_off_x = value.xoffset
            }
            if value.yoffset > max_off_y {
                max_off_y = value.yoffset
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
            max_offset_x: max_off_x + 2,
            max_offset_y: max_off_y + 2,
            characters: characters,
        }
    }

    pub fn get_character(&self, character: char) -> Result<CharacterData, String> {
        let character_code = character as u8;
        if let Some(character_data) = self.characters.get(&character_code) {
            Ok(character_data.clone())
        } else {
            Err(format!("Character not found: '{}'", character_code))
        }
    }
}
