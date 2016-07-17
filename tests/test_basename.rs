use common::util::*;

static UTIL_NAME: &'static str = "basename";

#[test]
fn test_directory() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let dir = "/root/alpha/beta/gamma/delta/epsilon/omega/";
    ucmd.args(&[dir]).succeeds().stdout_only("omega");
}

#[test]
fn test_file() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let file = "/etc/passwd";
    ucmd.args(&[file]).succeeds().stdout_only("passwd");
}

#[test]
fn test_remove_suffix() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let path = "/usr/local/bin/reallylongexecutable.exe";
    ucmd.args(&[path, ".exe"]).succeeds().stdout_only("reallylongexecutable");
}

#[test]
fn test_dont_remove_suffix() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let path = "/foo/bar/baz";
    ucmd.args(&[path, "baz"]).succeeds().stdout_only("baz");
}

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn test_multiple_param() {
    for multiple_param in vec!["-a", "--multiple"] {
        let (_, mut ucmd) = testing(UTIL_NAME);
        let path = "/foo/bar/baz";
        ucmd.args(&[multiple_param, path, path]).succeeds().stdout_only("baz\nbaz");
    }
}

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn test_suffix_param() {
    for suffix_param in vec!["-s", "--suffix"] {
        let (_, mut ucmd) = testing(UTIL_NAME);
        let path = "/foo/bar/baz.exe";
        let suffix = ".exe";
        ucmd.args(&[suffix_param, suffix, path, path]).succeeds().stdout_only("baz\nbaz");
    }
}

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn test_zero_param() {
    for zero_param in vec!["-z", "--zero"] {
        let (_, mut ucmd) = testing(UTIL_NAME);
        let path = "/foo/bar/baz";
        ucmd.args(&[zero_param, "-a", path, path]).succeeds().stdout_only("baz\0baz\0");
    }
}

#[test]
fn test_invalid_option() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let path = "/foo/bar/baz";
    assert!(ucmd.args(&["-q", path]).fails().stderr.len() > 0);
}

#[test]
fn test_no_args() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let args : Vec<String> = Vec::new();
    assert!(ucmd.args(&args).fails().stderr.len() > 0);
}

#[test]
fn test_too_many_args() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    assert!(ucmd.args(&["a", "b", "c"]).fails().stderr.len() > 0);
}
