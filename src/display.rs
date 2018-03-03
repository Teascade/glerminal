
use glutin::{ContextBuilder, DeviceId, ElementState, Event, EventsLoop, GlContext, GlWindow,
             KeyboardInput, VirtualKeyCode, WindowBuilder, WindowEvent};
use gl;

pub struct Display {
    window: GlWindow,
    events_loop: EventsLoop,
}

impl Display {
    pub fn new<T: Into<String>>(title: T) -> Display {
        let title = title.into();
        let events_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(1280, 720);
        let context = ContextBuilder::new()
            .with_vsync(true);
        let gl_window;
        match GlWindow::new(window, context, &events_loop) {
            Ok(window) => gl_window = window,
            Err(err) => panic!(err)
        }

        unsafe {
            if let Err(err) = gl_window.make_current() {
                panic!(err);
            }
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
            gl::ClearColor(0.5, 0.3, 0.7, 1.0);
        }

        Display {
            window: gl_window,
            events_loop: events_loop,
        }
    }

    pub fn refresh(&mut self) -> bool {
        let mut running = true;

        self.window.swap_buffers().unwrap();

        self.events_loop.poll_events(|event|
            match event {
                Event::WindowEvent {event, ..} => {
                    match event {
                        WindowEvent::Closed  =>  {
                            running = false;
                        }
                        _ => ()
                    }
                },
                _ => ()
            }
        );
        running
    }
}
