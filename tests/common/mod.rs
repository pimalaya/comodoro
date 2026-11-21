//! # Common test helpers
//!
//! A scratch account, optionally served by a `comodoro server start`
//! child on its own socket, and the binary run against it.

#![allow(dead_code)]

use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Child, Command, Output, Stdio},
    thread,
    time::{Duration, Instant},
};

use tempfile::TempDir;

/// A scratch directory holding an account file whose socket is in it,
/// and the server serving it, killed on drop.
pub struct Scratch {
    pub dir: TempDir,
    pub config: PathBuf,
    pub socket: PathBuf,
    pub server: Option<Child>,
}

impl Drop for Scratch {
    fn drop(&mut self) {
        if let Some(server) = &mut self.server {
            let _ = server.kill();
            let _ = server.wait();
        }
    }
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
        server: None,
    }
}

/// An account served by `comodoro server start`, once its socket exists.
pub fn serve() -> Scratch {
    let mut scratch = scratch();

    let server = Command::new(env!("CARGO_BIN_EXE_comodoro"))
        .arg("-c")
        .arg(&scratch.config)
        .args(["server", "start"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    scratch.server = Some(server);

    let deadline = Instant::now() + Duration::from_secs(10);

    while !scratch.socket.exists() {
        assert!(
            Instant::now() < deadline,
            "no socket at {:?}",
            scratch.socket
        );
        thread::sleep(Duration::from_millis(20));
    }

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
