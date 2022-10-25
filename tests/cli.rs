use std::error::Error;
use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "reqr";

#[test]
fn dies_no_input() -> TestResult {
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("error"));

    Ok(())
}

#[test]
fn print_usage() -> TestResult {
    Command::cargo_bin(PRG)?
        .args(&["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage"));

    Ok(())
}

#[test]
fn accept_url() -> TestResult {
    let input = "https://example.com";
    Command::cargo_bin(PRG)?
        .args(&["--url", input])
        .assert()
        .success()
        .stdout(predicate::str::contains(input));

    Ok(())
}