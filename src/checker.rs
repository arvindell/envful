use crate::EnvVar;
use crate::EnvVarDeclaration;
use colored::*;
use std::fs;
use std::path::PathBuf;

pub fn check_command(
    dir: &PathBuf,
    silent: bool,
    show_undeclared: bool,
    show_missing_optional: bool,
) {
    if !silent {
        println!("{}", "Checking environment...".cyan());
    }

    let manifest_path = dir.clone().join(".env.example");
    let env_file_path = dir.clone().join(".env");
    let declared_vars: Vec<EnvVarDeclaration> = parse_manifest_file(&manifest_path);

    // Get given vars from .env file
    let mut given_vars = parse_env_file(&env_file_path)
        .iter()
        // Remove empty vars
        .filter(|var| !var.value.is_none() && !var.value.as_ref().unwrap().is_empty())
        .map(|var| var.name.clone())
        .collect::<Vec<String>>();

    // Push to given vars the ones set in the system env
    let system_vars: Vec<String> = std::env::vars()
        .filter(|(key, _value)| {
            declared_vars
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<String>>()
                .contains(key)
        })
        .map(|(key, _value)| key)
        .collect();
    given_vars.extend(system_vars);

    let required_missing_vars: Vec<String> = declared_vars
        .iter()
        .filter(|v| !v.optional && !given_vars.contains(&v.name))
        .map(|v| v.name.clone())
        .collect();
    let optional_missing_vars: Vec<String> = declared_vars
        .iter()
        .filter(|v| v.optional && !given_vars.contains(&v.name))
        .map(|v| v.name.clone())
        .collect();
    let undeclared_vars: Vec<String> = given_vars
        .iter()
        .filter(|v| {
            !declared_vars
                .iter()
                .map(|dec_var| dec_var.name.clone())
                .collect::<Vec<String>>()
                .contains(v)
        })
        .map(|v| v.clone())
        .collect();
    let error = required_missing_vars.len() > 0;

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

    if optional_missing_vars.len() > 0 && show_missing_optional {
        println!("{}", "Some optional variables are missing:".yellow().bold());
        for optional_var in optional_missing_vars {
            println!(
                "{} {}",
                " Missing optional variable:".yellow(),
                optional_var.yellow()
            );
        }
    }

    if error {
        // Print message for every missing var
        eprintln!(
            "{}",
            "The process is missing required environment variables:"
                .red()
                .bold()
        );
        for missing_var in required_missing_vars {
            eprintln!(
                "{} {}",
                "❌ Missing variable:".yellow(),
                missing_var.yellow()
            );
        }
        // Exit with error code
        std::process::exit(1);
    }

    if !silent {
        println!("{}", "All variables are present ✅".green());
    }
}

fn parse_manifest_file(path: &PathBuf) -> Vec<EnvVarDeclaration> {
    let content = fs::read_to_string(path);

    if content.is_err() {
        eprintln!(
            "{}",
            "Could not find .env.example manifest file. If it is not in the working dir, use the -d option."
                .red()
                .bold()
        );
        std::process::exit(1);
    }
    let content = content.unwrap();

    let lines: Vec<&str> = content.split("\n").collect();

    let mut env_vars: Vec<EnvVarDeclaration> = Vec::new();

    // Iterate variables
    let mut optional = false;
    for line in lines {
        // If line is empty, skip
        if line.is_empty() {
            continue;
        }

        let mut description: Option<String> = None;
        // Get variable description
        let comment_marker = "###";
        if line.starts_with(comment_marker) {
            description = Some(line.replace(comment_marker, ""));
            if description.clone().unwrap().contains("[optional]") {
                optional = true;
            }
        }

        // If line starts with #, skip
        if line.starts_with("#") {
            continue;
        }

        // Convert to String vector
        let var_string: Vec<String> = line.split("=").map(|x| x.to_string()).collect();
        // Check key
        let name = var_string.get(0);
        if name.is_none() {
            panic!("Name is not present in the line: {}", line);
        }
        let name = parse_token(name.unwrap());
        // Check value

        env_vars.push(EnvVarDeclaration {
            name,
            optional,
            default: None,
            description,
        });
    }
    return env_vars;
}

fn parse_env_file(path: &PathBuf) -> Vec<EnvVar> {
    let content = fs::read_to_string(path);

    // If file is not found, return empty vector as vars could be set via system
    if content.is_err() {
        return Vec::new();
    }

    let content = content.unwrap();
    let lines: Vec<&str> = content.split("\n").collect();

    let mut env_vars: Vec<EnvVar> = Vec::new();

    // Iterate variables
    for line in lines {
        // If line is empty, skip
        if line.is_empty() {
            continue;
        }

        // Get variable description
        if line.starts_with("###") {
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
        let name = var_string.get(0);
        if name.is_none() {
            panic!("Name is not present in the line: {}", line);
        }

        // If keys not there, skip
        if name.unwrap().is_empty() {
            continue;
        }

        let name = parse_token(name.unwrap());
        let value = var_string.get(1).cloned();

        // Convert value to owned
        let value = value.clone();
        env_vars.push(EnvVar { name, value });
    }
    return env_vars;
}

fn parse_token(text: &String) -> String {
    let value = text.clone();
    let value = value.trim();
    return value.to_string();
}
