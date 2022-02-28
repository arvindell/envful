use crate::checker;
use std::path::PathBuf;
use std::process::Command;
use which::which;

pub fn run_command(
    file: Option<PathBuf>,
    manifest: &PathBuf,
    command: &Vec<String>,
    show_undeclared: bool,
    show_missing_optional: bool,
) {
    // Run the command with the given args
    checker::check_command(
        file,
        manifest,
        false,
        show_undeclared,
        show_missing_optional,
    );
    let binary = which(command.get(0).unwrap());
    if binary.is_err() {
        panic!("{}", binary.unwrap_err());
    }
    let binary = binary.unwrap();
    let other_args = command
        .iter()
        .skip(1)
        .map(|x| x.as_str())
        .collect::<Vec<&str>>();

    // Run the command with the given args
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
