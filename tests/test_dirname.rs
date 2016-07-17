use common::util::*;

static UTIL_NAME: &'static str = "dirname";

#[test]
fn test_path_with_trailing_slashes() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("/root/alpha/beta/gamma/delta/epsilon/omega//").succeeds()
        .stdout_only("/root/alpha/beta/gamma/delta/epsilon");
}

#[test]
fn test_path_without_trailing_slashes() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("/root/alpha/beta/gamma/delta/epsilon/omega").succeeds()
        .stdout_only("/root/alpha/beta/gamma/delta/epsilon");
}

#[test]
fn test_root() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("/").succeeds()
        .stdout_only("/");
}

#[test]
fn test_pwd() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg(".").succeeds()
        .stdout_only(".");
}

#[test]
fn test_empty() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("").succeeds()
        .stdout_only(".");
}
