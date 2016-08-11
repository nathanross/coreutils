use common::util::*;

static UTIL_NAME: &'static str = "false";
fn new_ucmd() -> UCommand {
    TestScenario::new(UTIL_NAME).ucmd()
}

#[test]
fn test_exit_code() {
    new_ucmd().fails().no_stdout().no_stderr();
}

#[test]
fn test_help_ignored() {
    new_ucmd().arg("--help").fails().no_stdout().no_stderr();
}

#[test]
fn test_version_ignored() {
    new_ucmd().arg("--version").fails().no_stdout().no_stderr();
}