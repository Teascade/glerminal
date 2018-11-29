//! The module that contains the `Parser`.

use std::collections::HashMap;

use super::{Color, TextBuffer};
use regex::Regex;

/// Represents a parser, that is able to read given texts and use [`TextBuffer`](struct.TextBuffer.html) accordingly, to write text and styles matching to the text.
///
/// See [TextBuffer](struct.TextBuffer.html) for examples and more detailed documentation.
pub struct Parser {
    colors: HashMap<String, Color>,
}

impl Parser {
    /// Creates a new Parser to use.
    pub fn new() -> Parser {
        Parser {
            colors: HashMap::<String, Color>::new(),
        }
    }

    /// Adds a color to the parser to use later.
    pub fn add_color<T: Into<String>>(&mut self, color_str: T, color: Color) {
        self.colors.insert(color_str.into(), color);
    }

    /// Parses the given text and makes it look according to the parsed text.
    ///
    /// Example:
    /// ```no_run
    /// use glerminal::{TerminalBuilder, TextBuffer, Parser};
    ///
    /// // Initialize a terminal to use
    /// let terminal = TerminalBuilder::new()
    ///     .with_title("Parser example!")
    ///     .with_dimensions((1280, 720))
    ///     .build();
    ///
    /// // Initialize a TextBuffer for the Parser
    /// let mut text_buffer;
    /// match TextBuffer::new(&terminal, (80, 24)) {
    ///   Ok(buffer) => text_buffer = buffer,
    ///   Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
    /// }
    ///
    /// // Test Parser
    /// let mut parser = Parser::new();
    /// parser.add_color("red", [1.0, 0.0, 0.0, 1.0]);
    /// parser.write(&mut text_buffer, "Hello, [fg=red]this color is red![/fg], [bg=red]this text has a red background[/bg] and [shake=0.5]this text shakes[/shake]");
    /// // Note: it is not necessary to close fg/bg/shake tags, parser will automatically revert colors in the TextBuffer.
    ///
    /// // Flush to "apply changes"
    /// terminal.flush(&mut text_buffer);
    /// ```
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
                        text_buffer.change_cursor_shakiness(default_shakiness);
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
                        text_buffer.change_cursor_shakiness(value);
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

    /// Gets the color specified, not compiled in a non-testing environment.
    #[cfg(test)]
    pub(crate) fn get_color(&self, color: String) -> Option<&Color> {
        self.colors.get(&color)
    }
}
