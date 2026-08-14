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
    let (_, account) = config
        .take_account(Some("example"))
        .expect("take account")
        .expect("account found");
    account.into()
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

#[test]
fn an_account_naming_no_transport_takes_every_default() {
    // Every transport field has a default, so an account holding only
    // its cycles reaches both, and the socket wins the tie.
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

    // Given none, a server binds the default transport alone, so no
    // port opens under an account nobody asked to expose.
    assert_eq!(account.addresses(&[]), vec![socket.clone()]);

    // Given some, it binds those, whatever the account marks default.
    assert_eq!(account.addresses(&[Transport::Tcp]), vec![tcp.clone()]);
    assert_eq!(
        account.addresses(&[Transport::UnixSocket, Transport::Tcp]),
        vec![socket, tcp]
    );
}
