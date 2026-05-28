use assert_cmd::{Command};
use predicates::prelude::*;
#[test]
fn dies_no_args(){
    let mut cmd = Command::cargo_bin("rust-echo").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs(){
    let mut cmd = Command::cargo_bin("rust-echo").unwrap();
    cmd.arg("hello").assert().success();
}
