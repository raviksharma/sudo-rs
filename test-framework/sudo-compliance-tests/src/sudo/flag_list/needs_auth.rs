use sudo_test::{Command, Env};

use crate::USERNAME;

#[test]
fn when_other_user_is_self() {
    let env = Env("Defaults !lecture
ALL ALL=(ALL:ALL) ALL")
    .user(USERNAME)
    .build();

    let output = Command::new("sudo")
        .args(["-S", "-l", "-U", USERNAME])
        .as_user(USERNAME)
        .output(&env);

    output.assert_exit_code(1);

    let diagnostic = if sudo_test::is_original_sudo() {
        if cfg!(not(target_os = "linux")) {
            "Password:".to_owned()
        } else {
            format!("[sudo] password for {USERNAME}:")
        }
    } else {
        "[sudo: authenticate] Password:".to_string()
    };
    assert_contains!(output.stderr(), diagnostic);
}

#[test]
fn other_user_has_nopasswd_tag() {
    let other_user = "ghost";
    let env = Env(format!(
        "Defaults !lecture
{other_user} ALL=(ALL:ALL) NOPASSWD: ALL
{USERNAME} ALL=(ALL:ALL) ALL"
    ))
    .user(USERNAME)
    .user(other_user)
    .build();

    let output = Command::new("sudo")
        .args(["-S", "-l", "-U", other_user])
        .as_user(USERNAME)
        .output(&env);

    output.assert_exit_code(1);

    let diagnostic = if sudo_test::is_original_sudo() {
        if cfg!(not(target_os = "linux")) {
            "Password:".to_owned()
        } else {
            format!("[sudo] password for {USERNAME}:")
        }
    } else {
        "[sudo: authenticate] Password:".to_string()
    };
    assert_contains!(output.stderr(), diagnostic);
}
