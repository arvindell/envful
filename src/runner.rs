use crate::checker;
use std::path::PathBuf;
use std::process::Command;

pub fn run_command(
    dir: &PathBuf,
    command: &Vec<String>,
    show_undeclared: bool,
    show_missing_optional: bool,
) {
    checker::check_command(dir, false, show_undeclared, show_missing_optional);
    // Run the command with the given args
    let binary = command.get(0).unwrap();
    let other_args = command
        .iter()
        .skip(1)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();
    let status = Command::new(binary).args(other_args).status();
    match status {
        Ok(code) => {
            std::process::exit(code.code().unwrap());
        }
        Err(e) => {
            // Check if e is of type file_not_found
            if e.kind() == std::io::ErrorKind::NotFound {
                eprintln!("{} Make sure the binary is in your PATH (not an alias)", e);
                std::process::exit(1);
            } else {
                panic!("{}", e);
            }
        }
    }
}
