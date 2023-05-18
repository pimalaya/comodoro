//! Email server CLI module.
//!
//! This module contains the command matcher, the subcommands and the
//! arguments related to the email server domain.

use anyhow::Result;
use clap::{value_parser, Arg, ArgMatches, Command};
use log::debug;

use crate::{preset, Protocol};

const ARG_PROTOCOL: &str = "protocol";
const CMD_START: &str = "start";
const CMD_GET: &str = "get";
const CMD_PAUSE: &str = "pause";
const CMD_RESUME: &str = "resume";
const CMD_STOP: &str = "stop";

/// Represents the server commands.
#[derive(Debug, PartialEq, Eq)]
pub enum Cmd<'a> {
    Start(&'a str, &'a Protocol),
    Get(&'a str, &'a Protocol),
    Pause(&'a str, &'a Protocol),
    Resume(&'a str, &'a Protocol),
    Stop(&'a str, &'a Protocol),
}

/// Represents the server command matcher.
pub fn matches<'a>(m: &'a ArgMatches) -> Result<Option<Cmd<'a>>> {
    let cmd = if let Some(ref m) = m.subcommand_matches(CMD_START) {
        debug!("start timer command matched");
        let preset = preset::args::parse_arg(m);
        let protocol = parse_protocol(m);
        Some(Cmd::Start(preset, protocol))
    } else if let Some(ref m) = m.subcommand_matches(CMD_GET) {
        debug!("get timer command matched");
        let preset = preset::args::parse_arg(m);
        let protocol = parse_protocol(m);
        Some(Cmd::Get(preset, protocol))
    } else if let Some(ref m) = m.subcommand_matches(CMD_PAUSE) {
        debug!("pause timer command matched");
        let preset = preset::args::parse_arg(m);
        let protocol = parse_protocol(m);
        Some(Cmd::Pause(preset, protocol))
    } else if let Some(ref m) = m.subcommand_matches(CMD_RESUME) {
        debug!("resume timer command matched");
        let preset = preset::args::parse_arg(m);
        let protocol = parse_protocol(m);
        Some(Cmd::Resume(preset, protocol))
    } else if let Some(ref m) = m.subcommand_matches(CMD_STOP) {
        debug!("stop timer command matched");
        let preset = preset::args::parse_arg(m);
        let protocol = parse_protocol(m);
        Some(Cmd::Stop(preset, protocol))
    } else {
        None
    };

    Ok(cmd)
}

/// Represents the client protocol argument.
pub fn protocol() -> Arg {
    Arg::new(ARG_PROTOCOL)
        .help("Define protocol the client should use to send requests")
        .required(true)
        .value_parser(value_parser!(Protocol))
}

/// Represents the client protocol argument parser.
pub fn parse_protocol(m: &ArgMatches) -> &Protocol {
    m.get_one::<Protocol>(ARG_PROTOCOL).unwrap()
}

/// Represents the client subcommands.
pub fn subcmds() -> [Command; 5] {
    [
        Command::new(CMD_START)
            .about("Start the timer")
            .arg(preset::args::arg())
            .arg(protocol()),
        Command::new(CMD_GET)
            .about("Get the current timer value")
            .arg(preset::args::arg())
            .arg(protocol()),
        Command::new(CMD_PAUSE)
            .about("Pause the timer")
            .arg(preset::args::arg())
            .arg(protocol()),
        Command::new(CMD_RESUME)
            .about("Resume the timer")
            .arg(preset::args::arg())
            .arg(protocol()),
        Command::new(CMD_STOP)
            .about("Stop the timer")
            .arg(preset::args::arg())
            .arg(protocol()),
    ]
}
