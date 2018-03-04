use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use std::io::Read;

pub struct BMCharacter {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct BMFont {
    pub font_name: String,
    pub chars: HashMap<u32, BMCharacter>,
}

impl BMFont {
    pub fn load_and_parse<T: Into<PathBuf>>(path: T) -> Result<BMFont, String> {
        let path = path.into();
        let mut file;
        match File::open(&path) {
            Ok(f) => file = f,
            Err(error) => return Err(format!("Error while loading .sfl file: {}", error)),
        }

        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        let mut lines = buffer.lines();

        if lines.clone().count() < 5 {
            return Err("Erronous .sfl file; too few lines to initialize.".to_owned());
        }

        let font_name = lines.next().unwrap().to_owned();
        let mut lines = lines.skip(2); // Skip two lines, nothing interesting there.
        let character_amount;
        match lines.next().unwrap().to_owned().parse::<u32>() {
            Ok(amount) => character_amount = amount,
            Err(_) => return Err(format!("Error while parsing character amount at line: 4")),
        }

        if lines.clone().count() + 5 < 5 + character_amount as usize {
            return Err(format!("Erronous .sfl file; character amount (line 4) does not match actual character amount; is {}, should be {}", lines.count() + 5, 5 + character_amount));
        }

        let mut chars = HashMap::<u32, BMCharacter>::new();
        for i in 0..character_amount {
            let character = BMFont::read_character(lines.next().unwrap().to_owned(), i + 1);
            match character {
                Ok(ch) => chars.insert(ch.id, ch),
                Err(error) => return Err(error),
            };
        }

        return Ok(BMFont { font_name, chars });
    }

    fn read_character(line: String, line_number: u32) -> Result<BMCharacter, String> {
        let mut parts = line.split(' ');
        if parts.clone().count() < 8 {
            return Err(format!(
                "Too few parts in character at line: {}",
                line_number
            ));
        }

        let mut numbers: Vec<u32> = vec![0; 5];
        for i in 0..5 {
            match parts.nth(0).unwrap().parse::<u32>() {
                Ok(number) => numbers[i] = number,
                Err(_) => {
                    return Err(format!(
                        "Error while parsing number at line: {}",
                        line_number
                    ));
                }
            }
        }

        Ok(BMCharacter {
            id: numbers[0],
            x: numbers[1],
            y: numbers[2],
            width: numbers[3],
            height: numbers[4],
        })
    }
}
