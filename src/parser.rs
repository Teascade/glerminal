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

        let regex = Regex::new(r"\[/?(fg|bg)(=([a-z]*))?\]").unwrap();
        let mut parts = regex.split(&text);

        for capture in regex.captures_iter(&text) {
            text_buffer.write(parts.next().unwrap());
            if let Some(target) = capture.get(1) {
                if let Some(color) = capture.get(3) {
                    if let Some(color) = self.colors.get(color.as_str()) {
                        self.change_color(
                            text_buffer,
                            target.as_str(),
                            Some(*color),
                            default_fg,
                            default_bg,
                        );
                    } else {
                        self.change_color(
                            text_buffer,
                            target.as_str(),
                            None,
                            default_fg,
                            default_bg,
                        );
                    }
                } else {
                    self.change_color(text_buffer, target.as_str(), None, default_fg, default_bg);
                }
            }
        }

        text_buffer.write(parts.next().unwrap());

        text_buffer.change_cursor_fg_color(default_fg);
        text_buffer.change_cursor_bg_color(default_bg);
    }

    fn change_color(
        &self,
        text_buffer: &mut TextBuffer,
        target: &str,
        color: Option<Color>,
        default_fg: Color,
        default_bg: Color,
    ) {
        if target == "fg" {
            let mut col = default_fg;
            if let Some(color) = color {
                col = color;
            }
            text_buffer.change_cursor_fg_color(col);
        } else {
            let mut col = default_bg;
            if let Some(color) = color {
                col = color;
            }
            text_buffer.change_cursor_bg_color(col);
        }
    }
}
