use glutin::{ContextBuilder, DeviceId, ElementState, Event, EventsLoop, GlContext, GlWindow,
             KeyboardInput, VirtualKeyCode, WindowBuilder, WindowEvent};
use gl;

use renderer;
use renderer::Matrix4;

pub struct Display {
    pub proj_matrix: Matrix4,
    window: GlWindow,
    events_loop: EventsLoop,
    aspect_ratio: f32,
}

impl Display {
    pub fn new<T: Into<String>>(title: T, dimensions: (u32, u32)) -> Display {
        let (width, height) = dimensions;
        let aspect_ratio = width as f32 / height as f32;
        let title = title.into();
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(width, height);
        let context = ContextBuilder::new().with_vsync(true);
        let gl_window;
        match GlWindow::new(window, context, &events_loop) {
            Ok(window) => gl_window = window,
            Err(err) => panic!(err),
        }

        unsafe {
            if let Err(err) = gl_window.make_current() {
                panic!(err);
            }
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.5, 0.3, 0.7, 1.0);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        };

        let proj_matrix = renderer::create_proj_matrix((width as f32, height as f32), aspect_ratio);

        Display {
            window: gl_window,
            events_loop: events_loop,
            aspect_ratio: aspect_ratio,
            proj_matrix: proj_matrix,
        }
    }

    pub fn refresh(&mut self) -> bool {
        let mut running = true;

        let mut dimensions: Option<(u32, u32)> = None;

        self.window.swap_buffers().unwrap();

        self.events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Closed => {
                    running = false;
                }
                WindowEvent::Resized(width, height) => {
                    dimensions = Some((width, height));
                }
                _ => (),
            },
            _ => (),
        });

        if let Some((width, height)) = dimensions {
            self.proj_matrix =
                renderer::create_proj_matrix((width as f32, height as f32), self.aspect_ratio);
            renderer::update_viewport((width, height));
        }

        running
    }

    pub fn set_title(&mut self, title: &str) {
        self.window.set_title(title);
    }
}
