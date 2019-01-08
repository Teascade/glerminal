use gl;
use glutin::{
    ContextBuilder, ElementState, Event, EventsLoop, GlContext, GlRequest, GlWindow, WindowBuilder,
    WindowEvent,
};

use events::Events;
use renderer::{self, Matrix4};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use TextBuffer;

#[cfg(test)]
use glutin::VirtualKeyCode;

#[derive(Clone)]
pub struct TextBufferDisplayData {
    pub proj_matrix: Matrix4,
    pub aspect_ratio: f32,
    pub overflows: (f32, f32),
    pub relative_dimensions: (f32, f32),
}

impl TextBufferDisplayData {
    pub fn new(width: u32, height: u32, text_buffer: &TextBuffer) -> TextBufferDisplayData {
        let (overflows, relative_dimensions) =
            Display::calc_overflows_dimensions(width, height, text_buffer.aspect_ratio);
        TextBufferDisplayData {
            proj_matrix: renderer::create_proj_matrix(
                (width as f32, height as f32),
                text_buffer.aspect_ratio,
            ),
            aspect_ratio: text_buffer.aspect_ratio,
            overflows: overflows,
            relative_dimensions: relative_dimensions,
        }
    }
}

pub struct Display {
    pub proj_matrix: Cell<Matrix4>,
    display_datas: RefCell<HashMap<u32, TextBufferDisplayData>>,
    aspect_ratio: Cell<f32>,
    window: GlWindow,
    events: RefCell<Events>,
    events_loop: RefCell<EventsLoop>,
    width: Cell<u32>,
    height: Cell<u32>,
}

impl Display {
    pub fn new<T: Into<String>>(
        title: T,
        dimensions: (u32, u32),
        clear_color: (f32, f32, f32, f32),
        visibility: bool,
        text_buffer_aspect_ratio: bool,
        vsync: bool,
    ) -> Display {
        let (width, height) = dimensions;
        let aspect_ratio = width as f32 / height as f32;
        let title = title.into();
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height)
            .with_visibility(visibility);
        let context = ContextBuilder::new()
            .with_vsync(vsync)
            .with_gl(GlRequest::Latest);
        let window = match GlWindow::new(window, context, &events_loop) {
            Ok(window) => window,
            Err(err) => panic!(err),
        };

        unsafe {
            let (r, g, b, a) = clear_color;
            if let Err(err) = window.make_current() {
                panic!(err);
            }
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::ClearColor(r, g, b, a);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        };

        let gl_version = renderer::get_version();
        if !renderer::is_gl_version_compatible(gl_version.clone()) {
            panic!("GL version too low: OpenGL {}", gl_version);
        }

        let proj_matrix = renderer::create_proj_matrix((width as f32, height as f32), aspect_ratio);

        let mut events = Events::new(text_buffer_aspect_ratio);
        let (display_overflows, display_relative_dimensions) =
            Display::calc_overflows_dimensions(width, height, aspect_ratio);
        events.cursor.update_display_datas(
            display_overflows,
            display_relative_dimensions,
            HashMap::new(),
        );

        Display {
            window: window,
            events: RefCell::new(events),
            events_loop: RefCell::new(events_loop),
            aspect_ratio: Cell::new(aspect_ratio),
            proj_matrix: Cell::new(proj_matrix),
            display_datas: RefCell::new(HashMap::new()),
            width: Cell::new(width),
            height: Cell::new(height),
        }
    }

    pub fn refresh(&self) -> bool {
        let mut running = true;

        let mut dimensions: Option<(u32, u32)> = None;

        let events = self.events.borrow_mut().clear_just_lists();
        drop(events);

        self.window.swap_buffers().ok();

        self.events_loop
            .borrow_mut()
            .poll_events(|event| match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Closed => {
                        running = false;
                    }
                    WindowEvent::Resized(width, height) => {
                        dimensions = Some((width, height));
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let (state, Some(keycode)) = (input.state, input.virtual_keycode) {
                            self.events
                                .borrow_mut()
                                .keyboard
                                .update_button_press(keycode, state == ElementState::Pressed);
                        }
                    }
                    WindowEvent::MouseInput { button, state, .. } => self
                        .events
                        .borrow_mut()
                        .mouse
                        .update_button_press(button, state == ElementState::Pressed),
                    WindowEvent::CursorMoved { position, .. } => {
                        self.events.borrow_mut().cursor.update_location((
                            position.0 as f32 / self.width.get() as f32,
                            position.1 as f32 / self.height.get() as f32,
                        ));
                    }
                    WindowEvent::CursorLeft { device_id: _ } => {
                        self.events.borrow_mut().cursor.cursor_left()
                    }
                    _ => (),
                },
                _ => (),
            });

        if let Some((width, height)) = dimensions {
            self.width.set(width);
            self.height.set(height);
            self.update_view();
        }

        running
    }

    pub fn get_current_events(&self) -> Events {
        self.events.borrow().clone()
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn show(&mut self) {
        self.window.show();
    }

    pub(crate) fn get_display_data(&self, text_buffer: &TextBuffer) -> TextBufferDisplayData {
        let mut display_datas = self.display_datas.borrow_mut();
        if !display_datas.contains_key(&text_buffer.get_idx()) {
            display_datas.insert(
                text_buffer.get_idx(),
                TextBufferDisplayData::new(self.width.get(), self.height.get(), &text_buffer),
            );

            self.update_event_display_datas(display_datas.clone());
        }
        display_datas.get(&text_buffer.get_idx()).unwrap().clone()
    }

    #[cfg(test)]
    pub(crate) fn update_virtual_keycode(&mut self, keycode: VirtualKeyCode, pressed: bool) {
        self.events
            .borrow_mut()
            .keyboard
            .update_button_press(keycode, pressed);
    }

    fn update_view(&self) {
        self.proj_matrix.set(renderer::create_proj_matrix(
            (self.width.get() as f32, self.height.get() as f32),
            self.aspect_ratio.get(),
        ));

        for data in self.display_datas.borrow_mut().values_mut() {
            data.proj_matrix = renderer::create_proj_matrix(
                (self.width.get() as f32, self.height.get() as f32),
                data.aspect_ratio,
            );

            let (overflows, relative_dimensions) = Display::calc_overflows_dimensions(
                self.width.get(),
                self.height.get(),
                data.aspect_ratio,
            );
            data.overflows = overflows;
            data.relative_dimensions = relative_dimensions;
        }

        self.update_event_display_datas(self.display_datas.borrow().clone());

        renderer::update_viewport((self.width.get(), self.height.get()));
    }

    fn update_event_display_datas(&self, datas: HashMap<u32, TextBufferDisplayData>) {
        let (display_overflows, display_relative_dimensions) = Display::calc_overflows_dimensions(
            self.width.get(),
            self.height.get(),
            self.aspect_ratio.get(),
        );

        self.events.borrow_mut().cursor.update_display_datas(
            display_overflows,
            display_relative_dimensions,
            datas,
        );
    }

    fn calc_overflows_dimensions(
        width: u32,
        height: u32,
        aspect_ratio: f32,
    ) -> ((f32, f32), (f32, f32)) {
        let width = width as f32;
        let height = height as f32;

        let true_width = height * aspect_ratio;
        let true_height = width / aspect_ratio;

        let mut overflow_width = 0f32;
        let mut overflow_height = 0f32;
        let mut relative_width = 1.0;
        let mut relative_height = 1.0;
        if true_width < width {
            overflow_width = (width - true_width) / width;
            relative_width = width / true_width;
        } else {
            overflow_height = (height - true_height) / height;
            relative_height = height / true_height;
        }

        let overflows = (overflow_width / 2.0, overflow_height / 2.0);
        let relative_dimensions = (relative_width, relative_height);
        (overflows, relative_dimensions)
    }
}
