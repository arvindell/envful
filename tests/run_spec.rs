mod run_spec {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn runs_command_after_check() -> Result<(), Box<dyn std::error::Error>> {
        run_run("success", true, "", "echo 'Hello world'", "Hello world")
    }

    #[test]
    fn warns_optional_if_enabled() -> Result<(), Box<dyn std::error::Error>> {
        run_run(
            "optional_missing",
            true,
            "--show-optional",
            "echo 'Hello world'",
            "Missing optional variable",
        )
    }

    #[test]
    fn warns_undeclared_if_enabled() -> Result<(), Box<dyn std::error::Error>> {
        run_run(
            "undeclared",
            true,
            "--show-undeclared",
            "echo 'Hello world'",
            "Undeclared variable",
        )
    }

    fn run_run(
        fixture: &str,
        should_succeed: bool,
        envful_args: &str,
        command: &str,
        expected_output: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("envful")?;
        let file = format!("tests/fixtures/{}/.env", fixture);
        let manifest = format!("tests/fixtures/{}/.env.example", fixture);

        let envful_args: Vec<&str> = envful_args.split_whitespace().collect();
        let mut command_args: Vec<&str> = command.split_whitespace().collect();
        command_args.insert(0, "--");
        let run_args: Vec<&str> = ["-f", file.as_str(), "-m", manifest.as_str(), "--"]
            .iter()
            .chain(envful_args.iter())
            .copied()
            .collect();

        let all_args = run_args.iter().chain(command_args.iter());
        cmd.args(all_args);

        let predicate = predicate::str::contains(expected_output);
        if should_succeed {
            cmd.assert().success().stdout(predicate);
        } else {
            cmd.assert().failure().stderr(predicate);
        }
        Ok(())
    }
}
