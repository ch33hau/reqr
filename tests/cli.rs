use std::error::Error;
use assert_cmd::Command;

type TestResult = Result<(), Box<dyn Error>>;

const PRG: &str = "reqr";

#[test]
fn empty() -> TestResult {
    Ok(())
}

#[test]
fn print_hello() -> TestResult {
    Command::cargo_bin(PRG)?
        .assert()
        .success()
        .stdout("Hello, world!\n");

    Ok(())
}