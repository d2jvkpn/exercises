use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{error::Error, process::Command}; // Run programs // Add methods on commands

#[test]
fn run_with_defaults01() {
    Command::cargo_bin("catsay").expect("binary exists").assert().success();
}

#[test]
fn run_with_defaults02() {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn Error>> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();

    Ok(())
}
