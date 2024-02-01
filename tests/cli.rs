use assert_cmd::Command;
use predicates::prelude::*;

/* If there is no argument, the test will fail */
#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("my_echo").unwrap();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}
