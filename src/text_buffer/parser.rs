use std::collections::HashMap;

use super::{Color, TextBuffer, TextStyle};
use regex::Regex;
use std::convert::From;
use std::iter::{IntoIterator, Iterator};
use std::vec::IntoIter;

/// Represents a parser, that is able to read given texts and use [`TextBuffer`](struct.TextBuffer.html) accordingly, to write text and styles matching to the text.
///
///**Note:** This struct requires _parser_ feature to be enabled.
///
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
/// match TextBuffer::create(&terminal, (80, 24)) {
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
///
/// See [TextBuffer](struct.TextBuffer.html) for examples and more detailed documentation.
#[derive(Default)]
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

    /// Parses the given text and immediately writes it to the text buffer
    pub fn write<T: Into<String>>(&self, text_buffer: &mut TextBuffer, text: T) {
        Parser::write_parsed(text_buffer, self.parse(text, text_buffer.cursor.style));
    }

    /// Writes any parsed text (for example the struct that is passed from `parser.parse`), or just a list of `ParsedChar`s
    pub fn write_parsed<T: IntoIterator<Item = ParsedChar>>(
        text_buffer: &mut TextBuffer,
        parsed: T,
    ) {
        let mut curr_style = Default::default();
        for character in parsed {
            if character.style != curr_style {
                curr_style = character.style;
                text_buffer.cursor.style = curr_style;
            }
            text_buffer.put_char(character.character);
        }
    }

    /// Parse any text into a `ParsedText` struct
    pub fn parse<T: Into<String>>(&self, text: T, default_style: TextStyle) -> ParsedText {
        let text = text.into();

        let default_fg = default_style.fg_color;
        let default_bg = default_style.bg_color;
        let default_shakiness = default_style.shakiness;
        let mut fg_stack = Vec::new();
        let mut bg_stack = Vec::new();
        let mut shakiness_stack = Vec::new();
        let mut current_style = default_style;

        let regex = Regex::new(r"\[(/)?((fg|bg|shake)(=([A-z]+|\d+(\.\d+)?))?)\]").unwrap();
        let mut parts = regex.split(&text);

        let mut parsed = ParsedText { list: Vec::new() };

        for capture in regex.captures_iter(&text) {
            parsed.list.push(ParsedTextPart {
                text: parts.next().unwrap().to_owned(),
                style: current_style,
            });

            if let Some(target) = capture.get(3) {
                if capture.get(1).is_some() {
                    if target.as_str() == "shake" {
                        current_style.shakiness =
                            shakiness_stack.pop().unwrap_or(default_shakiness);
                    } else if target.as_str() == "fg" {
                        current_style.fg_color = fg_stack.pop().unwrap_or(default_fg);
                    } else if target.as_str() == "bg" {
                        current_style.bg_color = bg_stack.pop().unwrap_or(default_bg);
                    }
                }
                if let Some(value) = capture.get(5) {
                    if target.as_str() == "shake" {
                        let value = match value.as_str().parse::<f32>() {
                            Ok(val) => val,
                            Err(e) => panic!("Failed to parse shake-number: {}", e),
                        };
                        shakiness_stack.push(current_style.shakiness);
                        current_style.shakiness = value;
                    } else if let Some(color) = self.colors.get(value.as_str()) {
                        if target.as_str() == "fg" {
                            fg_stack.push(current_style.fg_color);
                            current_style.fg_color = *color;
                        } else {
                            bg_stack.push(current_style.bg_color);
                            current_style.bg_color = *color;
                        }
                    }
                }
            }
        }
        if let Some(last_part) = parts.next() {
            parsed.list.push(ParsedTextPart {
                text: last_part.to_owned(),
                style: current_style,
            });
        }

        parsed
    }

    /// Gets the color specified, not compiled in a non-testing environment.
    #[cfg(test)]
    pub(crate) fn get_color(&self, color: &str) -> Option<&Color> {
        self.colors.get(color)
    }
}

/// Represents a text that has been parsed, consisting of `ParsedTextPart`s
pub struct ParsedText {
    /// The list of `ParsedTextPart`s
    pub list: Vec<ParsedTextPart>,
}

impl From<ParsedText> for Vec<ParsedChar> {
    fn from(text: ParsedText) -> Vec<ParsedChar> {
        let mut list: Vec<ParsedChar> = Vec::new();
        for part in text.list {
            list.append(&mut Vec::from(part));
        }
        list
    }
}

impl IntoIterator for ParsedText {
    type Item = ParsedChar;
    type IntoIter = IntoIter<ParsedChar>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self).into_iter()
    }
}

/// Represents any given number of consecutive ParsedChars that were parsed in `parser.parse` that weren't interrupted by style changes
#[derive(Clone, Debug)]
pub struct ParsedTextPart {
    /// The text
    pub text: String,
    /// The style this text is in
    pub style: TextStyle,
}

impl From<ParsedTextPart> for Vec<ParsedChar> {
    fn from(word: ParsedTextPart) -> Vec<ParsedChar> {
        let mut list = Vec::new();
        for character in word.text.chars() {
            list.push(ParsedChar {
                character: character,
                style: word.style,
            });
        }
        list
    }
}

impl IntoIterator for ParsedTextPart {
    type Item = ParsedChar;
    type IntoIter = IntoIter<ParsedChar>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(self).into_iter()
    }
}

/// A single parsed character from a [`ParsedText`](struct.ParsedText.html)
#[derive(Clone, Debug)]
pub struct ParsedChar {
    /// The character
    pub character: char,
    /// The style this character is in
    pub style: TextStyle,
}
