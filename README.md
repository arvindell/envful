# Envful

Envful is a CLI tool that verifies the presence of environment variables before running a process.

<!-- Add image -->

## Installation

### NPM

You can install Envful using NPM, allowing you to run it from your projects scripts.

```bash
npm install envful
```

### crates.io

You can also install dirrectly from crates.io using cargo.

```bash
cargo install envful
```

## Usage

Envful uses the `.env.example` file as a manifest for which variables are needed. If your project contains `.env.example` it already supports envful!

Check for variables and undeclared variables using `check`:

```bash
envful check
```

You can also specify a command using the '--' separator. It will immediately fail if a variable is missing, showing helful messages.

```bash
envful -- echo "I am envful!"
```

```
USAGE:
    envful [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -d, --dir <DIR>    Directory to look for envful.json
    -h, --help         Print help information
    -V, --version      Print version information

SUBCOMMANDS:
    check    Check if the .env has all required variables and warns if missing
    help     Print this message or the help of the given subcommand(s)
```

## How to declare variables

Inside your`.env.example` file, you can declare the variables that your process requires. You can use the triple # to add a comment to the variable.

Example:

```bash
### The URL to the database instance [required]
DATABASE_URL=mysql://user:pass@host:port/db

### The app secret used to sign JSON Web Tokens
APP_SECRET=


```
