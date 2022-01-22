use clap::{Parser, Subcommand};
use envful::checker;
use envful::runner;
use std::path::PathBuf;

/// A tool to verify the existence of environment variables before running a process.
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Directory to look for envful.json
    #[clap(short, long, parse(from_os_str), global = true)]
    dir: Option<PathBuf>,

    /// Command to execute
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Check if the .env has all required variables and warns if missing
    Check,
    #[clap(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let args = Args::parse();
    let dir = args.dir.unwrap_or_else(|| PathBuf::from("."));
    match args.command {
        Commands::Check => checker::check(&dir, true, false),
        Commands::Other(args) => runner::run(&dir, &args),
    }
}
