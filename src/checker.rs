use crate::Envful;
use colored::*;
use serde_json;
use std::fs;

pub fn check(filename: String) {
    println!("Checking variables");
    // Read file
    println!("{}", filename);
    let contents = fs::read_to_string(filename);

    if contents.is_ok() {
        let contents = contents.unwrap();
        let config: Envful = serde_json::from_str(contents.as_str()).unwrap();

        let env = fs::read_to_string("fixtures/.env").expect(".env file not found");
        let lines: Vec<&str> = env.split("\n").collect();

        let mut given_vars: Vec<String> = Vec::new();

        // Iterate variables
        for line in lines {
            // If line is empty, skip
            if line.is_empty() {
                continue;
            }
            // Convert to String vector
            let var_string: Vec<String> = line.split("=").map(|x| x.to_string()).collect();

            // Check name
            let name = var_string.get(0).clone();
            if name.is_none() {
                println!("Name is not defined");
                continue;
            }
            let name = name.unwrap().clone();
            if name.is_empty() {
                println!("Name is empty");
                continue;
            }

            // Check value
            let value = var_string.get(1).clone();
            if value.is_none() {
                println!("Value is not present");
                continue;
            }
            let value = value.unwrap().clone();
            if value.is_empty() {
                println!("Value is empty");
                continue;
            }
            given_vars.push(name);
        }

        let mut error = false;
        for declared_var in config.variables {
            if !given_vars.contains(&declared_var) {
                println!(
                    "{} {}",
                    "❌ Missing variable:".bold().yellow(),
                    declared_var.red()
                );
                error = true;
            }
        }
        if error {
            // Exit with error code
            std::process::exit(1);
        } else {
            println!("{}", "All variables are present ✅".green());
            std::process::exit(0);
        }
    } else {
        println!("Envful manifest not found");
    }
}
