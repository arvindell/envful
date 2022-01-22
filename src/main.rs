use clap::{Parser, Subcommand};
use envful::checker;
use envful::runner;
use std::path::PathBuf;

/// Check if environment variables are present before running a process.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Directory to look for envful.json
    #[clap(short, long, parse(from_os_str), global = true)]
    dir: Option<PathBuf>,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Check,
    Run {
        #[clap(raw = true)]
        command: String,
    },
}

fn main() {
    let args = Args::parse();

    let dir = args.dir;
    let dir = dir.unwrap_or_else(|| PathBuf::from("."));
    match args.command {
        Commands::Check => checker::check(&dir, false),
        Commands::Run { command } => runner::run(&dir, &command),
    }
}
