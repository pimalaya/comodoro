//! # Output tests
//!
//! What the binary prints under `--json`, run against a server owning a
//! timer on a scratch socket.

mod common;

use std::{
    env, fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
};

use serde_json::Value;

use crate::common::serve;

static NEXT_SOCKET: AtomicUsize = AtomicUsize::new(0);

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
    let scratch = serve();
    let config = &scratch.config;
    let timer = json(config, &["start"]);

    assert_eq!(timer["state"], "running", "{timer}");
    assert_eq!(timer["cycle"]["name"], "Work", "{timer}");
    assert!(is_rfc3339_utc(&timer["startedAt"]), "{timer}");
}

#[test]
fn every_control_answers_the_timer_after_it() {
    let scratch = serve();
    let config = &scratch.config;

    json(config, &["start"]);
    assert_eq!(json(config, &["pause"])["state"], "paused");
    assert_eq!(json(config, &["resume"])["state"], "running");
    assert_eq!(json(config, &["set", "60"])["cycle"]["duration"], 60);

    let stopped = json(config, &["stop"]);
    assert_eq!(stopped["state"], "stopped", "{stopped}");
    assert_eq!(stopped["startedAt"], Value::Null, "{stopped}");
}

#[test]
fn get_prints_the_start_as_an_instant() {
    let scratch = serve();
    let config = &scratch.config;

    json(config, &["start"]);
    let timer = json(config, &["get"]);
    assert!(is_rfc3339_utc(&timer["startedAt"]), "{timer}");
}

/// A configuration in a scratch directory of its own, and the socket its
/// account talks over.
fn config_at(name: &str) -> (PathBuf, PathBuf) {
    let id = NEXT_SOCKET.fetch_add(1, Ordering::Relaxed);
    let dir = env::temp_dir().join(format!("comodoro-{name}-{}-{id}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("create scratch dir");

    let socket = dir.join("comodoro.sock");
    let config = dir.join("config.toml");

    let account = format!(
        "[accounts.test]\ndefault = true\ncycles = [{{ name = \"Work\", duration = 1500 }}]\nsocket.path = {:?}\n",
        socket.display().to_string(),
    );
    fs::write(&config, account).expect("write config");

    (config, socket)
}

/// A server answering its first request with `answer`, or closing the
/// connection without a word when there is none.
#[cfg(unix)]
fn fake_server(socket: &PathBuf, answer: Option<&'static str>) {
    use std::{
        io::{BufRead, BufReader, Write},
        os::unix::net::UnixListener,
    };

    let listener = UnixListener::bind(socket).expect("bind fake server");

    thread::spawn(move || {
        let (stream, _) = listener.accept().expect("accept");
        let mut reader = BufReader::new(stream.try_clone().expect("clone"));
        let mut writer = stream;
        let mut line = String::new();
        reader.read_line(&mut line).expect("read request");

        if let Some(answer) = answer {
            writeln!(writer, "{answer}").expect("write answer");
        }
    });
}

/// Runs the binary under `--json` expecting a failure, and returns the
/// code of the failure object it prints.
fn failure_code(config: &PathBuf, args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_comodoro"))
        .arg("-c")
        .arg(config)
        .arg("--json")
        .args(args)
        .output()
        .expect("run comodoro");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    let failure: Value =
        serde_json::from_str(&stdout).unwrap_or_else(|err| panic!("{err}: {stdout}"));
    failure["code"].clone()
}

#[test]
fn no_server_running_is_coded_unreachable() {
    let (config, _) = config_at("unreachable");
    assert_eq!(failure_code(&config, &["get"]), "server-unreachable");
}

#[cfg(unix)]
#[test]
fn a_server_closing_before_its_answer_is_coded_closed() {
    let (config, socket) = config_at("closed");
    fake_server(&socket, None);
    assert_eq!(failure_code(&config, &["get"]), "server-closed");
}

#[cfg(unix)]
#[test]
fn a_json_rpc_error_answer_is_coded_server_error() {
    let (config, socket) = config_at("server-error");
    let answer = r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32601,"message":"Method not found"}}"#;
    fake_server(&socket, Some(answer));
    assert_eq!(failure_code(&config, &["get"]), "server-error");
}

#[cfg(unix)]
#[test]
fn a_server_of_another_version_is_coded_mismatch() {
    let (config, socket) = config_at("mismatch");
    let answer = r#"{"jsonrpc":"2.0","id":1,"result":{"schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":"Infinite"},"state":"Stopped","cycle":{"name":"Work","duration":1500},"startedAt":null,"elapsed":0}}"#;
    fake_server(&socket, Some(answer));
    assert_eq!(failure_code(&config, &["get"]), "server-mismatch");
}

/// A configuration in a scratch directory holding `toml`.
fn config_of(name: &str, toml: &str) -> PathBuf {
    let (config, _) = config_at(name);
    fs::write(&config, toml).expect("write config");
    config
}

fn account(name: &str, default: bool) -> String {
    format!(
        "[accounts.{name}]\ndefault = {default}\ncycles = [{{ name = \"Work\", duration = 1500 }}]\n"
    )
}

/// `default` is a name like any other: it takes no account marked default,
/// a missing `-a` does.
#[test]
fn a_missing_account_is_coded_account_missing() {
    let config = config_of("account-missing", &account("a", true));

    let output = Command::new(env!("CARGO_BIN_EXE_comodoro"))
        .arg("-c")
        .arg(&config)
        .args(["--json", "-a", "default", "get"])
        .output()
        .expect("run comodoro");

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    let failure: Value =
        serde_json::from_str(&stdout).unwrap_or_else(|err| panic!("{err}: {stdout}"));
    assert_eq!(failure["code"], "account-missing", "{failure}");

    let error = failure["error"].as_str().unwrap_or_default();
    assert!(error.contains("omit `-a`"), "{failure}");
}

#[test]
fn no_default_account_is_coded_account_default_missing() {
    let config = config_of("account-default-missing", &account("a", false));
    assert_eq!(failure_code(&config, &["get"]), "account-default-missing");
}

#[test]
fn several_default_accounts_are_coded_account_default_several() {
    let toml = account("a", true) + &account("b", true);
    let config = config_of("account-default-several", &toml);
    assert_eq!(failure_code(&config, &["get"]), "account-default-several");
}
