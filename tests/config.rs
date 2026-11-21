//! # Configuration refusal tests
//!
//! The account files the binary refuses, read through `get` under
//! `--json`, and the old spellings it still reads.

use std::{fs, process::Command};

use serde_json::Value;
use tempfile::TempDir;

/// Runs the binary's `get` on an account file holding `toml`, and returns
/// the failure it prints under `--json`.
fn refusal(toml: &str) -> String {
    let dir = TempDir::new().unwrap();
    let path = dir.path().join("config.toml");
    let socket = dir.path().join("none.sock");
    let toml = format!("{toml}\nsocket.path = {:?}\n", socket.display().to_string());
    fs::write(&path, toml).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_comodoro"))
        .arg("-c")
        .arg(&path)
        .args(["--json", "get"])
        .output()
        .unwrap();

    let failure: Value = serde_json::from_slice(&output.stdout).unwrap();
    failure.to_string()
}

#[test]
fn a_hook_no_event_fires_is_refused() {
    let failure = refusal(
        r#"
        [accounts.example]
        default = true
        cycles = [{ name = "Work", duration = 1500 }]
        hooks.on-wrok-end.command = "true"
        "#,
    );

    assert!(failure.contains("on-wrok-end"), "{failure}");
    assert!(failure.contains("work"), "{failure}");
}

#[test]
fn an_account_without_cycles_is_refused() {
    let failure = refusal(
        r#"
        [accounts.example]
        default = true
        cycles = []
        "#,
    );

    assert!(failure.contains("no cycle"), "{failure}");
}

#[cfg(feature = "notify")]
#[test]
fn a_cycle_variable_on_a_timer_hook_is_refused() {
    let failure = refusal(
        r#"
        [accounts.example]
        default = true
        cycles = [{ name = "Work", duration = 1500 }]
        hooks.on-timer-start.notify.summary = "$cycleName started"
        "#,
    );

    assert!(failure.contains("on-timer-start"), "{failure}");
    assert!(failure.contains("cycleName"), "{failure}");
}

/// A key no configuration reads is refused at every level, naming it,
/// rather than ignored.
#[test]
fn an_unknown_key_is_refused_at_every_level() {
    let cycles = r#"cycles = [{ name = "Work", duration = 1500 }]"#;
    let account = format!("[accounts.example]\ndefault = true\n{cycles}");

    let mut tomls = vec![
        format!("wrong-key = true\n{account}"),
        format!("{account}\nwrong-key = true"),
        format!("{account}\ntcp.wrong-key = true"),
        format!("{account}\nsocket.wrong-key = true"),
        format!("{account}\nhooks.on-work-end.command = \"true\"\nhooks.on-work-end.wrong-key = 1"),
        String::from(
            "[accounts.example]\ndefault = true\ncycles = [{ name = \"Work\", duration = 1500, wrong-key = 1 }]",
        ),
    ];

    if cfg!(feature = "notify") {
        tomls.push(format!(
            "{account}\nhooks.on-work-end.notify.summary = \"Done\"\nhooks.on-work-end.notify.wrong-key = 1"
        ));
    }

    for toml in tomls {
        let failure = refusal(&toml);
        assert!(failure.contains("wrong-key"), "{toml}\n{failure}");
    }
}

/// The per-second hook of 2.x is refused as any name no event fires,
/// never by name.
#[test]
fn the_running_hook_of_v2_is_a_name_no_event_fires() {
    let failure = refusal(
        r#"
        [accounts.example]
        default = true
        cycles = [{ name = "Work", duration = 1500 }]
        hooks.on-work-running.command = "true"
        "#,
    );

    assert!(failure.contains("matches no event"), "{failure}");
    assert!(!failure.contains("MIGRATION.md"), "{failure}");
}

/// `cmd`, the 1.x spelling, reads as the alias of `command`, as wherever
/// the family configures a command. A hook holding no command would be
/// refused as doing nothing, so the account resolving and `get` failing
/// only for want of a server shows `cmd` set the command.
#[test]
fn the_cmd_hook_key_reads_as_command() {
    let failure = refusal(
        r#"
        [accounts.example]
        default = true
        cycles = [{ name = "Work", duration = 1500 }]
        hooks.on-work-end.cmd = "true"
        "#,
    );

    assert!(
        failure.contains(r#""code":"server-unreachable""#),
        "{failure}"
    );
}

/// The 0.x keys are refused as any unknown key, by serde alone, never by
/// name.
#[test]
fn the_keys_of_v0_are_refused_as_unknown_keys() {
    let cycles = r#"cycles = [{ name = "Work", duration = 1500 }]"#;

    let cases = [
        (format!("[presets.example]\n{cycles}"), "`presets`"),
        (
            format!("[accounts.example]\ndefault = true\n{cycles}\npreset = \"pomodoro\""),
            "`preset`",
        ),
        (
            format!("[accounts.example]\ndefault = true\n{cycles}\ntimer-precision = \"minute\""),
            "`timer-precision`",
        ),
    ];

    for (toml, key) in cases {
        let failure = refusal(&toml);
        assert!(failure.contains("unknown field"), "{toml}\n{failure}");
        assert!(failure.contains(key), "{toml}\n{failure}");
        assert!(!failure.contains("MIGRATION.md"), "{toml}\n{failure}");
    }
}
