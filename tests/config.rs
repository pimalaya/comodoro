//! # Account configuration tests
//!
//! Coverage of the account configuration, from the TOML shapes
//! config.sample.toml documents to the addresses they resolve a command
//! to once folded into an account.

use comodoro::{
    cli::{
        account::Account,
        config::{Config, LOCALHOST, TCP_PORT},
        transport::Transport,
    },
    transport::TimerAddress,
};
use pimalaya_config::toml::TomlConfig;

/// Deserializes `toml`, takes its only account and resolves it the way
/// the dispatch layer does.
fn account(toml: &str) -> Account {
    let mut config: Config = toml::from_str(toml).expect("deserialize config");
    let (_, account) = config.take_account(Some("example")).expect("take account");
    account.try_into().expect("resolve account")
}

#[test]
fn a_v1_account_file_loads_unchanged() {
    let account = account(
        r#"
        [accounts.example]
        default = true
        unix-socket.path = "/tmp/comodoro.sock"
        unix-socket.default = true
        tcp.host = "127.0.0.1"
        tcp.port = 9999
        cycles = [{ name = "Work", duration = 1500 }]
        "#,
    );

    assert_eq!(
        account.address(None),
        TimerAddress::UnixSocket("/tmp/comodoro.sock".into())
    );
    assert_eq!(
        account.address(Some(Transport::Tcp)),
        TimerAddress::Tcp {
            host: "127.0.0.1".into(),
            port: 9999
        }
    );
}

/// Every transport field has a default, so an account holding only its
/// cycles reaches both transports, and the socket wins the tie.
#[test]
fn an_account_naming_no_transport_takes_every_default() {
    let account = account(
        r#"
        [accounts.example]
        cycles = [{ name = "Work", duration = 1500 }]
        "#,
    );

    assert!(matches!(account.address(None), TimerAddress::UnixSocket(_)));
    assert!(matches!(
        account.address(Some(Transport::UnixSocket)),
        TimerAddress::UnixSocket(_)
    ));
    assert_eq!(
        account.address(Some(Transport::Tcp)),
        TimerAddress::Tcp {
            host: LOCALHOST.into(),
            port: TCP_PORT
        }
    );
}

#[test]
fn tcp_takes_the_default_only_when_the_socket_leaves_it() {
    let untied = account(
        r#"
        [accounts.example]
        tcp.default = true
        cycles = [{ name = "Work", duration = 1500 }]
        "#,
    );

    assert_eq!(
        untied.address(None),
        TimerAddress::Tcp {
            host: LOCALHOST.into(),
            port: TCP_PORT
        }
    );

    let tied = account(
        r#"
        [accounts.example]
        socket.default = true
        tcp.default = true
        cycles = [{ name = "Work", duration = 1500 }]
        "#,
    );

    assert!(matches!(tied.address(None), TimerAddress::UnixSocket(_)));
}

/// Given no transport a server binds the default one alone, so no port
/// opens under an account nobody asked to expose. Given some, it binds
/// those, whatever the account marks default.
#[test]
fn a_server_binds_the_transports_it_is_given() {
    let account = account(
        r#"
        [accounts.example]
        socket.path = "/tmp/comodoro.sock"
        tcp.port = 19999
        cycles = [{ name = "Work", duration = 1500 }]
        "#,
    );

    let tcp = TimerAddress::Tcp {
        host: LOCALHOST.into(),
        port: 19999,
    };
    let socket = TimerAddress::UnixSocket("/tmp/comodoro.sock".into());

    assert_eq!(account.addresses(&[]), vec![socket.clone()]);
    assert_eq!(account.addresses(&[Transport::Tcp]), vec![tcp.clone()]);
    assert_eq!(
        account.addresses(&[Transport::UnixSocket, Transport::Tcp]),
        vec![socket, tcp]
    );
}

/// Runs the binary's `get` on an account file holding `toml`, and returns
/// the failure it prints under `--json`.
fn refusal(toml: &str) -> String {
    let dir = tempfile::TempDir::new().unwrap();
    let path = dir.path().join("config.toml");
    let socket = dir.path().join("none.sock");
    let toml = format!("{toml}\nsocket.path = {:?}\n", socket.display().to_string());
    std::fs::write(&path, toml).unwrap();

    let output = std::process::Command::new(env!("CARGO_BIN_EXE_comodoro"))
        .arg("-c")
        .arg(&path)
        .args(["--json", "get"])
        .output()
        .unwrap();

    let failure: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
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
        format!(
            "[accounts.example]\ndefault = true\ncycles = [{{ name = \"Work\", duration = 1500, wrong-key = 1 }}]"
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

/// The per-second hook of 2.x is refused naming its new name and the
/// migration guide, rather than as a name no event fires.
#[test]
fn the_running_hook_of_v2_is_refused_by_name() {
    let failure = refusal(
        r#"
        [accounts.example]
        default = true
        cycles = [{ name = "Work", duration = 1500 }]
        hooks.on-work-running.command = "true"
        "#,
    );

    assert!(failure.contains("on-work-tick"), "{failure}");
    assert!(failure.contains("MIGRATION.md"), "{failure}");
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

/// The 0.x keys are refused naming what replaced them and the migration
/// guide, rather than as any unknown key.
#[test]
fn the_keys_of_v0_are_refused_by_name() {
    let cycles = r#"cycles = [{ name = "Work", duration = 1500 }]"#;

    let cases = [
        (format!("[presets.example]\n{cycles}"), "`accounts`"),
        (
            format!("[accounts.example]\ndefault = true\n{cycles}\npreset = \"pomodoro\""),
            "config.sample.toml",
        ),
        (
            format!("[accounts.example]\ndefault = true\n{cycles}\ntimer-precision = \"minute\""),
            "`precision`",
        ),
    ];

    for (toml, replacement) in cases {
        let failure = refusal(&toml);
        assert!(failure.contains(replacement), "{toml}\n{failure}");
        assert!(failure.contains("MIGRATION.md"), "{toml}\n{failure}");
    }
}
