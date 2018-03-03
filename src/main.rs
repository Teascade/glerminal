
extern crate glutin;
extern crate gl;
extern crate libc;

mod display;
mod renderer;

use display::Display;

fn main() {
    println!("Hello, world!");

    let mut display: Display = Display::new("Hello");

    let vbo = renderer::create_vbo();
    let program = renderer::create_program();
    let vao = renderer::create_vao(program, vbo);
    while display.refresh() {
        renderer::clear();
        renderer::draw_triangle(program, vao);
    }
}
