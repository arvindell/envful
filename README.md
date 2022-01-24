# Envful

Envful is a CLI tool that verifies the presence of environment variables. It looks inside your [.env](https://www.npmjs.com/package/dotenv) file and system environment.

<img width="577" alt="Screen Shot 2022-01-23 at 7 48 05 p m" src="https://user-images.githubusercontent.com/29064411/150709280-7e2367f2-22ea-419a-83fd-812fab9c60ee.png">


## Installation

### NPM

You can install Envful using NPM, allowing you to run it from your project's scripts.

```bash
npm install envful
```

### crates.io

You can also install directly from crates.io using cargo.

```bash
cargo install envful
```

## Usage

Envful uses the `.env.example` file as a manifest for which variables are needed. If your project has a `.env.example` it already supports envful 🚀.

Check for variables and undeclared variables using `check`:

```bash
envful check
```

You can also specify a command to run if check is successful using the '--' separator. It will immediately fail if a variable is missing, showing helpful messages.

```bash
envful -- echo "I am envful!"

envful -- npm run dev
```

```
USAGE:
    envful [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -d, --dir <DIR>    Directory to look for .env and .env.example files
    -h, --help         Print help information
    -V, --version      Print version information

SUBCOMMANDS:
    check    Check if the .env has all required variables and warns if missing
    help     Print this message or the help of the given subcommand(s)
```

## How to declare variables

Inside your `.env.example` file, you can declare the variables that your application requires. You can use the triple # to add a comment to the variable.

Example:

```bash
### The URL to the database instance [required]
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres

### The app secret used to sign JSON Web Tokens
APP_SECRET=


```
