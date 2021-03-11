use assert_cmd::Command;
use predicates::prelude::*;
use std::ffi::OsStr;

#[test]
fn no_flags() {
    let mut cmd = Command::cargo_bin("examples/print").unwrap();
    cmd.assert().success();
}

/// Helper function to test errors when passing invalid arguments. Runs the
/// binary, passing `args`, expecting a failure that contains `msg`.
fn test_args_failure<I, S>(args: I, msg: &str)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::cargo_bin("examples/print").unwrap();
    cmd.args(args);
    cmd.assert().failure().stderr(predicate::str::contains(msg));
}

/// Helper function to test successes when passing valid arguments. Runs the
/// binary, passing `args`, expecting success that contains `msg`.
fn test_args_success<I, S>(args: I, msg: &str)
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut cmd = Command::cargo_bin("examples/print").unwrap();
    cmd.args(args);
    cmd.assert().success().stdout(predicate::str::contains(msg));
}

// Failure cases

#[test]
fn unrecognized_short_flag() {
    test_args_failure(&["-x"], "Unrecognized flag: -x\n");
}

#[test]
fn unrecognized_long_flag_no_arg() {
    test_args_failure(&["--foo"], "Unrecognized flag: --foo\n");
}

#[test]
fn unrecognized_long_flag_with_arg() {
    test_args_failure(&["--foo", "bar"], "Unrecognized flag: --foo\n");
}

#[test]
fn unrecognized_long_flag_equals_with_arg() {
    test_args_failure(&["--foo=bar"], "Unrecognized flag: --foo\n");
}

#[test]
fn unrecognized_long_flag_equals() {
    test_args_failure(&["--foo="], "Unrecognized flag: --foo\n");
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
fn short_flag_repeat() {
    test_args_success(&[""], 
    "verbose = false\nis verbose? = false\nverbose count = 0\n");

    test_args_success(&["-v", "-v"], 
    "verbose = true\nis verbose? = true\nverbose count = 2\n");
}

#[test]
fn short_flag_repeat_packed() {
    test_args_success(&["-vv"], 
    "verbose = true\nis verbose? = true\nverbose count = 2\n");
}

#[test]
fn long_flag_repeat() {
    test_args_success(&["--verbose", "--verbose"], 
    "verbose = true\nis verbose? = true\nverbose count = 2\n");
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
    test_args_success(&["--nobig_menu"], "big_menu = false\n");
}

#[test]
fn args_are_passed_through() {
    test_args_success(&["foo"], "args = [\"foo\"]\n");
}
