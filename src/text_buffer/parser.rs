use std::collections::HashMap;

use super::{Color, TextBuffer};
use regex::Regex;

use crate::text_processing::{OptTextStyle, Processable, ProcessedChar, TextProcessor};

/// Represents a parser (A [`TextProcessor`](text_processing/struct.TextProcessor.html)), that is able to read given texts and use [`TextBuffer`](struct.TextBuffer.html) accordingly, to write text and styles matching to the text.
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
#[derive(Default, Debug, Clone)]
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
        text_buffer.write_processed(&self.parse(text));
    }

    /// Parse any text into a `ProcessedChar`s, alias for `parser.process`
    pub fn parse<T: Into<String>>(&self, text: T) -> Vec<ProcessedChar> {
        let text = text.into();

        self.process(vec![text.into()])
    }

    /// Gets the color specified, not compiled in a non-testing environment.
    #[cfg(test)]
    pub(crate) fn get_color(&self, color: &str) -> Option<&Color> {
        self.colors.get(color)
    }
}

impl TextProcessor for Parser {
    fn process(&self, processables: Vec<Processable>) -> Vec<ProcessedChar> {
        let mut fg_stack = Vec::new();
        let mut bg_stack = Vec::new();
        let mut shakiness_stack = Vec::new();
        let mut current_style = OptTextStyle {
            fg_color: None,
            bg_color: None,
            shakiness: None,
        };

        let regex = Regex::new(r"\[(/)?((fg|bg|shake)(=([A-z]+|\d+(\.\d+)?))?)\]").unwrap();

        let mut parsed = Vec::new();

        for processable in processables {
            match processable {
                Processable::ToProcess(text) => {
                    let mut parts = regex.split(&text);
                    for capture in regex.captures_iter(&text) {
                        parsed.push(ParsedText {
                            text: parts.next().unwrap().to_owned(),
                            style: current_style.clone(),
                        });

                        if let Some(target) = capture.get(3) {
                            if capture.get(1).is_some() {
                                if target.as_str() == "shake" {
                                    current_style.shakiness = shakiness_stack.pop();
                                } else if target.as_str() == "fg" {
                                    current_style.fg_color = fg_stack.pop();
                                } else if target.as_str() == "bg" {
                                    current_style.bg_color = bg_stack.pop();
                                }
                            }
                            if let Some(value) = capture.get(5) {
                                if target.as_str() == "shake" {
                                    let value = match value.as_str().parse::<f32>() {
                                        Ok(val) => val,
                                        Err(e) => panic!("Failed to parse shake-number: {}", e),
                                    };
                                    if let Some(shakiness) = current_style.shakiness {
                                        shakiness_stack.push(shakiness);
                                    }
                                    current_style.shakiness = Some(value);
                                } else if let Some(color) = self.colors.get(value.as_str()) {
                                    if target.as_str() == "fg" {
                                        if let Some(fg) = current_style.fg_color {
                                            fg_stack.push(fg);
                                        }
                                        current_style.fg_color = Some(*color);
                                    } else {
                                        if let Some(bg) = current_style.bg_color {
                                            bg_stack.push(bg);
                                        }
                                        current_style.bg_color = Some(*color);
                                    }
                                }
                            }
                        }
                    }
                    if let Some(last_part) = parts.next() {
                        parsed.push(ParsedText {
                            text: last_part.to_owned(),
                            style: current_style.clone(),
                        });
                    }
                }
                Processable::NoProcess(text) => {
                    parsed.push(ParsedText {
                        text: text,
                        style: current_style.clone(),
                    });
                }
            }
        }

        let mut list = Vec::new();
        for text in parsed {
            for character in text.text.chars() {
                list.push(ProcessedChar {
                    character: character,
                    style: text.style.clone(),
                });
            }
        }
        list
    }
}

#[derive(Clone, Debug)]
struct ParsedText {
    pub text: String,
    pub style: OptTextStyle,
}
