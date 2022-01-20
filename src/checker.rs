use crate::Envful;
use serde_json;
use std::fs;

pub fn check(filename: String) {
    println!("Checking variables");
    // Read file
    let contents = fs::read_to_string(filename);

    if contents.is_ok() {
        let contents = contents.unwrap();
        let config: Envful = serde_json::from_str(contents.as_str()).unwrap();
        println!("{:?}", config);

        let env = fs::read_to_string("fixtures/.env").expect(".env file not found");
        let variables: Vec<&str> = env.split("\n").collect();
        println!("variables {:?}", variables);

        // Iterate variables
        for variable in variables {
            let var_string: Vec<&str> = variable.split("=").collect();
            let name = var_string[0];
            let value = var_string[1];
            println!("{} has value {}", name, value);
        }
    } else {
        println!("Envful manifest not found");
    }
}
