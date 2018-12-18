use super::test_setup_open_terminal;

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
