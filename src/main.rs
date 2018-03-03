
extern crate glutin;
extern crate gl;
extern crate libc;

mod display;

use display::Display;

fn main() {
    println!("Hello, world!");

    let mut display: Display = Display::new();

    while display.refresh() {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}
