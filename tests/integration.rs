mod tests {
    use assert_cmd::prelude::*; // Add methods on commands
    use predicates::prelude::*; // Used for writing assertions
    use std::process::Command;

    #[test]
    fn succeeds_with_required() -> Result<(), Box<dyn std::error::Error>> {
        run("success", true, "All variables are present")
    }

    #[test]
    fn fails_with_missing() -> Result<(), Box<dyn std::error::Error>> {
        run("missing", false, "Missing")
    }

    #[test]
    fn fails_if_malformed() -> Result<(), Box<dyn std::error::Error>> {
        run("malformed", false, "Missing")
    }

    fn run(
        fixture: &str,
        should_succeed: bool,
        expected_out_put: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("envful")?;
        let dir = format!("tests/fixtures/{}", fixture);
        cmd.args(["check", "-d", dir.as_str()]);

        let predicate = predicate::str::contains(expected_out_put);
        if should_succeed {
            cmd.assert().success().stdout(predicate);
        } else {
            cmd.assert().failure().stderr(predicate);
        }
        Ok(())
    }
}
