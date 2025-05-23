use sudo_test::{Command, Env, TextFile};

use crate::USERNAME;

#[test]
fn arguments_are_passed_to_shell() {
    let shell_path = "/tmp/my-shell";
    let shell = r#"#!/bin/sh
echo $0; echo $1; echo $2"#;
    let env = Env("")
        .user(USERNAME)
        .file(shell_path, TextFile(shell).chmod("755"))
        .build();

    let shell_args = ["a", "b c"];
    let stdout = Command::new("env")
        .args(["su", USERNAME, shell_path])
        .args(shell_args)
        .output(&env)
        .stdout();

    let [arg0, arg1] = shell_args;
    assert_eq!(
        format!(
            "{shell_path}
{arg0}
{arg1}"
        ),
        stdout
    );
}

#[test]
fn dash_user_shell_arguments() {
    let shell_path = "/tmp/my-shell";
    let shell = r#"#!/bin/sh
echo "${@}""#;
    let env = Env("")
        .user(USERNAME)
        .file(shell_path, TextFile(shell).chmod("755"))
        .build();

    let shell_args = ["a", "b c"];
    let stdout = Command::new("env")
        .args(["su", "-", USERNAME, shell_path])
        .args(shell_args)
        .output(&env)
        .stdout();

    assert_eq!(shell_args.join(" "), stdout);
}

#[test]
fn flag_after_positional_argument() {
    let expected = "-sh";
    let env = Env("").build();
    let stdout = Command::new("env")
        .args(["su", "-c", "echo $0", "root", "-l"])
        .output(&env)
        .stdout();

    assert_eq!(expected, stdout);
}
