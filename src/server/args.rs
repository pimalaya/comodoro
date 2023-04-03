//! Email server CLI module.
//!
//! This module contains the command matcher, the subcommands and the
//! arguments related to the email server domain.

use anyhow::Result;
use clap::{ArgMatches, Command};
use log::debug;

const CMD_START: &str = "start";
const CMD_STOP: &str = "stop";

pub(crate) const CMD_SERVER: &str = "server";

/// Represents the server commands.
#[derive(Debug, PartialEq, Eq)]
pub enum Cmd {
    Start,
    Stop,
}

/// Represents the server command matcher.
pub fn matches<'a>(m: &'a ArgMatches) -> Result<Option<Cmd>> {
    let cmd = if let Some(_) = m.subcommand_matches(CMD_START) {
        debug!("start server command matched");
        Some(Cmd::Start)
    } else if let Some(_) = m.subcommand_matches(CMD_STOP) {
        debug!("stop server command matched");
        Some(Cmd::Stop)
    } else {
        None
    };

    Ok(cmd)
}

/// Represents the server subcommands.
pub fn subcmd<'a>() -> Command {
    Command::new(CMD_SERVER)
        .about("Server commands")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new(CMD_START).about("Start the timer server"))
        .subcommand(Command::new(CMD_STOP).about("Stop the timer server"))
}
