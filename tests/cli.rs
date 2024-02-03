use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected: &'static str) -> TestResult {
    Command::cargo_bin("my_echo")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

#[test]
fn hello() -> TestResult {
    run(&["hello"], "hello\n")
}

#[test]
fn hello_world() -> TestResult {
    run(&["Hello", "World"], "Hello World\n")
}

#[test]
fn a_day_in_the_life() -> TestResult {
    run(&["A Day   In  The     Life"], "A Day   In  The     Life\n")
}

#[test]
fn no_args() {
    Command::cargo_bin("my_echo")
        .unwrap()
        .assert()
        .success()
        .stdout(predicate::str::contains("Enter characters or use -v to specify environment variables!"));
}

#[test]
fn dies_text_and_env() -> TestResult {
    Command::cargo_bin("my_echo")?
        .args(["-e", "path", "hello"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));

    Ok(())
}
