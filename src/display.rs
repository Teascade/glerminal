use glutin::{ContextBuilder, ElementState, Event, EventsLoop, GlContext, GlWindow, WindowBuilder,
             WindowEvent};
use gl;

use renderer::{self, Matrix4};
use input::Input;
use std::cell::{Cell, RefCell};

pub struct Display {
    pub proj_matrix: Cell<Matrix4>,
    window: GlWindow,
    input: RefCell<Input>,
    events_loop: RefCell<EventsLoop>,
    aspect_ratio: f32,
}

impl Display {
    pub fn new<T: Into<String>>(
        title: T,
        dimensions: (u32, u32),
        clear_color: (f32, f32, f32, f32),
        visibility: bool,
    ) -> Display {
        let (width, height) = dimensions;
        let aspect_ratio = width as f32 / height as f32;
        let title = title.into();
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height)
            .with_visibility(visibility);
        let context = ContextBuilder::new().with_vsync(true);
        let gl_window;
        match GlWindow::new(window, context, &events_loop) {
            Ok(window) => gl_window = window,
            Err(err) => panic!(err),
        }

        unsafe {
            let (r, g, b, a) = clear_color;
            if let Err(err) = gl_window.make_current() {
                panic!(err);
            }
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::ClearColor(r, g, b, a);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        };

        let proj_matrix = renderer::create_proj_matrix((width as f32, height as f32), aspect_ratio);

        Display {
            window: gl_window,
            input: RefCell::new(Input::new()),
            events_loop: RefCell::new(events_loop),
            aspect_ratio: aspect_ratio,
            proj_matrix: Cell::new(proj_matrix),
        }
    }

    pub fn refresh(&self) -> bool {
        let mut running = true;

        let mut dimensions: Option<(u32, u32)> = None;

        let input = self.input.borrow_mut().clear_just_lists();
        drop(input);

        self.window.swap_buffers().unwrap();

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
                            self.input
                                .borrow_mut()
                                .update_virtual_keycode(keycode, state == ElementState::Pressed);
                        }
                    }
                    _ => (),
                },
                _ => (),
            });

        if let Some((width, height)) = dimensions {
            self.proj_matrix.set(renderer::create_proj_matrix(
                (width as f32, height as f32),
                self.aspect_ratio,
            ));
            renderer::update_viewport((width, height));
        }

        running
    }

    pub fn get_current_input(&self) -> Input {
        self.input.borrow().clone()
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }

    pub fn show(&mut self) {
        self.window.show();
    }
}
