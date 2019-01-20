//! Text Processor is a trait that can be used in places where processing text may be necessary,
//! for example if using a `parser` is optional.
//!
//! Using the `DefaultProcessor` without intent on using any other TextProcessors, is very inefficient compared to just using `write`.

use crate::Color;

/// The default processor that does nothing else, but take the text and apply the given style to each of those `char`s
#[derive(Debug, Clone, Copy)]
pub struct DefaultProcessor;

impl TextProcessor for DefaultProcessor {
    fn process(&self, processables: Vec<Processable>) -> Vec<ProcessedChar> {
        let mut list = Vec::new();
        let none_style = OptTextStyle {
            fg_color: None,
            bg_color: None,
            shakiness: None,
        };
        for processable in processables {
            let text = match processable {
                Processable::ToProcess(text) => text,
                Processable::NoProcess(text) => text,
            };
            for c in text.chars() {
                list.push(ProcessedChar {
                    character: c,
                    style: none_style.clone(),
                });
            }
        }
        list
    }
}

/// A string that can be given for a TextProcessor.
pub enum Processable {
    /// A String that will be processed when given to a processor
    ToProcess(String),
    /// A String that will be not be processed when given to a processor,
    /// meaning the processor will keep it's previously determined style and insert only the chars
    NoProcess(String),
}

impl From<String> for Processable {
    fn from(item: String) -> Processable {
        Processable::ToProcess(item)
    }
}

impl From<&'static str> for Processable {
    fn from(item: &'static str) -> Processable {
        Processable::ToProcess(item.to_owned())
    }
}

/// A text processor that can take text, a style, and process them to produce some kind of output.
///
/// Usage of only the `DefaultProcessor` instead of regular `write` is very inefficient.
///
/// Primarily used in places where the Parser could be used, but isn't necessarily included in compilation,
/// but could be used in other places to process text in a wanted way.
pub trait TextProcessor {
    /// Process the given processables with the given style and produce a list of `ProcessedChar`s
    /// Strings and &'static str have From and Into for Processable so the following is possible:
    /// `processor.process(vec!("something".into()));`
    fn process(&self, processables: Vec<Processable>) -> Vec<ProcessedChar>;
}

/// A `char` that has been processed by a `TextProcessor`. Contains the `char` and it's style
#[derive(Debug, Clone)]
pub struct ProcessedChar {
    /// The character
    pub character: char,
    pub(crate) style: OptTextStyle,
}

#[derive(Debug, Clone)]
pub(crate) struct OptTextStyle {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub shakiness: Option<f32>,
}
