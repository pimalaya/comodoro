//! # Output tests
//!
//! What the binary prints under `--json`, run against a server owning a
//! timer on a scratch socket.

use std::{
    env, fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

use comodoro::{
    server::std::TimerServer,
    timer::{TimerCycle, TimerLoop, TimerSchedule},
    transport::TimerAddress,
};
use serde_json::Value;

static NEXT_SOCKET: AtomicUsize = AtomicUsize::new(0);

/// Starts a server on a socket unique to this test, and returns the
/// configuration file pointing at it.
fn serve() -> PathBuf {
    let id = NEXT_SOCKET.fetch_add(1, Ordering::Relaxed);
    let dir = env::temp_dir().join(format!("comodoro-output-{}-{id}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create scratch dir");

    let socket = dir.join("comodoro.sock");
    let config = dir.join("config.toml");

    let account = format!(
        "[accounts.test]\ndefault = true\ncycles = [{{ name = \"Work\", duration = 1500 }}]\nsocket.path = {:?}\n",
        socket.display().to_string(),
    );
    fs::write(&config, account).expect("write config");

    let events = TimerServer {
        schedule: TimerSchedule {
            cycles: vec![TimerCycle::new("Work", 1500)],
            loops: TimerLoop::Infinite,
        },
        addresses: vec![TimerAddress::UnixSocket(socket)],
    }
    .serve()
    .expect("serve timer");

    thread::spawn(move || while events.recv().is_ok() {});

    config
}

/// Runs the binary with the given arguments under `--json`, and parses
/// the one value it prints.
fn json(config: &PathBuf, args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_comodoro"))
        .arg("-c")
        .arg(config)
        .arg("--json")
        .args(args)
        .output()
        .expect("run comodoro");

    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    serde_json::from_str(&stdout).unwrap_or_else(|err| panic!("{err}: {stdout}"))
}

fn is_rfc3339_utc(value: &Value) -> bool {
    let Some(instant) = value.as_str() else {
        return false;
    };

    instant.len() == 20 && instant.ends_with('Z') && instant.as_bytes()[10] == b'T'
}

#[test]
fn a_started_timer_answers_the_timer() {
    let config = serve();
    let timer = json(&config, &["start"]);

    assert_eq!(timer["state"], "running", "{timer}");
    assert_eq!(timer["cycle"]["name"], "Work", "{timer}");
    assert!(is_rfc3339_utc(&timer["startedAt"]), "{timer}");
}

#[test]
fn every_control_answers_the_timer_after_it() {
    let config = serve();

    json(&config, &["start"]);
    assert_eq!(json(&config, &["pause"])["state"], "paused");
    assert_eq!(json(&config, &["resume"])["state"], "running");
    assert_eq!(json(&config, &["set", "60"])["cycle"]["duration"], 60);

    let stopped = json(&config, &["stop"]);
    assert_eq!(stopped["state"], "stopped", "{stopped}");
    assert_eq!(stopped["startedAt"], Value::Null, "{stopped}");
}

#[test]
fn get_prints_the_start_as_an_instant() {
    let config = serve();

    json(&config, &["start"]);
    let timer = json(&config, &["get"]);
    assert!(is_rfc3339_utc(&timer["startedAt"]), "{timer}");
}
