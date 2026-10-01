use serde_json::Value;
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};

fn repository_root() -> PathBuf {
    std::env::current_dir().expect("Cargo test working directory")
}

fn binary() -> PathBuf {
    let test_executable = std::env::current_exe().expect("current test executable");
    test_executable
        .parent()
        .and_then(|deps| deps.parent())
        .expect("Cargo target profile directory")
        .join(format!(
            "hat-specifications{}",
            std::env::consts::EXE_SUFFIX
        ))
}

fn run(arguments: &[&str]) -> Output {
    Command::new(binary())
        .args(arguments)
        .current_dir(repository_root())
        .output()
        .expect("CLI starts")
}

fn json(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("stdout is one JSON document")
}

#[test]
fn happy_path_commands_are_deterministic_and_side_effect_free() {
    for arguments in [
        vec!["doctor"],
        vec!["validate", "examples/source-curator.hat.toml"],
        vec!["validate-package", "examples/source-curator.package.json"],
        vec![
            "fitting",
            "examples/source-curator.hat.toml",
            "examples/editor.profile.toml",
        ],
    ] {
        let first = run(&arguments);
        let second = run(&arguments);
        assert!(first.status.success(), "{arguments:?}");
        assert_eq!(first.stdout, second.stdout, "{arguments:?}");
        let document = json(&first);
        assert_eq!(document["schema"], "hathq://hat-specifications/result/v1");
        assert_eq!(document["ok"], true);
        assert_eq!(document["external_actions"], false);
    }
}

#[test]
fn cli_rejects_unknown_classification_with_machine_readable_failure() {
    let source = include_str!("../examples/source-curator.hat.toml")
        .replace("[\"public\", \"internal\"]", "[\"unknown\"]");
    let mut path = std::env::temp_dir();
    path.push(format!(
        "hat-specifications-invalid-{}.toml",
        std::process::id()
    ));
    fs::write(&path, source).expect("temporary fixture");
    let output = run(&["validate", path.to_str().expect("UTF-8 path")]);
    let _ = fs::remove_file(path);

    assert_eq!(output.status.code(), Some(2));
    let document = json(&output);
    assert_eq!(document["ok"], false);
    assert_eq!(document["external_actions"], false);
    assert!(document["findings"].as_array().is_some_and(|findings| {
        findings.iter().any(|finding| {
            finding
                .as_str()
                .is_some_and(|text| text.contains("unknown"))
        })
    }));
}

#[test]
fn unsupported_arguments_have_a_stable_nonzero_contract() {
    let output = run(&["unknown"]);
    assert_eq!(output.status.code(), Some(2));
    let document = json(&output);
    assert_eq!(document["command"], "usage");
    assert_eq!(document["ok"], false);
}
