//! # Comodoro
//!
//! Binary entry point of the Comodoro CLI. It parses the command-line
//! interface defined in [`comodoro::cli`], wires the logger and the
//! printer from the shared pimalaya-cli toolkit, then hands control to
//! the parsed command, or meets a bare invocation.
//!
//! A person missing a configuration meets the welcome on stderr, a
//! script or a JSON caller the error.
//!
//! Everything below this file lives in the library: see
//! [`comodoro`](../comodoro/index.html) for the crate architecture.

use std::process;

use clap::Parser;
use comodoro::cli::{self, Cli};
use pimalaya_cli::{
    error::ErrorReport,
    log::Logger,
    printer::StdoutPrinter,
    welcome::{is_wanted, shipped_sample},
};
use pimalaya_config::toml::ConfigMissingError;

fn main() {
    let cli = Cli::parse();

    Logger::try_init(&cli.log).expect("init logger");
    let mut printer = StdoutPrinter::new(&cli.json);

    let config_paths = cli.config.paths.as_ref();
    let account_name = cli.account.name.as_deref();

    let result = match cli.cmd {
        Some(cmd) => cmd.execute(&mut printer, config_paths, account_name),
        None => cli::meet_bare_invocation(config_paths),
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

    ErrorReport::eval(&mut printer, result)
}
