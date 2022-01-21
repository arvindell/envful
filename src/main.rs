use envful::checker;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];
    let dir = &args[2];
    match command.as_str() {
        "check" => checker::check(String::from(dir) + "envful.json"),
        _ => {
            println!("Command not recognized")
        }
    }
}
