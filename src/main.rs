use std::env;

fn main() {
    println!("Envful!");
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let command = &args[1];
    println!("{}", command);
}
