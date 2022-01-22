mod run_spec {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn runs_command_after_check() -> Result<(), Box<dyn std::error::Error>> {
        run_run("success", true, "echo 'Hello world'", "Hello world")
    }

    fn run_run(
        fixture: &str,
        should_succeed: bool,
        command: &str,
        expected_out_put: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("envful")?;
        let dir = format!("tests/fixtures/{}", fixture);

        let command_args: Vec<&str> = command.split_whitespace().collect();
        let run_args = ["-d", dir.as_str(), "--"];
        let all_args = run_args.iter().chain(command_args.iter());
        cmd.args(all_args);

        let predicate = predicate::str::contains(expected_out_put);
        if should_succeed {
            cmd.assert().success().stdout(predicate);
        } else {
            cmd.assert().failure().stderr(predicate);
        }
        Ok(())
    }
}