//! # Watch tests
//!
//! `watch` run as a status bar runs it: stdout piped, events streamed as
//! they come, then one result once a signal stops it, or the failure once
//! the server goes away.

#![cfg(unix)]

mod common;

use std::{
    io::{BufRead, BufReader, Lines, Write},
    os::unix::net::UnixListener,
    process::{Child, ChildStdout, Command, Stdio},
    thread,
};

use serde_json::Value;

use crate::common::{comodoro, kill, scratch, serve};

/// Spawns `comodoro -c <config> <args> watch`, stdout piped.
fn watch(config: &std::path::Path, args: &[&str]) -> (Child, Lines<BufReader<ChildStdout>>) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_comodoro"))
        .arg("-c")
        .arg(config)
        .args(args)
        .arg("watch")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let lines = BufReader::new(child.stdout.take().unwrap()).lines();
    (child, lines)
}

fn parse(line: &str) -> Value {
    serde_json::from_str(line).unwrap_or_else(|err| panic!("{err}: {line}"))
}

#[test]
fn json_events_stream_then_a_signal_prints_the_result() {
    let scratch = serve();
    let (mut child, mut lines) = watch(&scratch.config, &["--json"]);

    let first = parse(&lines.next().unwrap().unwrap());
    assert_eq!(first["event"], "timer-changed", "{first}");
    assert_eq!(first["state"], "stopped", "{first}");

    let started = comodoro(&scratch.config, &["start"], b"");
    assert!(started.status.success());

    let change = parse(&lines.next().unwrap().unwrap());
    assert_eq!(change["event"], "timer-changed", "{change}");
    assert_eq!(change["state"], "running", "{change}");
    assert_eq!(change["cycle"]["name"], "Work", "{change}");

    kill(&child, "INT");

    let rest: Vec<Value> = lines.map(|line| parse(&line.unwrap())).collect();
    let (result, events) = rest.split_last().expect("a result line");

    assert!(events.iter().all(|event| event["event"] == "timer-changed"));
    assert_eq!(result, &serde_json::json!({ "stoppedBy": "signal" }));
    assert_eq!(child.wait().unwrap().code(), Some(0));
}

/// A status bar pipes the text: each line reaches it as it comes, though
/// stdout is no terminal.
#[test]
fn text_events_reach_a_pipe_then_sigterm_stops_the_watch() {
    let scratch = serve();
    let (mut child, mut lines) = watch(&scratch.config, &[]);

    assert_eq!(lines.next().unwrap().unwrap(), "OFF");

    kill(&child, "TERM");

    let rest: Vec<String> = lines.map(Result::unwrap).collect();
    assert_eq!(rest.last().map(String::as_str), Some("Watch stopped"));
    assert_eq!(child.wait().unwrap().code(), Some(0));
}

/// A server answering the subscription and the timer, then closing the
/// connection, as a server stopping does.
fn closing_server(scratch: &common::Scratch) {
    let listener = UnixListener::bind(&scratch.socket).unwrap();

    thread::spawn(move || {
        let (stream, _) = listener.accept().unwrap();
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut writer = stream;

        let answers = [
            r#"{"jsonrpc":"2.0","id":1,"result":{"subscribed":true}}"#,
            r#"{"jsonrpc":"2.0","id":2,"result":{"schedule":{"cycles":[{"name":"Work","duration":1500}],"loops":null},"state":"stopped","cycle":{"name":"Work","duration":1500},"startedAt":null,"elapsed":0}}"#,
        ];

        for answer in answers {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            writeln!(writer, "{answer}").unwrap();
        }
    });
}

#[test]
fn a_server_closing_the_connection_fails_coded_server_closed() {
    let scratch = scratch();
    closing_server(&scratch);

    let (mut child, lines) = watch(&scratch.config, &["--json"]);
    let lines: Vec<Value> = lines.map(|line| parse(&line.unwrap())).collect();

    assert_eq!(lines.len(), 2, "{lines:?}");
    assert_eq!(lines[0]["event"], "timer-changed", "{}", lines[0]);
    assert_eq!(lines[1]["code"], "server-closed", "{}", lines[1]);
    assert_eq!(child.wait().unwrap().code(), Some(1));
}
