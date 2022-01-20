use envful::checker;
use std::env;

fn main() {
    println!("Envful!");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let command = &args[1];
    println!("{}", command);

    match command.as_str() {
        "check" => checker::check(String::from("fixtures/envful.json")),
        _ => {
            println!("Command not recognized")
        }
    }
}
