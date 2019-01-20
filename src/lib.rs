//! # Introduction
//! GLerminal is an OpenGL Terminal that is both easy to use and lightweight!
//!
//! This crate consists of two major parts that are exposed to the user:
//!
//! - [**terminal**](#terminal)
//! - [**text_buffer**](#text_buffer)
//!
//!
//! Also see [**Features**](#features) list to see what other exciting features exist.
//!
//! Starting with a simple [example](#example) might be wise though;
//!
//! After that you should browse through the rest of this guide and read about [`terminal`](terminal) and [`text_buffer`](text_buffer) documentations respectively
//!
//! # Example
//! ```no_run
//! use glerminal::{TerminalBuilder, TextBuffer};
//!
//! fn main() {
//!     let terminal = TerminalBuilder::new()
//!         .with_title("Hello GLerminal!")
//!         .with_dimensions((1280, 720))
//!         .build();
//!     let mut text_buffer;
//!     match TextBuffer::create(&terminal, (80, 24)) {
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
//! [`Terminal`](struct.Terminal.html) (and [`TerminalBuilder`](struct.TerminalBuilder.html)) will be the first struct(s) that you'll have to deal with:
//!
//! `TerminalBuilder` is used to build the terminal, and `Terminal` is used as sort of a 'window' or 'canvas' for everything else, like the [`TextBuffer`](#text_buffer).
//! The `Terminal` is able to change the title of the screen with [`.set_title`](struct.Terminal.html#method.set_title),
//! get events with [`.get_current_events`](struct.Terminal.html#method.get_current_events), draw the latest flush with [`.draw`](struct.Terminal.html#method.draw)
//! or (like mentioned earlier), flush the `TextBuffer` with [`.flush`](struct.Terminal.html#method.flush).
//!
//! Examples can be found at [`Terminal`](struct.Terminal.html) struct.
//!
//!
//! # text_buffer
//! [`TextBuffer`](struct.TextBuffer.html) is the struct that will be used the most.
//! With this struct you will be writing, clearing and changing colors of the text you are writing, for example.
//!
//! Another important struct that you might be using however, is the [`Parser`](struct.Parser.html).
//! With this struct you can simply pass a string that the parser will parse, and the `TextBuffer` will then change colors or shakiness of your text accordingly.
//!
//! Examples can be found at [`TextBuffer`](struct.TextBuffer.html) struct.
//!
//! # Features
//! There are two features for Glerminal that can be enabled (or disabled) to enable or disable wanted features.
//!
//! | Feature name             | What it enables                                             |
//! |--------------------------|-------------------------------------------------------------|
//! | parser _(on by default)_ | enables the [Praser](struct.Parser.html) struct.            |
//! | menu_systems             | enables the [menu_systems](menu_systems/index.html) module. |
#![warn(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::float_cmp)]

#[cfg(test)]
mod tests;

mod display;
mod events;
mod font;
mod renderer;
mod terminal;
mod text_buffer;

pub use crate::events::{Cursor, Events, Input};
pub use crate::font::{CharacterData, Font};
pub use crate::terminal::{Terminal, TerminalBuilder};
pub use crate::text_buffer::text_processing;
pub use crate::text_buffer::{Color, TermCharacter, TermCursor, TermLimits, TextBuffer, TextStyle};

#[cfg(feature = "parser")]
pub use crate::text_buffer::parser::Parser;

#[cfg(feature = "menu_systems")]
pub mod menu_systems;

pub use bmfont_parser::Format as FontFormat;
pub use glutin::MouseButton;
pub use glutin::VirtualKeyCode;
