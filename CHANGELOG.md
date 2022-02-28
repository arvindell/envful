## Breaking changes

- The `-d` option has been removed in favor of the -m and -f options, which offer more granular control.

## Improvements

- The `-f` option has been added to specify the properties file to use (commonly .env)
- The `-m` option has been added to specify the manifest to use (commonly .env.example)
- The `--show-undeclared` option has been added for warning undeclared variables when using in run mode.
- The `--show-optional` option has been added for warning undeclared variables when using in run mode.
- An error message will be now shown when the environment file is specified and not found.
