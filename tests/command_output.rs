use std::process::Command;

// test check trim subcommand help
#[test]
fn test_help() {
    let status = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("trim")
        .arg("help")
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

// test check list subcommand help
#[test]
fn test_list_help() {
    let status = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("trim")
        .arg("help")
        .arg("list")
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

// test check remove subcommand help
#[test]
fn test_remove_help() {
    let status = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("trim")
        .arg("help")
        .arg("remove")
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

// test check config subcommand help
#[test]
fn test_config_help() {
    let status = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("trim")
        .arg("help")
        .arg("config")
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

// test check git subcommand help
#[test]
fn test_git_help() {
    let status = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("trim")
        .arg("help")
        .arg("git")
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}

// test check registry subcommand help
#[test]
fn test_registry_help() {
    let status = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("trim")
        .arg("help")
        .arg("registry")
        .status()
        .expect("failed to execute process");
    assert!(status.success());
}
