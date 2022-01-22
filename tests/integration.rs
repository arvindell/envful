mod tests {
    use std::process::{Command, Stdio};

    #[test]
    fn succeeds_with_required() {
        let status = Command::new("cargo")
            .args(["run", "--", "check", "-d", "tests/fixtures/success"])
            .stdout(Stdio::null())
            .status();
        assert_eq!(status.unwrap().code(), Some(0));
    }

    #[test]
    fn fails_with_missing() {
        let status = Command::new("cargo")
            .args(["run", "--", "check", "-d", "tests/fixtures/missing"])
            .stdout(Stdio::null())
            .status();
        assert_eq!(status.unwrap().code(), Some(1));
    }
    #[test]
    fn fails_if_malformed() {
        let status = Command::new("cargo")
            .args(["run", "--", "check", "-d", "tests/fixtures/malformed"])
            .stdout(Stdio::null())
            .status();
        assert_eq!(status.unwrap().code(), Some(1));
    }
}
