use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn new_creates_requested_template_and_preserves_files() {
    let dir = tempdir().expect("tempdir");
    let project = dir.path().join("demo");
    Command::cargo_bin("forge")
        .expect("binary")
        .args([
            "--no-color",
            "new",
            project.to_str().expect("path"),
            "--template",
            "library",
            "--tests",
            "--standard",
            "cxx20",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created"));
    let source = project.join("src/main.cpp");
    std::fs::write(&source, "custom").expect("write custom source");
    Command::cargo_bin("forge")
        .expect("binary")
        .args(["new", project.to_str().expect("path"), "--force"])
        .assert()
        .success();
    assert_eq!(
        std::fs::read_to_string(source).expect("read source"),
        "custom"
    );
}

#[test]
fn invalid_name_is_rejected() {
    let dir = tempdir().expect("tempdir");
    Command::cargo_bin("forge")
        .expect("binary")
        .args(["new", dir.path().join("bad.name").to_str().expect("path")])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid project name"));
}
