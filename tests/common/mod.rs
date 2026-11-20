//! # Common test helpers
//!
//! A scratch account, optionally served by an in-process timer server on
//! its own socket, and the binary run against it.

#![allow(dead_code)]

use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Child, Command, Output, Stdio},
    thread,
};

use comodoro::{
    server::std::TimerServer,
    timer::{TimerCycle, TimerLoop, TimerSchedule},
    transport::TimerAddress,
};
use tempfile::TempDir;

/// A scratch directory holding an account file whose socket is in it.
pub struct Scratch {
    pub dir: TempDir,
    pub config: PathBuf,
    pub socket: PathBuf,
}

/// An account no server answers for.
pub fn scratch() -> Scratch {
    let dir = TempDir::new().unwrap();
    let socket = dir.path().join("comodoro.sock");
    let config = dir.path().join("config.toml");

    let account = format!(
        "[accounts.test]\ndefault = true\ncycles = [{{ name = \"Work\", duration = 1500 }}, {{ name = \"Rest\", duration = 300 }}]\nsocket.path = {:?}\n",
        socket.display().to_string(),
    );
    fs::write(&config, account).unwrap();

    Scratch {
        dir,
        config,
        socket,
    }
}

/// An account served by a timer server running in this process.
pub fn serve() -> Scratch {
    let scratch = scratch();

    let events = TimerServer {
        schedule: TimerSchedule {
            cycles: vec![TimerCycle::new("Work", 1500), TimerCycle::new("Rest", 300)],
            loops: TimerLoop::Infinite,
        },
        addresses: vec![TimerAddress::UnixSocket(scratch.socket.clone())],
    }
    .serve()
    .unwrap();

    thread::spawn(move || while events.recv().is_ok() {});

    scratch
}

/// Runs the binary on `config` with `args`, `input` on its stdin.
pub fn comodoro(config: &PathBuf, args: &[&str], input: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_comodoro"))
        .arg("-c")
        .arg(config)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

/// Sends `signal` to the child, as a status bar or a service manager
/// stopping it does.
pub fn kill(child: &Child, signal: &str) {
    let status = Command::new("kill")
        .arg(format!("-{signal}"))
        .arg(child.id().to_string())
        .status()
        .unwrap();

    assert!(status.success());
}
