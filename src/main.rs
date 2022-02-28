use clap::{Parser, Subcommand};
use envful::checker;
use envful::runner;
use std::path::PathBuf;

/// A tool to verify the existence of environment variables before running a process.
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Path to environment file. Defaults to ./.env
    #[clap(short, long, parse(from_os_str), global = true)]
    file: Option<PathBuf>,
    /// Path to manifest file. Defaults to ./.env.example
    #[clap(short, long, parse(from_os_str), global = true)]
    manifest: Option<PathBuf>,

    /// Command to execute if successful
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check if env has all required variables and warns if missing
    Check,
    #[clap(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let args = Args::parse();
    let env_file = args.file;
    let env_manifest = args
        .manifest
        .unwrap_or_else(|| PathBuf::from(".env.example"));
    match args.command {
        Commands::Check => checker::check_command(env_file, &env_manifest, true, false),
        Commands::Other(args) => runner::run_command(env_file, &env_manifest, &args),
    }
}
