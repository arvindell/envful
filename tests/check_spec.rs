mod check_spec {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::process::Command;

    #[test]
    fn succeeds_with_required() -> Result<(), Box<dyn std::error::Error>> {
        run_check("success", true, "All variables are present", None)
    }

    #[test]
    fn fails_with_missing() -> Result<(), Box<dyn std::error::Error>> {
        run_check("missing", false, "Missing", None)
    }

    #[test]
    fn warns_on_undeclared() -> Result<(), Box<dyn std::error::Error>> {
        run_check("undeclared", true, "not declared", None)
    }

    #[test]
    fn warns_on_optional_missing() -> Result<(), Box<dyn std::error::Error>> {
        run_check(
            "optional_missing",
            true,
            "Some optional variables are missing",
            None,
        )
    }

    #[test]
    fn fails_if_env_file_missing() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("envful")?;
        let file = format!("tests/fixtures/success/nonexistent");
        let manifest = format!("tests/fixtures/success/.env.example");
        cmd.args(["check", "-f", file.as_str(), "-m", manifest.as_str()]);

        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Could not find environment file"));
        Ok(())
    }

    #[test]
    fn fails_if_manifest_file_missing() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("envful")?;
        let file = format!("tests/fixtures/success/.env");
        let manifest = format!("tests/fixtures/success/nonexistent");
        cmd.args(["check", "-f", file.as_str(), "-m", manifest.as_str()]);

        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Could not find manifest file"));
        Ok(())
    }

    #[test]
    fn succeeds_if_found_in_env() -> Result<(), Box<dyn std::error::Error>> {
        // Set SENGRID_API_KEY in env
        let env_vars = vec![("SENDGRID_API_KEY", "12345")];
        let result = run_check(
            "missing_but_in_env",
            true,
            "All variables are present",
            Some(env_vars),
        );
        result
    }

    #[test]
    fn displays_correct_optional() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("envful")?;
        let fixture = "two_optional";
        let file = format!("tests/fixtures/{}/.env", fixture);
        let manifest = format!("tests/fixtures/{}/.env.example", fixture);
        cmd.args(["check", "-f", file.as_str(), "-m", manifest.as_str()]);

        let has_first = predicate::str::contains("Missing optional variable: SENDGRID_API_KEY");
        let has_second = predicate::str::contains("Missing optional variable: STRIPE_SK");
        cmd.assert().success().stdout(has_first).stdout(has_second);
        Ok(())
    }

    fn run_check(
        fixture: &str,
        should_succeed: bool,
        expected_output: &str,
        env_vars: Option<Vec<(&str, &str)>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("envful")?;
        let file = format!("tests/fixtures/{}/.env", fixture);
        let manifest = format!("tests/fixtures/{}/.env.example", fixture);
        cmd.args(["check", "-f", file.as_str(), "-m", manifest.as_str()]);

        if env_vars.is_some() {
            for (key, value) in env_vars.unwrap() {
                cmd.env(key, value);
            }
        }

        let predicate = predicate::str::contains(expected_output);
        if should_succeed {
            cmd.assert().success().stdout(predicate);
        } else {
            cmd.assert().failure().stderr(predicate);
        }
        Ok(())
    }
}
