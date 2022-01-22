mod check_spec {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn succeeds_with_required() -> Result<(), Box<dyn std::error::Error>> {
        run_check("success", true, "All variables are present")
    }

    #[test]
    fn fails_with_missing() -> Result<(), Box<dyn std::error::Error>> {
        run_check("missing", false, "Missing")
    }

    #[test]
    fn fails_if_malformed() -> Result<(), Box<dyn std::error::Error>> {
        run_check("malformed", false, "Missing")
    }

    fn run_check(
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
