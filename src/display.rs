use gl;
use glutin::{
    ContextBuilder, ElementState, Event, EventsLoop, GlContext, GlRequest, GlWindow, WindowBuilder,
    WindowEvent,
};

use events::Events;
use renderer::{self, Matrix4};
use std::cell::{Cell, RefCell};

#[cfg(test)]
use glutin::VirtualKeyCode;

pub struct Display {
    pub proj_matrix: Cell<Matrix4>,
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
            .with_vsync(true)
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

        let mut events = Events::new();
        events
            .cursor_position
            .update_overflows((width as f32, height as f32), aspect_ratio);

        Display {
            window: window,
            events: RefCell::new(events),
            events_loop: RefCell::new(events_loop),
            aspect_ratio: Cell::new(aspect_ratio),
            proj_matrix: Cell::new(proj_matrix),
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
                        self.events.borrow_mut().cursor_position.update_location((
                            position.0 as f32 / self.width.get() as f32,
                            position.1 as f32 / self.height.get() as f32,
                        ));
                    }
                    WindowEvent::CursorLeft { device_id: _ } => {
                        self.events.borrow_mut().cursor_position.cursor_left()
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

    pub(crate) fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio.get()
    }

    pub(crate) fn set_aspect_ratio(&self, aspect_ratio: f32) {
        self.aspect_ratio.set(aspect_ratio);
        self.update_view()
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
        self.events.borrow_mut().cursor_position.update_overflows(
            (self.width.get() as f32, self.height.get() as f32),
            self.aspect_ratio.get(),
        );
        renderer::update_viewport((self.width.get(), self.height.get()));
    }
}
