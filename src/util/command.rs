use std::{env::current_dir, process::Command};

pub fn run_command_and_get_exit_code(command: &mut Command) -> Option<i32> {
    match current_dir() {
        Ok(x) => {
            let status = command.current_dir(x).status();
            status.ok().and_then(|status| status.code())
        }
        Err(_) => None,
    }
}
