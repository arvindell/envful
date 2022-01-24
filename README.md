# Envful 🌳

<a href="https://github.com/arvindell/envful/actions/workflows/build.yml"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/arvindell/envful/build?style=flat-square"></a>

<a href="https://www.npmjs.com/package/envful"><img alt="npm" src="https://img.shields.io/npm/v/envful?style=flat-square"></a>

Envful is a CLI tool that verifies the presence of environment variables. It looks inside your [.env](https://www.npmjs.com/package/dotenv) file and the host system. You can use it to run any process while ensuring all the variables are set.

Never again waste time debugging your app because of a misconfigured environment.

<img width="605" alt="Screen Shot 2022-01-23 at 10 13 06 p m" src="https://user-images.githubusercontent.com/29064411/150721003-78752d65-9477-4ace-8987-db6e1cf8ea20.png">

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

Envful uses the `.env.example` file as a manifest for which variables are needed. If your project has a `.env.example` it already supports envful! 🚀

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
    -d, --dir <DIR>    Directory to look for .env and .env.example files. Defaults to current directory.
    -h, --help         Print help information
    -V, --version      Print version information

SUBCOMMANDS:
    check    Check if env has all required variables and warns if missing
    help     Print this message or the help of the given subcommand(s)
```

## How to declare variables

Inside your `.env.example` file, you can declare the variables that your application requires. You can use the triple hash market (`###`) to add a description to the variable. Add `[optional]` to a variable for warning instead of failing.

Example:

```bash
### The URL to the database instance
DATABASE_URL=

### The app secret used to sign JSON Web Tokens
APP_SECRET=

### Google Analytics ID [optional]
GA_ID=
```

## Contributions welcome

This project welcomes contributions of any kind, whether you want to add new features, improve the documentation or just want to give some feedback.

## License

Envful is published under the MIT license. See the LICENSE file for more information.
