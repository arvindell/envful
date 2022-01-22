use crate::EnvVar;
use crate::Envful;
use colored::*;
use serde_json;
use std::path::PathBuf;

use std::fs;

pub fn check(dir: &PathBuf, show_undeclared: bool) {
    println!("{}", "Checking env vars...".cyan());

    let manifest_path = dir.clone().join("envful.json");
    let env_file_path = dir.clone().join(".env");
    let config: Envful = get_config(&manifest_path);
    // Get name of vars from EnvVar vector
    let given_vars = parse_env_file(&env_file_path)
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>();
    // Get missing vars from config
    let missing_vars: Vec<String> = config
        .variables
        .iter()
        .filter(|x| !given_vars.contains(x))
        .map(|x| x.clone())
        .collect();
    let undeclared_vars: Vec<String> = given_vars
        .iter()
        .filter(|x| !config.variables.contains(x))
        .map(|x| x.clone())
        .collect();
    let error = missing_vars.len() > 0;

    if show_undeclared {
        if undeclared_vars.len() > 0 {
            println!(
                "{}",
                "Found variables not declared in the manifest:"
                    .yellow()
                    .bold()
            );
            for undeclared_var in undeclared_vars {
                println!(
                    "{} {}",
                    " Undeclared variable:".yellow(),
                    undeclared_var.yellow()
                );
            }
        }
    }

    if error {
        // Print message for every missing var
        println!(
            "{}",
            "The process is missing environment variables:".red().bold()
        );
        for missing_var in missing_vars {
            println!(
                "{} {}",
                "❌ Missing variable:".yellow(),
                missing_var.yellow()
            );
        }
        // Exit with error code
        std::process::exit(1);
    }
    println!("{}", "All variables are present ✅".green());
}

fn get_config(path: &PathBuf) -> Envful {
    let contents = fs::read_to_string(path);

    if contents.is_ok() {
        let contents = contents.unwrap();
        let config: Envful = serde_json::from_str(contents.as_str()).unwrap();
        return config;
    } else {
        println!("Envful manifest not found");
        std::process::exit(1);
    }
}

fn parse_env_file(path: &PathBuf) -> Vec<EnvVar> {
    let content = fs::read_to_string(path).expect("env file not found");
    let lines: Vec<&str> = content.split("\n").collect();

    let mut env_vars: Vec<EnvVar> = Vec::new();

    // Iterate variables
    for line in lines {
        // If line is empty, skip
        if line.is_empty() {
            continue;
        }

        // Get variable description
        if line.starts_with("#!") {
            let chars = line.split("#!").collect::<Vec<&str>>();
            let _description = chars.last().unwrap();
        }

        // If line starts with #, skip
        if line.starts_with("#") {
            continue;
        }

        // Convert to String vector
        let var_string: Vec<String> = line.split("=").map(|x| x.to_string()).collect();
        // Check key
        let key = var_string.get(0);
        if key.is_none() {
            println!("name is not present");
            std::process::exit(1);
        }
        let key = parse_token(key.unwrap());
        // Check value

        let value = var_string.get(1);
        if value.is_none() {
            println!("name is not present");
            std::process::exit(1);
        }
        let value = parse_token(value.unwrap());

        if value.is_empty() {
            println!(
                "{}",
                format!(
                    "Value for variable {} is empty, please verify your .env file",
                    key
                )
                .red()
                .bold()
            );
            std::process::exit(1);
        }

        env_vars.push(EnvVar {
            name: key,
            value: value,
            required: true,
            default: None,
            description: None,
        });
    }
    return env_vars;
}

fn parse_token(text: &String) -> String {
    let value = text.clone();
    let value = value.trim();
    return value.to_string();
}
