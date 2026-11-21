//! # Comodoro
//!
//! A CLI sharing a timer: one server owns it, any number of clients
//! drive it and watch it. The timer logic is a pure state machine
//! performing no I/O and reading no clock, so the interesting behaviour
//! is testable without a socket and without waiting for time to pass.
//!
//! ## Layers
//!
//! The contract sits at the bottom and performs no I/O. [`timer`] holds
//! the state machine, [`jsonrpc20`] the JSON-RPC 2.0 envelope and
//! [`protocol`] the Comodoro method surface expressed in it. A change to
//! what travels on the wire starts there, documented in cairn/spec/.
//!
//! The blocking layer comes next. [`transport`] resolves, opens and
//! accepts connections, [`client`] drives a server over one of them, and
//! [`server`] owns the timer, answers requests and pushes notifications.
//!
//! The CLI lives under [`cli`]. [`cli`] holds the command grammar and
//! the first-run welcome, [`cli::config`] the TOML document,
//! [`cli::account`] the view a command runs against, [`cli::hook`] the
//! event reactions, [`cli::json_schema`] the schemas of the output.
//!
//! [`cli::error`] holds the codes of the failures. [`cli::repl`] runs the
//! commands over one account in a session, and [`cli::mcp`] serves them
//! to an agent as Model Context Protocol tools, both on pimalaya-cli's
//! shared core. One module per command sits under [`cli::client`] and
//! [`cli::server`].
//!
//! The configuration belongs to the user: Comodoro never writes it, and
//! an account starts from config.sample.toml, one of whose cycle presets
//! the user uncomments.
//!
//! ## Protocol
//!
//! Client and server speak [JSON-RPC 2.0] framed as NDJSON, one compact
//! JSON value per line, over a Unix domain socket or over TCP. The
//! specification defines the payload alone, so the protocol is
//! implementable in any language, without reading a line of Rust.
//!
//! Requests flow client to server and are named after the imperative
//! that performs them.
//!
//! | Method | Parameters | Result |
//! |---|---|---|
//! | `timer.get` | none | the timer |
//! | `timer.start` | none | the events it emitted |
//! | `timer.pause` | none | the events it emitted |
//! | `timer.resume` | none | the events it emitted |
//! | `timer.stop` | none | the events it emitted |
//! | `timer.set` | `duration` in seconds | the events it emitted |
//! | `timer.subscribe` | none | whether the connection is subscribed |
//! | `timer.unsubscribe` | none | whether the connection is subscribed |
//!
//! Notifications flow the other way, are named after the past tense of
//! what just happened, and reach only the connections that asked for
//! them with `timer.subscribe`. Naming the two directions differently is
//! what keeps a request and a notification method from ever colliding.
//!
//! A subscriber receives `timer.started`, `timer.began`,
//! `timer.ticked`, `timer.durationSet`, `timer.paused`,
//! `timer.resumed`, `timer.ended` and `timer.stopped`, each carrying the
//! cycle it concerns except the two timer-wide ones.
//!
//! Failures come back as the standard codes, and only those.
//!
//! | Code | Meaning |
//! |---|---|
//! | -32700 | the payload is not valid JSON |
//! | -32600 | the request is malformed |
//! | -32601 | the method is unknown |
//! | -32602 | the parameters are missing or wrong |
//! | -32603 | the server failed internally |
//! | -32000 to -32099 | reserved for Comodoro, unused today |
//!
//! ## Entry point
//!
//! [`main`] parses the command line, wires the logger and the printer
//! from pimalaya-cli, then runs the parsed command, or meets a bare
//! invocation through pimalaya-cli. A person missing a configuration
//! meets the welcome on stderr, a script or a JSON caller the error.
//!
//! A failure prints the failure object and exits 1. A command line the
//! grammar refuses exits 2, before any configuration is read, its
//! failure object coded `usage-invalid` on stdout under `--json`.
//!
//! ## Where to look next
//!
//! The unit tests cover the state machine, the envelope against the
//! specification's own examples, the account configuration and its
//! hooks, and full exchanges over both transports. The tests/ folder
//! drives the binary as a script, a session and an agent meet it.
//!
//! The development history and living design notes live in the cairn/
//! folder.
//!
//! [JSON-RPC 2.0]: https://www.jsonrpc.org/specification

mod cli;
mod client;
mod jsonrpc20;
mod protocol;
mod server;
mod timer;
mod transport;

use std::process;

use clap::CommandFactory;
use pimalaya_cli::{
    clap::{meet_bare_invocation, parse},
    error::ErrorReport,
    log::Logger,
    printer::StdoutPrinter,
    welcome::{is_wanted, shipped_sample},
};
use pimalaya_config::toml::{ConfigMissingError, TomlConfig};

use crate::cli::{Cli, Command, Session, config::Config, error::ErrorCode};

/// Runs the command line.
///
/// `watch` streams its text events into a pipe too, since a status bar
/// reads them from one.
fn main() {
    let cli = parse::<Cli>();

    Logger::try_init(&cli.log).expect("init logger");
    let mut printer = match cli.cmd {
        Some(Command::Watch(_)) => StdoutPrinter::new(&cli.json).streaming(),
        _ => StdoutPrinter::new(&cli.json),
    };

    let config_paths = cli.config.paths.as_ref();
    let account_name = cli.account.name.as_deref();

    let result = match cli.cmd {
        Some(cmd) => {
            let mut session = Session::new(config_paths, account_name);
            cmd.execute(&mut printer, &mut session)
        }
        None => meet_bare_invocation(&printer, Cli::command(), || {
            let missing = Config::load(config_paths).err();
            missing.filter(|err| err.is::<ConfigMissingError>())
        }),
    };

    if let Err(err) = &result
        && let Some(missing) = err.downcast_ref::<ConfigMissingError>()
        && is_wanted(&printer)
    {
        let sample = shipped_sample("comodoro");
        let welcome = cli::welcome(&missing.path, config_paths, sample.as_deref());
        eprint!("{welcome}");
        process::exit(1);
    }

    ErrorReport::<ErrorCode>::eval_coded(&mut printer, result)
}
