extern crate uu_dircolors;
use self::uu_dircolors::{StrUtils, guess_syntax, OutputFmt};

use common::util::*;

static UTIL_NAME: &'static str = "dircolors";

#[test]
fn test_shell_syntax() {
    use std::env;
    let last = env::var("SHELL");
    env::set_var("SHELL", "/path/csh");
    assert_eq!(OutputFmt::CShell, guess_syntax());
    env::set_var("SHELL", "csh");
    assert_eq!(OutputFmt::CShell, guess_syntax());
    env::set_var("SHELL", "/path/bash");
    assert_eq!(OutputFmt::Shell, guess_syntax());
    env::set_var("SHELL", "bash");
    assert_eq!(OutputFmt::Shell, guess_syntax());
    env::set_var("SHELL", "/asd/bar");
    assert_eq!(OutputFmt::Shell, guess_syntax());
    env::set_var("SHELL", "foo");
    assert_eq!(OutputFmt::Shell, guess_syntax());
    env::set_var("SHELL", "");
    assert_eq!(OutputFmt::Unknown, guess_syntax());
    env::remove_var("SHELL");
    assert_eq!(OutputFmt::Unknown, guess_syntax());

    if let Ok(s) = last {
        env::set_var("SHELL", s);
    }
}

#[test]
fn test_strutils() {
    let s = "  asd#zcv #hk\t\n  ";
    assert_eq!("asd#zcv", s.purify());

    let s = "con256asd";
    assert!(s.fnmatch("*[2][3-6][5-9]?sd"));

    let s = "zxc \t\nqwe jlk    hjl";
    let (k, v) = s.split_two();
    assert_eq!("zxc", k);
    assert_eq!("qwe jlk    hjl", v);
}

#[test]
fn test1() {
    test_helper("test1", "gnome");
}

#[test]
fn test_keywords() {
    test_helper("keywords", "");
}

#[test]
fn test_internal_db() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("-p")
        .succeeds()
        .stdout_only(at.read("internal.expected"));
}

#[test]
fn test_bash_default() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("-b").env("TERM", "screen")
        .succeeds()
        .stdout_only(at.read("bash_def.expected"));
}

#[test]
fn test_csh_default() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.env("TERM", "screen").arg("-c")
        .succeeds()
        .stdout_only(at.read("csh_def.expected"));
}

#[test]
fn test_no_env() {
    // no SHELL and TERM
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.fails();
}

#[test]
fn test_exclusive_option() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    ucmd.arg("-cp").fails();
}

fn test_helper(file_name: &str, term: &str) {
    let (at, mut ucmd) = testing(UTIL_NAME);
    let filename = format!("{}.csh.expected", file_name);
    ucmd.env("TERM", term)
        .arg("-c").arg(format!("{}.txt", file_name))
        .succeeds()
        .stdout_only(at.read(&filename));

    let (at, mut ucmd) = testing(UTIL_NAME);
    let filename = format!("{}.sh.expected", file_name);
    ucmd.env("TERM", term)
        .arg("-b").arg(format!("{}.txt", file_name))
        .succeeds()
        .stdout_only(at.read(&filename));
}
