//! Text Processor is a trait that can be used in places where processing text may be necessary,
//! for example if using a `parser` is optional.
//!
//! Using the `DefaultProcessor` without intent on using any other TextProcessors, is very inefficient compared to just using `write`.

use crate::TextStyle;

/// The default processor that does nothing else, but take the text and apply the given style to each of those `char`s
pub struct DefaultProcessor;

impl TextProcessor for DefaultProcessor {
    fn process(&self, text: &str, style: TextStyle) -> Vec<ProcessedChar> {
        let mut list = Vec::new();
        for c in text.chars() {
            list.push(ProcessedChar {
                character: c,
                style: style,
            });
        }
        list
    }
}

/// A text processor that can take text, a style, and process them to produce some kind of output.
///
/// Usage of only the `DefaultProcessor` instead of regular `write` is very inefficient.
///
/// Primarily used in places where the Parser could be used, but isn't necessarily included in compilation,
/// but could be used in other places to process text in a wanted way.
pub trait TextProcessor {
    /// Process the given text with the given style and produce a list of `ProcessedChar`s
    fn process(&self, text: &str, style: TextStyle) -> Vec<ProcessedChar>;
}

/// A `char` that has been processed by a `TextProcessor`. Contains the `char` and it's style
#[derive(Debug, Clone, Copy)]
pub struct ProcessedChar {
    /// The character
    pub character: char,
    /// The style of this character
    pub style: TextStyle,
}
