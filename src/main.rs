//! # Comodoro
//!
//! Binary entry point of the Comodoro CLI. It parses the command-line
//! interface defined in [`comodoro::cli`], wires the logger and the
//! printer from the shared pimalaya-cli toolkit, then hands control to
//! the parsed command, or meets a bare invocation through pimalaya-cli,
//! a refused command line under `--json`.
//!
//! A person missing a configuration meets the welcome on stderr, a
//! script or a JSON caller the error. A failure prints the failure object
//! and exits 1. A command line the grammar refuses exits 2, before any
//! configuration is read, its failure object coded `usage-invalid` on
//! stdout under `--json`.
//!
//! Everything below this file lives in the library: see
//! [`comodoro`](../comodoro/index.html) for the crate architecture.

use std::process;

use clap::CommandFactory;
use comodoro::cli::{self, Cli, Command, Session, config::Config, error::ErrorCode};
use pimalaya_cli::{
    clap::{meet_bare_invocation, parse},
    error::ErrorReport,
    log::Logger,
    printer::StdoutPrinter,
    welcome::{is_wanted, shipped_sample},
};
use pimalaya_config::toml::{ConfigMissingError, TomlConfig};

fn main() {
    let cli = parse::<Cli>();

    Logger::try_init(&cli.log).expect("init logger");
    // NOTE: `watch` streams its text events into a pipe too, since a
    // status bar reads them from one.
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
