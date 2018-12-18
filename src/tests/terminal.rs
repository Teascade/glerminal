use super::test_setup_open_terminal;
use rand;
use rand::distributions::{Range, Sample};
use std::time::SystemTime;
use terminal::Timer;

#[test]
fn open_refresh_and_close() {
    let terminal = test_setup_open_terminal();
    while terminal.refresh() {
        terminal.close();
    }
}

#[test]
fn programs_debug_shaders() {
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
pub fn timer() {
    let mut range = Range::new(500i32, 900);
    let mut rnd = rand::thread_rng();

    let mut timer = Timer::new();

    let mut error_margin = 0.0;
    for _ in 0..100 {
        let start = SystemTime::now();
        let mut time = 0.0;
        timer.update();
        while time < 0.01 {
            timer.update();
            time += timer.get_delta_time();
        }
        timer.update();
        let duration = SystemTime::now().duration_since(start).unwrap();
        let curr = duration.subsec_nanos() as f32 / 1_000_000_000.0 - 0.01;
        if error_margin < curr {
            error_margin = curr;
        }
    }

    for _ in 0..20 {
        let time_to_wait = range.sample(&mut rnd) as f32 / 1000.0;
        let mut time_passed = 0.0;

        let start = SystemTime::now();
        timer.update();
        while time_passed < time_to_wait {
            timer.update();
            time_passed += timer.get_delta_time();
        }
        let duration = SystemTime::now().duration_since(start).unwrap();
        let difference =
            (duration.subsec_nanos() as f32 / 1_000_000_000.0 - time_to_wait).abs() - error_margin;
        assert!(difference < 0.05);
    }
}
