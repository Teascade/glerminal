use super::{run_multiple_times, test_setup_open_terminal};
use terminal::Timer;
use rand;
use rand::distributions::{Range, Sample};
use std::time::Duration;
use std::thread;

#[test]
fn test_terminal_open_refresh_and_close() {
    let terminal = test_setup_open_terminal();
    while terminal.refresh() {
        terminal.close();
    }
}

#[test]
fn test_terminal_programs_debug_shaders() {
    let terminal = test_setup_open_terminal();
    if !terminal.headless {
        let program = terminal.get_program();
        let background_program = terminal.get_background_program();

        assert_ne!(program, background_program);

        terminal.set_debug(true);
        let debug_program = terminal.get_program();
        let debug_background_program = terminal.get_background_program();

        assert_eq!(debug_program, debug_background_program);
        assert_ne!(program, debug_program);
        assert_ne!(background_program, debug_background_program);
    }
}

#[test]
pub fn test_terminal_frame_counter() {
    run_multiple_times(10, || {
        let mut range = Range::new(1i32, 100);
        let mut rnd = rand::thread_rng();

        let target_fps = range.sample(&mut rnd);

        let mut frame_counter = Timer::new();
        for _ in 0..target_fps {
            frame_counter.update();
        }

        thread::sleep(Duration::from_secs(1));
        frame_counter.update();

        assert_eq!(frame_counter.get_fps(), target_fps as f32 + 1.0);
    })
}
