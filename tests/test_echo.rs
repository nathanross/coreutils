use common::util::*;

static UTIL_NAME: &'static str = "echo";

#[test]
fn test_default() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.run()
        .succeeds()
        .stdout_only("\n");
}

#[test]
fn test_no_trailing_newline() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("-n").arg("hello_world")
        .succeeds()
        .stdout_only("hello_world");
}

#[test]
fn test_enable_escapes() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("-e").arg("\\\\\\t\\r")
        .succeeds()
        .stdout_only("\\\t\r\n");
}

#[test]
fn test_disable_escapes() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("-E").arg("\\b\\c\\e")
        .succeeds()
        .stdout_only("\\b\\c\\e\n");
}
