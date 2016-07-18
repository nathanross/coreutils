use common::util::testing;
use std::ffi::OsStr;

static UTIL_NAME: &'static str = "comm";

#[test]
fn ab_no_args() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.args(&["a", "b"]).succeeds()
        .stdout_only(at.read("ab.expected")));
}

#[test]
fn ab_dash_one() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.args(&["a", "b", "-1"]).succeeds()
        .stdout_only(at.read("ab.expected")));
}

#[test]
fn ab_dash_two() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.args(&["a", "b", "-2"]).succeeds()
        .stdout_only(at.read("ab2.expected")));
}

#[test]
fn ab_dash_three() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.args(&["a", "b", "-3"]).succeeds()
        .stdout_only(at.read("ab3.expected")));
}

#[test]
fn aempty() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.args(&["a", "empty"]).succeeds()
        .stdout_only(at.read("aempty.expected")));
}

#[test]
fn emptyempty() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.args(&["empty", "empty"]).succeeds()
        .stdout_only(at.read("aempty.expected")));
}

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn output_delimiter() {
    let (at, mut ucmd) = testing(UTIL_NAME);
    ucmd.args(&["--output-delimiter=word", "a", "b"]).succeeds()
        .stdout_only(at.read("ab_delimiter_word.expected"));
}

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn output_delimiter_require_arg() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    assert!(ucmd.args(&["--output-delimiter=", "a", "b"])
        .fails().no_stdout().stderr.len() > 0);
}

// even though (info) documentation suggests this is an option
// in latest GNU Coreutils comm, it actually is not.
// this test is essentially an alarm in case someone well-intendingly
// implements it.
#[test]
fn zero_terminated() {
    for param in vec!["-z", "--zero-terminated"] {
        let (_, mut ucmd) = testing(UTIL_NAME);
        assert!(ucmd.args(&[param, "a", "b"])
                .fails().no_stdout().stderr.len() > 0);
    }
}

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn check_order() {
    assert!(
        ucmd.args(&["--check-order", "bad_order_1", "bad_order_2"]).fails()
            .stdout_is("bad_order12.check_order.expected")
            .stderr.len() > 0)
}

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn nocheck_order() {
    ucmd.args(&["--nocheck-order", "bad_order_1", "bad_order_2"]).succeeds()
        .stdout_only(at.read("bad_order12.nocheck_order.expected"));
}

// when neither --check-order nor --no-check-order is provided,
// stderr and the error code behaves like check order, but stdout
// behaves like nocheck_order. However with some quirks detailed below.
#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn defaultcheck_order() {
    comm(&["a", "bad_order_1"], None, Some("error to be defined"));
}

// * the first: if both files are not in order, the default behavior is the only
// behavior that will provide an error message

// * the second: if two rows are paired but are out of order,
// it won't matter if all rows in the two files are exactly the same.
// This is specified in the documentation

#[test]
fn defaultcheck_order_identical_bad_order_files() {
    comm(&["bad_order_1", "bad_order_1"],
               Some("bad_order11.defaultcheck_order.expected"),
               None);
}

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn defaultcheck_order_two_different_bad_order_files() {
    comm(&["bad_order_1", "bad_order_2"],
               Some("bad_order12.nocheck_order.expected"),
               Some("error to be defined"));
}

// * the third: (it is not know whether this is a bug or not)
// for the first incident, and only the first incident,
// where both lines are different and one or both file lines being
// compared are out of order from the preceding line,
// it is ignored and no errors occur.

// * the fourth: (it is not known whether this is a bug or not)
// there are additional, not-yet-understood circumstances where an out-of-order
// pair is ignored and is not counted against the 1 maximum out-of-order line.

#[cfg_attr(not(feature="test_unimplemented"),ignore)]
#[test]
fn unintuitive_default_behavior_1() {
    comm(&["defaultcheck_unintuitive_1", "defaultcheck_unintuitive_2"],
               Some("defaultcheck_unintuitive.expected"),
               None);
}

#[ignore] //bug? should help be stdout if not called via -h|--help?
#[test]
fn no_arguments() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    assert!(ucmd.fails().no_stdout().stderr.len() > 0);
}

#[ignore] //bug? should help be stdout if not called via -h|--help?
#[test]
fn one_argument() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    assert!(ucmd.arg("a").fails().no_stdout().stderr.len() > 0);
}
