//! PAM integration tests

use sudo_test::{Command, Env, User, BIN_TRUE};

use crate::{PASSWORD, USERNAME};

#[test]
fn given_pam_permit_then_no_password_auth_required() {
    let env = Env("")
        .user(USERNAME)
        .file("/etc/pam.d/su", "auth sufficient pam_permit.so")
        .build();

    Command::new("su")
        .args(["-c", BIN_TRUE])
        .as_user(USERNAME)
        .output(&env)
        .assert_success();
}

#[test]
fn given_pam_deny_then_password_auth_always_fails() {
    let invoking_user = USERNAME;
    let target_user = "ghost";

    let env = Env("")
        .file("/etc/pam.d/su", "auth requisite pam_deny.so")
        .user(invoking_user)
        .user(User(target_user).password(PASSWORD))
        .build();

    let output = Command::new("su")
        .args(["-s", BIN_TRUE, target_user])
        .as_user(invoking_user)
        .stdin(PASSWORD)
        .output(&env);

    output.assert_exit_code(1);

    let diagnostic = if sudo_test::is_original_sudo() {
        "su: Authentication failure"
    } else {
        "3 incorrect authentication attempts"
    };
    assert_contains!(output.stderr(), diagnostic);
}

#[test]
fn being_root_has_precedence_over_missing_pam_file() {
    let env = Env("").build();

    Command::new("su").output(&env).assert_success();
}

#[test]
fn being_root_has_no_precedence_over_pam_deny() {
    let env = Env("")
        .file("/etc/pam.d/su", "auth requisite pam_deny.so")
        .build();

    let output = Command::new("su").args(["-c", BIN_TRUE]).output(&env);

    output.assert_exit_code(1);

    let diagnostic = if sudo_test::is_original_sudo() {
        "su: Authentication failure"
    } else {
        "3 incorrect authentication attempts"
    };
    assert_contains!(output.stderr(), diagnostic);
}

#[test]
#[cfg_attr(target_os = "freebsd", ignore = "FreeBSD doesn't use /etc/pam.d/su-l")]
fn su_uses_correct_service_file() {
    let env = Env("")
        .file("/etc/pam.d/su", "auth sufficient pam_permit.so")
        .file("/etc/pam.d/su-l", "auth requisite pam_deny.so")
        .user(USERNAME)
        .build();

    Command::new("su")
        .args(["-c", "true"])
        .as_user(USERNAME)
        .output(&env)
        .assert_success();
}

#[test]
#[cfg_attr(target_os = "freebsd", ignore = "FreeBSD doesn't use /etc/pam.d/su-l")]
fn su_dash_l_uses_correct_service_file() {
    let env = Env("")
        .file("/etc/pam.d/su-l", "auth sufficient pam_permit.so")
        .file("/etc/pam.d/su", "auth requisite pam_deny.so")
        .user(USERNAME)
        .build();

    Command::new("su")
        .args(["-l", "-c", "true"])
        .as_user(USERNAME)
        .output(&env)
        .assert_success();
}
