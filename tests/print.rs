use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::ffi;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[test]
fn no_flags() -> Result<()> {
    let mut cmd = Command::cargo_bin("examples/print")?;
    cmd.assert().success();
    Ok(())
}

/// Helper function to test errors when passing invalid arguments. Runs the
/// binary, passing `args`, expecting a failure that contains `msg`.
pub fn test_args_failure<I, S>(args: I, msg: &str) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
{
    let mut cmd = Command::cargo_bin("examples/print")?;
    cmd.args(args);
    cmd.assert().failure().stderr(predicate::str::contains(msg));
    Ok(())
}

/// Helper function to test successes when passing valid arguments. Runs the
/// binary, passing `args`, expecting success that contains `msg`.
pub fn test_args_success<I, S>(args: I, msg: &str) -> Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
{
    let mut cmd = Command::cargo_bin("examples/print")?;
    cmd.args(args);
    cmd.assert().success().stdout(predicate::str::contains(msg));
    Ok(())
}

// Failure cases

#[test]
fn unrecognized_short_flag() -> Result<()> {
    test_args_failure(&["-x"], "Unrecognized flag: -x")
}

#[test]
fn unrecognized_long_flag_no_arg() -> Result<()> {
    test_args_failure(&["--foo"], "Unrecognized flag: --foo")
}

#[test]
fn unrecognized_long_flag_with_arg() -> Result<()> {
    test_args_failure(&["--foo", "bar"], "Unrecognized flag: --foo")
}

#[test]
fn unrecognized_long_flag_equals_with_arg() -> Result<()> {
    test_args_failure(&["--foo=bar"], "Unrecognized flag: --foo")
}

#[test]
fn unrecognized_long_flag_equals() -> Result<()> {
    test_args_failure(&["--foo="], "Unrecognized flag: --foo")
}

// Success cases

#[test]
fn short_language_flag() -> Result<()> {
    test_args_success(&["-l", "french"], "language = french\n")
}

#[test]
fn short_language_flag_cuddled() -> Result<()> {
    test_args_success(&["-lfrench"], "language = french\n")
}

#[test]
fn long_language_flag() -> Result<()> {
    test_args_success(&["--language", "french"], "language = french\n")
}

#[test]
fn long_language_flag_equals() -> Result<()> {
    test_args_success(&["--language=french"], "language = french\n")
}

#[test]
fn no_prefix_on_long_args() -> Result<()> {
    test_args_success(&["--nobig_menu"], "big_menu = false")
}

#[test]
fn args_are_passed_through() -> Result<()> {
    test_args_success(&["foo"], "args = [\"foo\"]")
}
