use common::util::*;

static UTIL_NAME: &'static str = "cksum";

#[test]
fn test_single_file() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("lorem_ipsum.txt")
        .succeeds()
        .stdout_only(at.read("single_file.expected"));
}

#[test]
fn test_multiple_files() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("lorem_ipsum.txt")
        .arg("alice_in_wonderland.txt")
        .succeeds()
        .stdout_only(at.read("multiple_files.expected"));
}

#[test]
fn test_stdin() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    let input = at.read("lorem_ipsum.txt");
    ucmd.pipe_in(input)
        .succeeds()
        .stdout_only(at.read("stdin.expected"));
}
