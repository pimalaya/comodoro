//! Email server CLI module.
//!
//! This module contains the command matcher, the subcommands and the
//! arguments related to the email server domain.

use anyhow::Result;
use clap::{ArgMatches, Command};
use log::debug;

const CMD_START: &str = "start";
const CMD_GET: &str = "get";
const CMD_PAUSE: &str = "pause";
const CMD_RESUME: &str = "resume";
const CMD_STOP: &str = "stop";

/// Represents the server commands.
#[derive(Debug, PartialEq, Eq)]
pub enum Cmd {
    Start,
    Get,
    Pause,
    Resume,
    Stop,
}

/// Represents the server command matcher.
pub fn matches<'a>(m: &'a ArgMatches) -> Result<Option<Cmd>> {
    let cmd = if let Some(_) = m.subcommand_matches(CMD_START) {
        debug!("start timer command matched");
        Some(Cmd::Start)
    } else if let Some(_) = m.subcommand_matches(CMD_GET) {
        debug!("get timer command matched");
        Some(Cmd::Get)
    } else if let Some(_) = m.subcommand_matches(CMD_PAUSE) {
        debug!("pause timer command matched");
        Some(Cmd::Pause)
    } else if let Some(_) = m.subcommand_matches(CMD_RESUME) {
        debug!("resume timer command matched");
        Some(Cmd::Resume)
    } else if let Some(_) = m.subcommand_matches(CMD_STOP) {
        debug!("stop timer command matched");
        Some(Cmd::Stop)
    } else {
        None
    };

    Ok(cmd)
}

/// Represents the server subcommands.
pub fn subcmds<'a>() -> Vec<Command> {
    vec![
        Command::new(CMD_START).about("Start the timer"),
        Command::new(CMD_GET).about("Get the current timer value"),
        Command::new(CMD_PAUSE).about("Pause the timer"),
        Command::new(CMD_RESUME).about("Resume the timer"),
        Command::new(CMD_STOP).about("Stop the timer"),
    ]
}
