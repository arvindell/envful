use clap::{Parser, Subcommand};
use envful::checker;

/// Check if environment variables are present before running a process.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,

    /// Directory to look for envful.json
    #[clap(parse(from_os_str), default_value = ".")]
    dir: std::path::PathBuf,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Check {
        /// Directory to look for envful.json
        #[clap(parse(from_os_str))]
        dir: Option<std::path::PathBuf>,
    },
    Run {
        #[clap(short, long)]
        binary: String,
    },
}

fn main() {
    let args = Args::parse();

    // let command = args.command;

    match args.command {
        Commands::Check { dir } => {
            let dir = dir.unwrap_or_else(|| std::path::PathBuf::from("."));
            checker::check(dir, false);
        }
        Commands::Run { binary } => {
            checker::check(args.dir, false);
            // Run the command with the given args
            let binary = binary;
            // let other_args: Vec<String> = args.iter().skip(2).map(|x| x.to_string()).collect();
            // println!(
            //     "{}",
            //     format!(
            //         "Running command {} with args {}",
            //         binary,
            //         other_args.join(" ")
            //     )
            // );
            // let status = Command::new(binary).args(other_args).status();
            // match status {
            //     Ok(code) => {
            //         std::process::exit(code.code().unwrap());
            //     }
            //     Err(_e) => {
            //         std::process::exit(1);
            //     }
            // }
        }
    }
}
