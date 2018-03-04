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
}

pub struct Font {
    pub name: String,
    pub image_buffer: Vec<u8>,
    pub width: u32,
    pub height: u32,
    characters: HashMap<u8, CharacterData>,
}

impl Font {
    pub fn load<T: Into<PathBuf>>(image_path: T, fnt_path: T) -> Font {
        let image_path = image_path.into();
        let fnt_path = fnt_path.into();
        if !image_path.exists() || !fnt_path.exists() {
            panic!("Font image or .sfl file missing");
        }

        // Load Font image file
        let decoder = Decoder::new(File::open(&image_path).unwrap());
        let (info, mut reader) = decoder.read_info().unwrap();

        if info.color_type != ColorType::RGBA {
            panic!("Font color type is not RGBA");
        }

        let mut image_buffer = vec![0; info.buffer_size()];

        reader.next_frame(&mut image_buffer).unwrap();

        if image_buffer.len() != (info.width * info.height * 4) as usize {
            panic!("Font image is deformed");
        }

        // Load Font .fnt file
        let bm_font;
        match BMFont::load_and_parse(fnt_path) {
            Ok(bmf) => bm_font = bmf,
            Err(error) => panic!("Failed to load .sfl file: {}", error),
        }

        let mut characters = HashMap::<u8, CharacterData>::new();
        let width_float = info.width as f32;
        let height_float = info.height as f32;
        for (key, value) in bm_font.chars.iter() {
            let x1 = value.x as f32 / width_float;
            let x2 = (value.x as f32 + value.width as f32) / width_float;
            let y1 = value.y as f32 / height_float;
            let y2 = (value.y as f32 + value.height as f32) / height_float;

            characters.insert(*key as u8, CharacterData { x1, x2, y1, y2 });
        }

        Font {
            name: image_path.file_name().unwrap().to_str().unwrap().to_owned(),
            image_buffer: image_buffer,
            width: info.width,
            height: info.height,
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
