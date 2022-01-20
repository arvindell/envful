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
        let lines: Vec<&str> = env.split("\n").collect();
        println!("variables {:?}", lines);

        // Iterate variables
        for line in lines {
            // If line is empty, skip
            if line.is_empty() {
                continue;
            }
            let var_string: Vec<&str> = line.split("=").collect();
            // Convert to String vector
            let var_string: Vec<String> = var_string.iter().map(|x| x.to_string()).collect();

            // Check name
            let name = var_string.get(0);
            if name.is_none() {
                println!("Name is not defined");
                continue;
            }
            let name = name.unwrap();
            if name.is_empty() {
                println!("Name is empty");
                continue;
            }

            // Check value
            let value = var_string.get(1);
            if value.is_none() {
                println!("Value is not present");
                continue;
            }
            let value = value.unwrap();
            if value.is_empty() {
                println!("value is empty");
                continue;
            }

            // Check if variable is defined in config
            if config.variables.contains(name) {
                println!("{} is declared!", name);
            } else {
                println!("{} is not declared", name);
            }
        }
    } else {
        println!("Envful manifest not found");
    }
}
