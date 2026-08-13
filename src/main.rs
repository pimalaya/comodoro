//! # Comodoro
//!
//! Binary entry point of the Comodoro CLI. It parses the command-line
//! interface defined in [`comodoro::cli`], wires the logger and the
//! printer from the shared pimalaya-cli toolkit, then hands control to
//! the parsed command.
//!
//! Everything below this file lives in the library: see
//! [`comodoro`](../comodoro/index.html) for the crate architecture.

use clap::Parser;
use comodoro::cli::Cli;
use pimalaya_cli::{error::ErrorReport, log::Logger, printer::StdoutPrinter};

fn main() {
    let cli = Cli::parse();

    Logger::try_init(&cli.log).expect("init logger");
    let mut printer = StdoutPrinter::new(&cli.json);

    let result = cli.execute(&mut printer);

    ErrorReport::eval(&mut printer, result)
}
