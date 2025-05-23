use sudo_test::{Command, Env, TextFile};

#[test]
fn syslog_writer_should_not_hang() {
    let env = Env(TextFile("ALL ALL=(ALL:ALL) NOPASSWD: ALL").chmod("644")).build();

    let stdout = Command::new("sudo")
        .args(["env", "CC=clang-18", "CXX=clang++-18", "FOO=\"........................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................................\"", "whoami"])
        .output(&env)
        .stdout();

    assert_eq!(stdout, "root");
}

#[test]
fn no_permissions_should_not_violate_io_safety() {
    let env = Env(TextFile("ALL ALL=(ALL:ALL) NOPASSWD: ALL").chmod("644"))
        .file("/bin/foo", "#!/bin/sh") // File not executable
        .build();

    let output = Command::new("sudo").arg("/bin/foo").output(&env);

    assert!(!output.status().success());

    let stderr = output.stderr();
    assert!(!stderr.contains("IO Safety violation"), "{stderr}");

    assert_eq!(
        stderr,
        "sudo-rs: cannot execute '/usr/bin/foo': Permission denied (os error 13)"
    );
}
