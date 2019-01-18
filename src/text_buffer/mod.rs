#[cfg(feature = "parser")]
pub mod parser;

use crate::font::Font;
use crate::renderer::backgroundmesh::BackgroundMesh;
use crate::renderer::textbuffermesh::TextBufferMesh;
use crate::terminal::Terminal;

use std::sync::atomic::{AtomicUsize, Ordering};

/// Represents a color with values from 0.0 to 1.0 (red, green, blue, alpha)
pub type Color = [f32; 4];

/// Represents a raw encoded character.
pub type RawCharacter = u16;

static INDEX_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// The `TextBuffer` acts as a "state machine" where you can set foreground color, background color and shakiness for the cursor,
/// move the cursor around, clear the screen and write with the cursor (using the cursor's styles) (through [`TermCursor`](struct.TermCursor.html)),
/// put, get characters or write strings.  
/// It's often the most efficient way to write things, especially if you have a very structured way of displaying things, but for a more simple-to-use
/// way of writing, that isn't as structured ie. for a dialogue, you might want to use the Parser.  
///
/// As of version 0.2.0, drawing multiple `TextBuffer`s on top of eachother is also possible.
/// As of this version writing 16-bit characters is also possible.
///
/// `Parser` is a struct added as a default feature, that is able to take in a piece of text and then parse it and change the cursor styles easily using the `TextBuffer`.
/// The `Parser` can handle tags imilar to BBCode tags, and can change fg, bg and shake, meaning the following tags are available to use mid-text:
/// - `[fg=color]`
/// - `[bg=color]`
/// - `[shake=decimal]`
/// - optional closing/style-resetting tags: `[/fg]`, `[/bg]` and `[/shake]`
///
/// The colors the `Parser` uses mid text must be pre-defined however with `add_color`.
///
/// ### Example usage of `TextBuffer`:
/// ```no_run
/// use glerminal::{TerminalBuilder, TextBuffer, TextStyle};
///
/// let terminal = TerminalBuilder::new()
///     .with_title("Hello GLerminal!")
///     .with_dimensions((1280, 720))
///     .build();
///
/// let mut text_buffer;
/// match TextBuffer::create(&terminal, (80, 24)) {
///   Ok(buffer) => text_buffer = buffer,
///   Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
/// }
///
/// // Test TextBuffer
/// text_buffer.cursor.style = TextStyle {
///     fg_color: [1.0, 0.0, 0.0, 1.0],
///     bg_color: [1.0; 4],
///     shakiness: 0.5,
/// };
/// text_buffer.cursor.move_to(0, 0);
/// text_buffer.write("This text is shaking in red in a white background!");
///
/// // Flush to "apply changes"
/// terminal.flush(&mut text_buffer);
/// ```
///
/// ### Example usage of `Parser`
/// ```no_run
/// use glerminal::{TerminalBuilder, TextBuffer, Parser};
///
/// let terminal = TerminalBuilder::new()
///     .with_title("Hello GLerminal!")
///     .with_dimensions((1280, 720))
///     .build();
///
/// let mut text_buffer;
/// match TextBuffer::create(&terminal, (80, 24)) {
///   Ok(buffer) => text_buffer = buffer,
///   Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
/// }
///
/// // Test Parser
/// let mut parser = Parser::new();
/// parser.add_color("red", [1.0, 0.0, 0.0, 1.0]);
/// parser.add_color("white", [1.0, 1.0, 1.0, 1.0]);
/// parser.write(&mut text_buffer, "[fg=red][bg=white][shake=1.0]This text is also shaking in red in a white background![/fg][/bg][/shake]");
/// // Note: it is not necessary to close fg/bg/shake tags, parser will automatically revert colors in the TextBuffer.
///
/// // Flush to "apply changes"
/// terminal.flush(&mut text_buffer);
/// ```
///
/// ### Simple example drawing multiple `TextBuffer`s
/// ```no_run
/// use glerminal::{TerminalBuilder, TextBuffer};
///
/// let terminal = TerminalBuilder::new().build();
///
/// let mut background_text_buffer;
/// match TextBuffer::create(&terminal, (80, 24)) {
///   Ok(buffer) => background_text_buffer = buffer,
///   Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
/// }
///
/// let mut foreground_text_buffer;
/// match TextBuffer::create(&terminal, (80, 24)) {
///   Ok(buffer) => foreground_text_buffer = buffer,
///   Err(error) => panic!(format!("Failed to initialize text buffer: {}", error)),
/// }
///
/// // Write to the background buffer
/// background_text_buffer.write("I am in the background");
///
/// // Write to the foreground buffer
/// foreground_text_buffer.write("I am in the foreground");
///
/// // Flush to "apply changes"
/// terminal.flush(&mut background_text_buffer);
/// terminal.flush(&mut foreground_text_buffer);
///
/// // Draw the text buffers
/// // The two texts will appear on top of eachother because they are written in the same location
/// terminal.draw(&background_text_buffer);
/// terminal.draw(&foreground_text_buffer);
/// ```
pub struct TextBuffer {
    index: u32,

    pub(crate) chars: Vec<TermCharacter>,
    pub(crate) height: u32,
    pub(crate) width: u32,
    pub(crate) mesh: Option<TextBufferMesh>,
    pub(crate) background_mesh: Option<BackgroundMesh>,

    pub(crate) aspect_ratio: f32,

    /// The cursor of the TextBuffer, specifies where characters are written and in what style.
    pub cursor: TermCursor,

    dirty: bool,
}

impl TextBuffer {
    /// Creates a new text buffer with the given dimensions (width in characters, height in characters)
    pub fn create(terminal: &Terminal, dimensions: (u32, u32)) -> Result<TextBuffer, String> {
        let (width, height) = dimensions;

        if width == 0 || height == 0 {
            return Err(
                "TextBuffer dimensions are erronous; either width or height is below 1".to_owned(),
            );
        }

        let chars =
            vec![TermCharacter::new(' ' as u16, Default::default()); (width * height) as usize];
        let (mesh, background_mesh) = if terminal.headless {
            (None, None)
        } else {
            (
                Some(TextBufferMesh::new(
                    terminal.get_program(),
                    dimensions,
                    &terminal.font,
                )),
                Some(BackgroundMesh::new(
                    terminal.get_background_program(),
                    dimensions,
                )),
            )
        };

        let true_height = height * terminal.font.line_height;
        let true_width = (width as f32 * terminal.font.average_xadvance) as u32;

        let index = INDEX_COUNTER.fetch_add(1, Ordering::Relaxed) as u32;
        Ok(TextBuffer {
            index: index,
            chars,
            height,
            width,
            mesh,
            background_mesh,
            cursor: TermCursor {
                x: 0,
                y: 0,
                style: Default::default(),
                limits: TermLimits::new(width, height),
            },

            aspect_ratio: true_width as f32 / true_height as f32,

            dirty: true,
        })
    }

    pub(crate) fn get_idx(&self) -> u32 {
        self.index
    }

    pub(crate) fn swap_buffers(&mut self, font: &Font) {
        if self.dirty {
            if let (&Some(ref mesh), &Some(ref background_mesh)) =
                (&self.mesh, &self.background_mesh)
            {
                mesh.update(&self, font);
                background_mesh.update(&self);
            }
            self.dirty = false;
        }
    }

    /// Get the dimensions of the text buffer (in characters). Returns (width, height)
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    /// Sets the character at the specified position. It is the user's responsibility to check if such a position exists.
    pub fn set_char(&mut self, x: u32, y: u32, character: TermCharacter) {
        self.chars[(y * self.width + x) as usize] = character;
    }

    /// Gets the TermChaacter in the given position
    ///
    /// Returns None if x/y are out of bounds
    pub fn get_character(&self, x: u32, y: u32) -> Option<TermCharacter> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.chars[(y * self.width + x) as usize])
        }
    }

    /// Clears the screen (makes every character empty and resets their style)
    pub fn clear(&mut self) {
        self.chars = vec![
            TermCharacter::new(' ' as u16, Default::default());
            (self.width * self.height) as usize
        ];
    }

    /// Puts a regular character to the current position of the cursor with the cursor's style
    pub fn put_char(&mut self, character: char) {
        if character.len_utf16() > 1 {
            panic!("Can not insert over 16-bit characters");
        } else {
            let mut bytes = [0; 1];
            character.encode_utf16(&mut bytes);

            self.put_raw_char(bytes[0]);
        }
    }

    /// Puts a raw 16-bit character to the current position of the cursor with the cursor's style (See text_buffer.cursor)
    pub fn put_raw_char(&mut self, character: RawCharacter) {
        self.chars[(self.cursor.y * self.width + self.cursor.x) as usize] =
            TermCharacter::new(character, self.cursor.style);
        self.cursor.move_by(1);

        self.dirty = true;
    }

    /// Puts the given text the same way as put_char
    pub fn write<T: Into<String>>(&mut self, text: T) {
        let text = text.into();
        for c in text.to_owned().encode_utf16() {
            self.put_raw_char(c);
        }
    }

    /// Returns the current position of the cursor
    pub fn get_cursor_position(&self) -> (u32, u32) {
        (self.cursor.x, self.cursor.y)
    }

    /// Returns whether the TextBuffer is dirty or not (whether flush will have any effect or not)
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}

/// Represents a style that can be used to style text.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextStyle {
    /// Foreground color of the text
    pub fg_color: Color,
    /// Background color of the text
    pub bg_color: Color,
    /// "Shakiness" of the text, meaning how much it shakes (safe values are between around -1.0 and 1.0)
    pub shakiness: f32,
}

impl Default for TextStyle {
    fn default() -> TextStyle {
        TextStyle {
            fg_color: [1.0; 4],
            bg_color: [0.0; 4],
            shakiness: 0.0,
        }
    }
}

/// Represents a single character in a [`TextBuffer`](struct.TextBuffer.html)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TermCharacter {
    character: RawCharacter,
    /// The styling of this character
    pub style: TextStyle,
}

impl TermCharacter {
    pub(crate) fn new(character: RawCharacter, style: TextStyle) -> TermCharacter {
        TermCharacter { character, style }
    }

    /// Gets the RawCharacter (u16-point) in the TermCharacter
    pub fn get_raw_char(&self) -> RawCharacter {
        self.character
    }

    /// Get the char in the TermCharacter
    pub fn get_char(&self) -> char {
        String::from_utf16(&[self.character]).unwrap().remove(0)
    }
}

/// The cursor on the TextBuffer that you can move around and change it's style.  
/// Determines where and with what style the TextBuffer writes characters
#[derive(Clone, Debug)]
pub struct TermCursor {
    x: u32,
    y: u32,
    /// The style of the cursor. Determines what style is used when writing characters.
    pub style: TextStyle,
    limits: TermLimits,
}

impl TermCursor {
    /// Set the limits for drawing, other than the current screen.
    /// None means no limit in this direction.
    pub fn set_limits(
        &mut self,
        x_min: Option<u32>,
        x_max: Option<u32>,
        y_min: Option<u32>,
        y_max: Option<u32>,
    ) {
        self.limits.x_min = x_min;
        self.limits.x_max = x_max;
        self.limits.y_min = y_min;
        self.limits.y_max = y_max;
    }

    /// Get the current term limits of the cursor.
    pub fn get_limits(&self) -> TermLimits {
        self.limits.clone()
    }

    /// Moves the cursor to a specified location in the terminal. If the location does not exist, nothing happens.
    pub fn move_to(&mut self, x: u32, y: u32) {
        let x = x
            .max(self.limits.get_min_x())
            .min(self.limits.get_max_x() - 1);
        let y = y
            .max(self.limits.get_min_y())
            .min(self.limits.get_max_y() - 1);
        self.x = x;
        self.y = y;
    }

    fn move_by(&mut self, amount: u32) {
        self.x += amount;
        if self.x >= self.limits.get_max_x() {
            self.x = self.limits.get_min_x();
            self.y += 1;
            if self.y >= self.limits.get_max_y() {
                self.y = self.limits.get_min_y();
            }
        }
    }
}

/// Represents the limits of the terminal.
#[derive(Clone, Debug)]
pub struct TermLimits {
    width: u32,
    height: u32,
    x_min: Option<u32>,
    x_max: Option<u32>,
    y_min: Option<u32>,
    y_max: Option<u32>,
}

impl TermLimits {
    fn new(width: u32, height: u32) -> TermLimits {
        TermLimits {
            width: width,
            height: height,
            x_min: None,
            x_max: None,
            y_min: None,
            y_max: None,
        }
    }

    /// Represents the smallest x-coordinate you should be able to write to (e.g. 0).
    pub fn get_min_x(&self) -> u32 {
        if let Some(x_min) = self.x_min {
            x_min.max(0)
        } else {
            0
        }
    }

    /// Represents the largest x-coordinate, where you should not be able to write anymore. (e.g. screen width)
    pub fn get_max_x(&self) -> u32 {
        if let Some(x_max) = self.x_max {
            x_max.min(self.width)
        } else {
            self.width
        }
    }

    /// Represents the smallest y-coordinate, where you should be able to write to (e.g. 0).
    pub fn get_min_y(&self) -> u32 {
        if let Some(y_min) = self.y_min {
            y_min.max(0)
        } else {
            0
        }
    }

    /// Represents the largest y-coordinate, where you should not be able to write anymore. (e.g. screen height)
    pub fn get_max_y(&self) -> u32 {
        if let Some(y_max) = self.y_max {
            y_max.min(self.height)
        } else {
            self.height
        }
    }
}
