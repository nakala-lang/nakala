use assert_cmd::prelude::*;
use std::process::{Command, Stdio};

pub fn compare_output(arg_list: Vec<&str>, input: Option<&str>, expected: &str) {
    let mut cmd = match Command::cargo_bin("nakala") {
        Ok(bin) => bin,
        Err(e) => {
            unreachable!(format!("Failed to create new Command. Reasoning: {}", e))
        }
    };

    for arg in arg_list {
        cmd.arg(arg);
    }

    if let Some(input) = input {
        cmd.arg(format!("-i \"{}\"", input));
    }

    cmd.stdout(Stdio::piped());
    let output = cmd.output().expect("Failed to run nakala");

    let actual = String::from_utf8(output.stdout).expect("Failed to get command output as String");

    // the actual always has a new line, so add a new line to expected to be consistent
    let mut real_expected = String::from(expected);
    real_expected.push('\n');

    assert_eq!(actual, real_expected);
}
