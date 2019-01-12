#[allow(unused_imports)]
use glutin::VirtualKeyCode;
use std::cell::{Cell, RefCell};
use std::time::SystemTime;

use crate::display::Display;
use crate::events::Events;
use crate::font::Font;
use crate::renderer;
use crate::renderer::Program;
use crate::text_buffer::TextBuffer;

static IOSEVKA_SFL: &'static str = include_str!("../fonts/iosevka.sfl");
static IOSEVKA_PNG: &'static [u8] = include_bytes!("../fonts/iosevka.png");

/// A builder for the `Terminal`. Includes some settings that can be set before building.
///
/// See [terminal mod](index.html) for examples and more detailed documentation.
pub struct TerminalBuilder {
    pub(crate) title: String,
    pub(crate) dimensions: (u32, u32),
    pub(crate) clear_color: (f32, f32, f32, f32),
    pub(crate) font: Font,
    pub(crate) visibility: bool,
    pub(crate) headless: bool,
    pub(crate) text_buffer_aspect_ratio: bool,
    pub(crate) vsync: bool,
}

#[allow(dead_code)]
impl TerminalBuilder {
    /// Creates a new terminal builder with default settings.
    pub fn new() -> TerminalBuilder {
        TerminalBuilder {
            title: "Hello, World ! ".to_owned(),
            dimensions: (1280, 720),
            clear_color: (0.14, 0.19, 0.28, 1.0),
            font: Font::load_raw(IOSEVKA_SFL, IOSEVKA_PNG),
            visibility: true,
            headless: false,
            text_buffer_aspect_ratio: true,
            vsync: true,
        }
    }

    /// Sets the title for the `Terminal`.
    pub fn with_title<T: Into<String>>(mut self, title: T) -> TerminalBuilder {
        self.title = title.into();
        self
    }

    /// Sets the dimensions the `Terminal` is to be opened with.
    pub fn with_dimensions(mut self, dimensions: (u32, u32)) -> TerminalBuilder {
        self.dimensions = dimensions;
        self
    }

    /// Sets the clear color of the terminal.
    pub fn with_clear_color(mut self, clear_color: (f32, f32, f32, f32)) -> TerminalBuilder {
        self.clear_color = clear_color;
        self
    }

    /// Changes the font that the terminal uses.
    pub fn with_font(mut self, font: Font) -> TerminalBuilder {
        self.font = font;
        self
    }

    /// Changes the visibility that the terminal will be opened with. If headless, visibility will not matter.
    pub fn with_visibility(mut self, visibility: bool) -> TerminalBuilder {
        self.visibility = visibility;
        self
    }

    /// Changes the visibility that the terminal will be opened with
    pub fn with_headless(mut self, headless: bool) -> TerminalBuilder {
        self.headless = headless;
        self
    }

    /// Changes whether the aspect ratio should be retrieved from TextBuffer instead of the original resolution of the screen.
    ///
    /// If set to false, the aspect ratio used to make black bars for the screen will be fetched from the original resolution of the screen;
    /// This will cause the fonts to strech a bit though.
    ///
    /// If set to true (default), the aspect ratio will be fetched from the TextBuffer, causing almost any resolution
    /// to have black bars to make up for the missing spaces.
    pub fn with_text_buffer_aspect_ratio(mut self, tbar: bool) -> TerminalBuilder {
        self.text_buffer_aspect_ratio = tbar;
        self
    }

    /// Enable/Disable vsync. Enabled by default.
    pub fn with_vsync(mut self, vsync: bool) -> TerminalBuilder {
        self.vsync = vsync;
        self
    }

    /// Builds the actual terminal and opens the window
    pub fn build(self) -> Terminal {
        Terminal::new(self)
    }
}

/// The Terminal acts as the window and "canvas" of the terminal, handling most behind-the-sceneries
///
/// The Terminal is used to create the window and canvas for the [`TextBuffer`](text_buffer/struct.TextBuffer.html)
/// which can then draw it, close the window, reset the title of the window or handle events.
///
/// **Note** when building with debug-mode, you are able to press `F3` to toggle between debug and non-debug. see ([`set_debug`](#method.set_debug)) for more information.
///
/// ### Terminal example:
/// ```no_run
/// use glerminal::TerminalBuilder;
///
/// let terminal = TerminalBuilder::new()
///     .with_title("Hello GLerminal!")
///     .with_dimensions((1280, 720))
///     .build();
/// ```
///
/// ### `let mut terminal` vs `let terminal`
/// In most cases you might just want to initialize the terminal as immutable, but in some, you will need to initialize it as mutable,
/// allowing it to run some additional methods, such as `.show()` and `.set_title("title")`
///
/// #### Example of a mutable terminal:
/// ```no_run
/// use glerminal::TerminalBuilder;
///
/// let mut terminal = TerminalBuilder::new()
///     .with_title("Hello GLerminal!")
///     .with_dimensions((1280, 720))
///     .with_visibility(false)
///     .build();
///
/// terminal.set_title("Changed title!");
/// terminal.show();
/// ```
pub struct Terminal {
    display: Option<Display>,
    program: Program,
    background_program: Program,
    debug_program: Program,
    debug: Cell<bool>,
    running: Cell<bool>,
    pub(crate) headless: bool,
    since_start: SystemTime,
    pub(crate) font: Font,

    timer: RefCell<Timer>,
    text_buffer_aspect_ratio: bool,
}

impl Terminal {
    fn new(builder: TerminalBuilder) -> Terminal {
        let (display, program, background_program, debug_program) = if builder.headless {
            (None, Program::empty(), Program::empty(), Program::empty())
        } else {
            (
                Some(Display::new(
                    builder.title,
                    builder.dimensions,
                    builder.clear_color,
                    builder.visibility,
                    builder.text_buffer_aspect_ratio,
                    builder.vsync,
                )),
                renderer::create_program(renderer::VERT_SHADER, renderer::FRAG_SHADER),
                renderer::create_program(renderer::VERT_SHADER, renderer::BG_FRAG_SHADER),
                renderer::create_program(renderer::VERT_SHADER, renderer::DEBUG_FRAG_SHADER),
            )
        };
        Terminal {
            display,
            program,
            background_program,
            debug_program,
            debug: Cell::new(false),
            running: Cell::new(true),
            headless: builder.headless,
            since_start: SystemTime::now(),
            font: builder.font,
            timer: RefCell::new(Timer::new()),
            text_buffer_aspect_ratio: builder.text_buffer_aspect_ratio,
        }
    }

    /// Sets debug mode (changes characters and backgrounds into wireframe)
    pub fn set_debug(&self, debug: bool) {
        if !self.headless {
            renderer::set_debug(debug);
            self.debug.set(debug);
        }
    }

    /// Refreshes the screen and returns whether the while-loop should continue (is the program running)
    #[cfg(debug_assertions)]
    pub fn refresh(&self) -> bool {
        let mut timer = self.timer.borrow_mut();
        timer.update();
        drop(timer);

        let running = if let Some(ref display) = self.display {
            let events = self.get_current_events();
            if events.keyboard.was_just_pressed(VirtualKeyCode::F3) {
                self.set_debug(!self.debug.get());
            }
            display.refresh() && self.running.get()
        } else {
            self.running.get()
        };

        if running && !self.headless {
            renderer::clear();
        }
        running
    }

    /// Refreshes the screen and returns whether the while-loop should continue (is the program running)
    #[cfg(not(debug_assertions))]
    pub fn refresh(&self) -> bool {
        let mut timer = self.timer.borrow_mut();
        timer.update();
        drop(timer);

        let running = if let Some(ref display) = self.display {
            display.refresh() && self.running.get()
        } else {
            self.running.get()
        };

        if running && !self.headless {
            renderer::clear();
        }
        running
    }

    /// Flushes `TextBuffer`, taking it's character-grid and making it show for the next draw.
    ///
    /// This is quite a heavy function and it's calling should be avoided when unnecessary.
    pub fn flush(&self, text_buffer: &mut TextBuffer) {
        text_buffer.swap_buffers(&self.font);
    }

    /// Draws a `TextBuffer`. This should be called every frame for each text buffer.
    pub fn draw(&self, text_buffer: &TextBuffer) {
        if let (&Some(ref display), &Some(ref mesh), &Some(ref background_mesh)) = (
            &self.display,
            &text_buffer.mesh,
            &text_buffer.background_mesh,
        ) {
            let proj_matrix = if self.text_buffer_aspect_ratio {
                display.get_display_data(&text_buffer).proj_matrix
            } else {
                display.proj_matrix.get()
            };

            let duration = SystemTime::now().duration_since(self.since_start).unwrap();

            let time = duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0;

            renderer::draw(
                self.get_background_program(),
                proj_matrix,
                time,
                background_mesh,
            );
            renderer::draw(self.get_program(), proj_matrix, time, mesh);
        }
    }

    /// Gets the current Events, must be retrieved every time you want new events. (ie. every frame)
    pub fn get_current_events(&self) -> Events {
        if let Some(ref display) = self.display {
            display.get_current_events()
        } else {
            Events::new(self.text_buffer_aspect_ratio)
        }
    }

    /// Closes the Terminal
    pub fn close(&self) {
        self.running.set(false);
    }

    /// Sets the title for the window.
    ///
    /// **Warning:** This is a nuclear hazard (takes up a lot of performance), it might melt down your computer if called every frame (or so).
    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        if let Some(ref mut display) = self.display {
            display.set_title(&title.into());
        }
    }

    /// Shows the window, if it's hidden
    pub fn show(&mut self) {
        if let Some(ref mut display) = self.display {
            display.show();
        }
    }

    /// Get the delta-time (in seconds).
    pub fn delta_time(&self) -> f32 {
        self.timer.borrow().get_delta_time()
    }

    pub(crate) fn get_program(&self) -> Program {
        if self.headless {
            panic!("Unable to get program from headless terminal");
        }
        if !self.debug.get() {
            self.program
        } else {
            self.debug_program
        }
    }

    pub(crate) fn get_background_program(&self) -> Program {
        if self.headless {
            panic!("Unable to get program from headless terminal");
        }
        if !self.debug.get() {
            self.background_program
        } else {
            self.debug_program
        }
    }

    #[cfg(test)]
    pub(crate) fn update_virtual_keycode(&mut self, keycode: VirtualKeyCode, pressed: bool) {
        if let Some(ref mut display) = self.display {
            display.update_virtual_keycode(keycode, pressed);
        }
    }
}

pub(crate) struct Timer {
    last_check: SystemTime,
    delta_time: f32,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            last_check: SystemTime::now(),
            delta_time: 0.0,
        }
    }

    pub fn update(&mut self) {
        let current_time = SystemTime::now();
        let duration = current_time.duration_since(self.last_check).unwrap();
        self.last_check = current_time;

        self.delta_time =
            duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0;
    }

    pub fn get_delta_time(&self) -> f32 {
        self.delta_time
    }
}
