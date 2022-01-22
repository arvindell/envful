mod tests {
    use std::process::Command;

    #[test]
    fn succeeds_with_required() {
        let status = Command::new("cargo")
            .args(["run", "--", "check", "-d", "tests/fixtures/success"])
            .status();
        assert_eq!(status.unwrap().code(), Some(0));
    }

    #[test]
    fn fails_with_missing() {
        let status = Command::new("cargo")
            .args(["run", "--", "check", "-d", "tests/fixtures/missing"])
            .status();
        assert_eq!(status.unwrap().code(), Some(1));
    }
    #[test]
    fn fails_if_malformed() {
        let status = Command::new("cargo")
            .args(["run", "--", "check", "-d", "tests/fixtures/malformed"])
            .status();
        assert_eq!(status.unwrap().code(), Some(1));
    }
}
