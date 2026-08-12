//! Coverage of the account configuration, and of the transport it
//! resolves a command to.

use comodoro::{
    cli::{
        config::{ComodoroAccountConfig, ComodoroConfig},
        transport::ComodoroTransport,
    },
    transport::TimerAddress,
};
use pimalaya_config::toml::TomlConfig;

/// Deserializes `toml` and takes its only account.
fn account(toml: &str) -> ComodoroAccountConfig {
    let mut config: ComodoroConfig = toml::from_str(toml).expect("deserialize config");
    let (_, account) = config
        .take_account(Some("example"))
        .expect("take account")
        .expect("account found");
    account
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
        account.address(None).unwrap(),
        TimerAddress::UnixSocket("/tmp/comodoro.sock".into())
    );
    assert_eq!(
        account.address(Some(ComodoroTransport::Tcp)).unwrap(),
        TimerAddress::Tcp {
            host: "127.0.0.1".into(),
            port: 9999
        }
    );
}

#[test]
fn tcp_takes_the_default_only_when_the_socket_leaves_it() {
    let untied = account(
        r#"
        [accounts.example]
        tcp.port = 9999
        tcp.default = true
        cycles = [{ name = "Work", duration = 1500 }]
        "#,
    );

    // NOTE: the host defaults to loopback, which is the only default the TCP
    // table has: an account without a port opens none.
    assert_eq!(
        untied.address(None).unwrap(),
        TimerAddress::Tcp {
            host: "127.0.0.1".into(),
            port: 9999
        }
    );

    let tied = account(
        r#"
        [accounts.example]
        socket.default = true
        tcp.port = 9999
        tcp.default = true
        cycles = [{ name = "Work", duration = 1500 }]
        "#,
    );

    assert!(matches!(
        tied.address(None).unwrap(),
        TimerAddress::UnixSocket(_)
    ));
}

#[test]
fn an_account_without_tcp_binds_the_socket_alone() {
    let account = account(
        r#"
        [accounts.example]
        cycles = [{ name = "Work", duration = 1500 }]
        "#,
    );

    let addresses = account.addresses(&[]).unwrap();
    assert_eq!(addresses.len(), 1);
    assert!(matches!(addresses[0], TimerAddress::UnixSocket(_)));

    let err = account
        .address(Some(ComodoroTransport::Tcp))
        .unwrap_err()
        .to_string();
    assert_eq!(err, "Missing TCP configuration");
}

#[test]
fn a_server_binds_every_configured_transport() {
    let account = account(
        r#"
        [accounts.example]
        socket.path = "/tmp/comodoro.sock"
        tcp.port = 9999
        cycles = [{ name = "Work", duration = 1500 }]
        "#,
    );

    assert_eq!(
        account.addresses(&[]).unwrap(),
        [
            TimerAddress::UnixSocket("/tmp/comodoro.sock".into()),
            TimerAddress::Tcp {
                host: "127.0.0.1".into(),
                port: 9999
            },
        ]
    );

    assert_eq!(
        account.addresses(&[ComodoroTransport::Tcp]).unwrap(),
        [TimerAddress::Tcp {
            host: "127.0.0.1".into(),
            port: 9999
        }]
    );
}
