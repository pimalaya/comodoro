//! # Repl command
//!
//! The `repl` subcommand, a session over stdin and stdout running the
//! CLI's own commands against one account, resolved once for the whole
//! session.
//!
//! pimalaya-cli's [`Repl`] runs the loop: each line split as a shell
//! splits it, parsed with the one-shot grammar plus `--json` and run over
//! the [`Session`] holding the account, every answer on stdout, the
//! session flags refused by name, a nested `repl` or `mcp` line refused
//! coded `usage-invalid` before it reaches this crate, and a `--version`
//! line answered with the version [`Command`] carries. Each line connects
//! to the server anew, so a server restarted between two lines is met by
//! the next as a one-shot run meets it.
//!
//! pimalaya-cli installs no Ctrl-C handler, so the commands running until
//! interrupted, `watch` and `server start`, are refused: interrupting
//! them would end the session with them.

use anyhow::{Error, Result};
use clap::Parser;
use pimalaya_cli::{
    error::{CodedError, CommonCode},
    printer::Printer,
    repl::Repl,
};

use crate::cli::{Command, Session, error::ErrorCode};

/// Start a session running commands over one account.
///
/// Each line is a command without the binary name (`get`, `set 300`),
/// run against the account the session resolved once; `--json` on a
/// line switches that line alone to JSON. `watch` and `server start` run
/// until interrupted, so they are refused here. `quit`, `exit`, or EOF
/// ends the session.
#[derive(Debug, Parser)]
pub struct ReplCommand;

impl ReplCommand {
    /// Runs the read-eval-print loop until EOF or a quit command, keeping
    /// the session's account across lines, each line printing as
    /// `printer` does unless it asks for JSON.
    pub fn execute(self, printer: &impl Printer, session: &mut Session) -> Result<()> {
        // NOTE: taken up front, so a missing configuration or account
        // fails the session rather than its first line.
        session.account()?;

        let repl = Repl::new("comodoro", printer.is_json());

        repl.run::<Command, ErrorCode>(|printer, cmd| match cmd {
            Command::Watch(_) => Err(until_interrupted("watch")),
            Command::Server(_) => Err(until_interrupted("server start")),
            cmd => cmd.execute(printer, session),
        })
    }
}

/// The refusal of a command running until interrupted, coded
/// `usage-invalid` as a nested session is, nothing having run.
fn until_interrupted(cmd: &str) -> Error {
    let message = format!("`{cmd}` runs until interrupted, run it outside a session");
    CodedError::new(CommonCode::UsageInvalid, message).into()
}
