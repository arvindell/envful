use crate::checker;
use std::path::PathBuf;
use std::process::Command;

pub fn run(dir: &PathBuf, command: &Vec<String>) {
    checker::check(dir, false, true);
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
            panic!("{}", e);
        }
    }
}
