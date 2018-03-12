use std::collections::HashMap;

use regex::Regex;
use text_buffer::{Color, TextBuffer};

pub struct Parser {
    colors: HashMap<String, Color>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            colors: HashMap::<String, Color>::new(),
        }
    }

    pub fn add_color<T: Into<String>>(&mut self, color_str: T, color: Color) {
        self.colors.insert(color_str.into(), color);
    }

    pub fn write<T: Into<String>>(&self, text_buffer: &mut TextBuffer, text: T) {
        let text = text.into();

        let default_fg = text_buffer.get_cursor_fg_color();
        let default_bg = text_buffer.get_cursor_bg_color();
        let default_shakiness = text_buffer.get_cursor_shakiness();

        let regex = Regex::new(r"\[(/)?((fg|bg|shake)(=([A-z]+|\d+(\.\d+)?))?)\]").unwrap();
        let mut parts = regex.split(&text);

        for capture in regex.captures_iter(&text) {
            text_buffer.write(parts.next().unwrap());
            if let Some(target) = capture.get(3) {
                if let Some(_) = capture.get(1) {
                    if target.as_str() == "shake" {
                        text_buffer.set_cursor_shakiness(default_shakiness);
                    } else if target.as_str() == "fg" {
                        text_buffer.change_cursor_fg_color(default_fg);
                    } else if target.as_str() == "bg" {
                        text_buffer.change_cursor_bg_color(default_bg);
                    }
                }
                if let Some(value) = capture.get(5) {
                    if target.as_str() == "shake" {
                        let value = match value.as_str().parse::<f32>() {
                            Ok(val) => val,
                            Err(_) => panic!("Failed to parse shake-number"),
                        };
                        text_buffer.set_cursor_shakiness(value);
                    } else {
                        if let Some(color) = self.colors.get(value.as_str()) {
                            if target.as_str() == "fg" {
                                text_buffer.change_cursor_fg_color(*color);
                            } else {
                                text_buffer.change_cursor_bg_color(*color);
                            }
                        }
                    }
                }
            }
        }
        if let Some(last_part) = parts.next() {
            text_buffer.write(last_part);
        }

        text_buffer.change_cursor_fg_color(default_fg);
        text_buffer.change_cursor_bg_color(default_bg);
    }
}
