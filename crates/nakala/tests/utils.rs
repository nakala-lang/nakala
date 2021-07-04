use assert_cmd::prelude::*;
use std::process::{Command, Stdio};

pub fn compare_output(arg_list: Vec<&str>, expected: &str) {
    let mut cmd = match Command::cargo_bin("nakala") {
        Ok(bin) => bin,
        Err(e) => {
            unreachable!(format!("Failed to create new Command. Reasoning: {}", e))
        }
    };

    for arg in arg_list {
        cmd.arg(arg);
    }

    cmd.stdout(Stdio::piped());
    let output = cmd.output().expect("Failed to run nakala");

    let actual = String::from_utf8(output.stdout).expect("Failed to get command output as String");

    assert_eq!(actual, expected);
}
