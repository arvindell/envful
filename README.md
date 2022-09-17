# Envful

<a href="https://github.com/arvindell/envful/actions/workflows/build.yml"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/arvindell/envful/build?style=flat-square"></a> <a href="https://www.npmjs.com/package/envful"><img alt="npm" src="https://img.shields.io/npm/v/envful?style=flat-square"></a> <a href="https://crates.io/crates/envful"><img alt="Crates.io" src="https://img.shields.io/crates/v/envful?style=flat-square"></a>

Envful is a CLI tool that verifies the presence of environment variables. It looks inside your [.env](https://www.npmjs.com/package/dotenv) file and the host system. You can use it to run any process while ensuring all the variables are set.

Never again waste time debugging your app because of a misconfigured environment.

<img width="605" alt="Screen Shot 2022-01-23 at 10 13 06 p m" src="https://user-images.githubusercontent.com/29064411/150721003-78752d65-9477-4ace-8987-db6e1cf8ea20.png">

## Installation

### Installation script

Use the convenience install script that can run on all bash systems. Run the following command in your terminal (git bash for Windows).

```bash
# Warning: always examine scripts downloaded from the internet before running them locally.
curl https://raw.githubusercontent.com/arvindell/envful/main/install.sh -o install.sh && bash install.sh
```

This command can also be used to update your installation.

### NPM

You can install Envful using NPM.

```bash
# Install locally
npm install envful

# Or globally
npm install -g envful
```

### crates.io

Install directly from crates.io using cargo.

```bash
cargo install envful
```

## Usage

Envful uses the `.env.example` file as a manifest for which variables are needed. If your project has a `.env.example` it already supports envful! 🚀

Check for variables and undeclared variables using `check`:

```bash
envful check
```

You can also specify a command to run if check is successful using the '--' separator. It will immediately fail if a variable is missing, showing a helpful message.

```bash
envful -- echo "I am envful!"
```

This becomes very useful to check the environment inside `package.json` scripts:

```json
"scripts": {
    "dev": "envful -- next",
    "build": "envful -- next build"
}
```

### Arguments

```
USAGE:
    envful [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -f, --file <FILE>            Path to environment file. Defaults to ./.env
    -h, --help                   Print help information
    -m, --manifest <MANIFEST>    Path to manifest file. Defaults to ./.env.example
        --show-optional          Whether to print missing optional variables. Defaults to false
        --show-undeclared        Whether to show undeclared variables in output. Defaults to false
    -V, --version                Print version information

SUBCOMMANDS:
    check    Check if env has all required variables and warns if missing
    help     Print this message or the help of the given subcommand(s)
```

## How to declare variables

Inside your `.env.example` file, you can declare the variables that your application requires. You can use the triple hash marker (`###`) to add a description to the variable. 

Add `[optional]` to a variable for warning instead of failing.

Example:

```bash
### The URL to the database instance
DATABASE_URL=

### The app secret used to sign JSON Web Tokens
APP_SECRET=

### Google Analytics ID [optional]
GA_ID=
```

Note: At the moment any ### marker will be intepreted as the comment for the next variable in the file, regardless of any whitespace between the two lines.

## Contributions welcome

This project welcomes contributions of any kind, whether you want to add new features, improve the documentation or just want to give some feedback.

## License

Envful is published under the MIT license. See the LICENSE file for more information.
