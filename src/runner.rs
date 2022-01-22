use crate::checker;
use std::path::PathBuf;
use std::process::Command;

pub fn run(dir: &PathBuf, command: &str) {
    checker::check(dir, false);
    // Run the command with the given args
    let binary = command.split_whitespace().next().unwrap();
    let other_args = command.split_whitespace().skip(1).collect::<Vec<&str>>();
    let status = Command::new(binary).args(other_args).status();
    match status {
        Ok(code) => {
            std::process::exit(code.code().unwrap());
        }
        Err(_e) => {
            std::process::exit(1);
        }
    }
}
