use assert_cmd::Command;
use predicates::prelude::*;
use std::ffi;

#[test]
fn no_flags() {
    let mut cmd = Command::cargo_bin("examples/print").unwrap();
    cmd.assert().success();
}

/// Helper function to test errors when passing invalid arguments. Runs the
/// binary, passing `args`, expecting a failure that contains `msg`.
pub fn test_args_failure<I, S>(args: I, msg: &str)
where
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
{
    let mut cmd = Command::cargo_bin("examples/print").unwrap();
    cmd.args(args);
    cmd.assert().failure().stderr(predicate::str::contains(msg));
}

/// Helper function to test successes when passing valid arguments. Runs the
/// binary, passing `args`, expecting success that contains `msg`.
pub fn test_args_success<I, S>(args: I, msg: &str)
where
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
{
    let mut cmd = Command::cargo_bin("examples/print").unwrap();
    cmd.args(args);
    cmd.assert().success().stdout(predicate::str::contains(msg));
}

// Failure cases

#[test]
fn unrecognized_short_flag() {
    test_args_failure(&["-x"], "Unrecognized flag: -x");
}

#[test]
fn unrecognized_long_flag_no_arg() {
    test_args_failure(&["--foo"], "Unrecognized flag: --foo");
}

#[test]
fn unrecognized_long_flag_with_arg() {
    test_args_failure(&["--foo", "bar"], "Unrecognized flag: --foo");
}

#[test]
fn unrecognized_long_flag_equals_with_arg() {
    test_args_failure(&["--foo=bar"], "Unrecognized flag: --foo");
}

#[test]
fn unrecognized_long_flag_equals() {
    test_args_failure(&["--foo="], "Unrecognized flag: --foo");
}

// Success cases

#[test]
fn short_language_flag() {
    test_args_success(&["-l", "french"], "language = french\n");
}

#[test]
fn short_language_flag_cuddled() {
    test_args_success(&["-lfrench"], "language = french\n");
}

#[test]
fn long_language_flag() {
    test_args_success(&["--language", "french"], "language = french\n");
}

#[test]
fn long_language_flag_equals() {
    test_args_success(&["--language=french"], "language = french\n");
}

#[test]
fn no_prefix_on_long_args() {
    test_args_success(&["--nobig_menu"], "big_menu = false");
}

#[test]
fn args_are_passed_through() {
    test_args_success(&["foo"], "args = [\"foo\"]");
}
