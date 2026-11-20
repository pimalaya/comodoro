//! # Server tests
//!
//! `server start` run as a service manager runs it: stopped by a signal
//! with one result and its socket removed, and refused with a code when
//! its address is busy or its socket path refused.

#![cfg(unix)]

mod common;

use std::{
    fs,
    path::Path,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use serde_json::Value;

use crate::common::{Scratch, comodoro, kill, scratch, serve};

/// Spawns `comodoro -c <config> <args> server start`, and waits for its
/// socket to appear.
fn start(scratch: &Scratch, args: &[&str]) -> Child {
    let child = Command::new(env!("CARGO_BIN_EXE_comodoro"))
        .arg("-c")
        .arg(&scratch.config)
        .args(args)
        .args(["server", "start"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let deadline = Instant::now() + Duration::from_secs(10);

    while !scratch.socket.exists() {
        assert!(
            Instant::now() < deadline,
            "no socket at {:?}",
            scratch.socket
        );
        thread::sleep(Duration::from_millis(20));
    }

    child
}

fn failure(stdout: &[u8]) -> Value {
    let stdout = String::from_utf8_lossy(stdout);
    serde_json::from_str(&stdout).unwrap_or_else(|err| panic!("{err}: {stdout}"))
}

#[test]
fn sigterm_stops_the_server_with_its_result_and_removes_its_socket() {
    let scratch = scratch();
    let child = start(&scratch, &["--json"]);

    kill(&child, "TERM");
    let out = child.wait_with_output().unwrap();

    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "{\"stoppedBy\":\"signal\"}\n"
    );
    assert!(!scratch.socket.exists());
}

#[test]
fn sigint_stops_the_server_saying_so() {
    let scratch = scratch();
    let child = start(&scratch, &[]);

    kill(&child, "INT");
    let out = child.wait_with_output().unwrap();

    assert_eq!(out.status.code(), Some(0), "{out:?}");
    assert_eq!(String::from_utf8_lossy(&out.stdout), "Server stopped\n");
    assert!(!scratch.socket.exists());
}

/// A second server on a socket a live one holds fails, leaving the live
/// one serving.
#[test]
fn a_busy_socket_is_coded_address_busy() {
    let scratch = serve();

    let out = comodoro(&scratch.config, &["--json", "server", "start"], b"");
    assert_eq!(out.status.code(), Some(1), "{out:?}");

    let failure = failure(&out.stdout);
    assert_eq!(failure["code"], "address-busy", "{failure}");

    let error = failure["error"].as_str().unwrap_or_default();
    assert!(error.starts_with("Cannot listen at"), "{failure}");

    let get = comodoro(&scratch.config, &["get"], b"");
    assert!(get.status.success(), "{get:?}");
}

/// A socket path longer than a socket address holds is the
/// configuration's fault, not a missing server's.
#[test]
fn a_socket_path_too_long_is_coded_socket_path_invalid() {
    let scratch = scratch();
    let socket = scratch.dir.path().join("s".repeat(200));
    let toml = format!(
        "[accounts.test]\ndefault = true\ncycles = [{{ name = \"Work\", duration = 1500 }}]\nsocket.path = {:?}\n",
        socket.display().to_string(),
    );
    fs::write(&scratch.config, toml).unwrap();

    for args in [&["--json", "get"][..], &["--json", "server", "start"]] {
        let out = comodoro(&scratch.config, args, b"");
        assert_eq!(out.status.code(), Some(1), "{args:?}: {out:?}");

        let failure = failure(&out.stdout);
        assert_eq!(failure["code"], "socket-path-invalid", "{failure}");

        let error = failure["error"].as_str().unwrap_or_default();
        assert!(error.starts_with("Cannot"), "{failure}");
    }

    assert!(!Path::new(&socket).exists());
}
