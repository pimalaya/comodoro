//! # Account tests
//!
//! The `account` group: `list` reads the configuration alone, `check`
//! reads the timer over each transport the account declares, one row
//! each, a failed row exiting 3.

#![cfg(unix)]

mod common;

use std::{fs, net::TcpListener};

use serde_json::{Value, json};

use crate::common::{comodoro, scratch, serve};

fn parse(stdout: &[u8]) -> Value {
    let stdout = String::from_utf8_lossy(stdout);
    serde_json::from_str(&stdout).unwrap_or_else(|err| panic!("{err}: {stdout}"))
}

/// A TCP port nothing listens on.
fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.local_addr().unwrap().port()
}

/// A transport counts as declared when the account holds its table or
/// talks over it by default, so no account lists a transport it never
/// mentions beside the socket every command falls back to.
#[test]
fn the_accounts_list_with_the_transports_they_declare() {
    let scratch = scratch();
    let cycles = r#"cycles = [{ name = "Work", duration = 1500 }]"#;
    let toml = format!(
        "[accounts.b]\ndefault = true\n{cycles}\n\n[accounts.a]\n{cycles}\ntcp.port = 1234\n\n[accounts.c]\n{cycles}\ntcp.default = true\n"
    );
    fs::write(&scratch.config, toml).unwrap();

    let out = comodoro(&scratch.config, &["--json", "account", "list"], b"");
    assert_eq!(out.status.code(), Some(0), "{out:?}");

    let row = |name: &str, default: bool, backends: &[&str]| {
        let not_built: [&str; 0] = [];
        json!({ "name": name, "default": default, "backends": backends, "notBuilt": not_built })
    };

    let expected = json!({
        "accounts": [
            row("a", false, &["socket", "tcp"]),
            row("b", true, &["socket"]),
            row("c", false, &["tcp"]),
        ]
    });

    assert_eq!(parse(&out.stdout), expected);
}

#[test]
fn a_check_reads_the_timer_over_each_declared_transport() {
    let scratch = serve();

    let out = comodoro(&scratch.config, &["--json", "account", "check"], b"");
    assert_eq!(out.status.code(), Some(0), "{out:?}");

    let expected = json!({
        "accounts": [{
            "account": "test",
            "backends": [
                { "backend": "socket", "ok": true, "code": null, "error": null, "tls": null }
            ]
        }]
    });

    assert_eq!(parse(&out.stdout), expected);
}

/// Every row prints whatever the others were, and a failed one turns the
/// exit status into 3, the report being the output.
#[test]
fn a_transport_nobody_serves_fails_its_row_exiting_3() {
    let scratch = serve();
    let mut toml = fs::read_to_string(&scratch.config).unwrap();
    toml.push_str(&format!("tcp.port = {}\n", free_port()));
    fs::write(&scratch.config, toml).unwrap();

    let out = comodoro(&scratch.config, &["--json", "account", "check"], b"");
    assert_eq!(out.status.code(), Some(3), "{out:?}");

    let output = parse(&out.stdout);
    let rows = &output["accounts"][0]["backends"];

    assert_eq!(rows[0]["backend"], "socket", "{output}");
    assert_eq!(rows[0]["ok"], true, "{output}");
    assert_eq!(rows[1]["backend"], "tcp", "{output}");
    assert_eq!(rows[1]["ok"], false, "{output}");
    assert_eq!(rows[1]["code"], "server-unreachable", "{output}");

    let out = comodoro(&scratch.config, &["account", "check"], b"");
    assert_eq!(out.status.code(), Some(3), "{out:?}");

    let stdout = String::from_utf8(out.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();

    assert!(lines[0].contains("test"), "{stdout}");
    assert!(
        lines[1].contains("socket") && lines[1].contains("OK"),
        "{stdout}"
    );
    assert!(
        lines[2].contains("tcp") && lines[2].contains("FAIL"),
        "{stdout}"
    );
}
