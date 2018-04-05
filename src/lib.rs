//! # Introduction
//! GLerminal is an OpenGL Terminal that is both easy to use and lightweight!
//!
//! This crate consists of two major parts that are exposed to the user:
//!
//! - [**terminal**](#terminal)
//! - [**text_buffer**](#text_buffer)
//!
//! Starting with a simple [example](#example) might be wise though;
//!
//! After that you should browse through the rest of this guide and read about [`terminal`](terminal) and [`text_buffer`](text_buffer) documentations respectively
//!
//! # Example
//! ```no_run
//! extern crate glerminal;
//!
//! use glerminal::terminal::TerminalBuilder;
//! use glerminal::text_buffer::TextBuffer;
//!
//! fn main() {
//!     let terminal = TerminalBuilder::new()
//!         .with_title("Hello GLerminal!")
//!         .with_dimensions((1280, 720))
//!         .build();
//!     let mut text_buffer;
//!     match TextBuffer::new(&terminal, (80, 24)) {
//!         Ok(buffer) => text_buffer = buffer,
//!         Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
//!     }
//!
//!     text_buffer.write("Hello, GLerminal!");
//!     terminal.flush(&mut text_buffer);
//!
//!     while terminal.refresh() {
//!         terminal.draw(&text_buffer);
//!     }
//! }
//! ```
//!
//! # terminal
//! [`Terminal`](terminal/struct.Terminal.html) (and [`TerminalBuilder`](terminal/struct.TerminalBuilder.html)) will be the first struct(s) that you'll have to deal with:
//!
//! `TerminalBuilder` is used to build the terminal, and `Terminal` is used as sort of a 'window' or 'canvas' for everything else, like the [`TextBuffer`](#text_buffer).
//! The `Terminal` is able to change the title of the screen with [`.set_title`](terminal/struct.Terminal.html#method.set_title),
//! get input with [`.get_input`](terminal/struct.Terminal.html#method.get_input), draw the latest flush with [`.draw`](terminal/struct.Terminal.html#method.draw)
//! or (like mentioned earlier), flush the `TextBuffer` with [`.flush`](terminal/struct.Terminal.html#method.flush).
//!
//! Examples can be found at [`terminal`](terminal) module.
//!
//!
//! # text_buffer
//! [`TextBuffer`](text_buffer/struct.TextBuffer.html) is the struct that will be used the most.
//! With this struct you will be writing, clearing and changing colors of the text you are writing, for example.
//!
//! Another important struct that you might be using however, is the [`Parser`](text_buffer/parser/struct.Parser.html).
//! With this struct you can simply pass a string that the parser will parse, and the `TextBuffer` will then change colors or shakiness of your text accordingly.
//!
//! Examples can be found at [`text_buffer`](text_buffer) module.
#![warn(missing_docs)]

#[cfg(test)]
extern crate rand;

extern crate gl;
extern crate glutin;
extern crate libc;
extern crate png;
extern crate sfl_parser;

#[cfg(feature = "parser")]
extern crate regex;

#[cfg(test)]
mod tests;

mod display;
mod renderer;
pub mod input;
pub mod font;
pub mod text_buffer;
pub mod terminal;

#[cfg(feature = "menu_systems")]
pub mod menu_systems;

pub use glutin::VirtualKeyCode;
