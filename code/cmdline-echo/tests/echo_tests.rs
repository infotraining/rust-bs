use assert_cmd::cargo_bin_cmd;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    let mut cmd = cargo_bin_cmd!("cmdline-echo");
    cmd.assert().failure().stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs() {
    let mut cmd = cargo_bin_cmd!("cmdline-echo");
    cmd.arg("some text").assert().success();
}

#[test]
fn echo_with_newline() {
    let expected = "some text\n\n";
    let mut cmd = cargo_bin_cmd!("cmdline-echo");
    cmd.arg("some text").assert().success().stdout(predicate::eq(expected));
}

#[test]
fn echo_without_newline() {
    let expected = "some text\n";
    let mut cmd = cargo_bin_cmd!("cmdline-echo");
    cmd.args(&["some text", "-n"]).assert().success().stdout(predicate::eq(expected));
}