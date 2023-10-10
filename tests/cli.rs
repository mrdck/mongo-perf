use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use std::env;

#[test]
fn test_cli_invalid_args() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mongodb-perf")?;

    cmd.arg("foobar").arg("baz");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Found argument \'foobar\' which wasn\'t expected, or isn\'t valid in this context"));

    Ok(())
}

#[test]
fn test_data_load() -> Result<(), Box<dyn std::error::Error>> {
    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI not set in environment");
    let mut cmd = Command::cargo_bin("mongodb-perf")?;

    cmd.arg("--uri").arg(&mongodb_uri);
    cmd.assert().success();

    Ok(())
}