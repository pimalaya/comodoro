//! Command-line interface of Comodoro.
//!
//! [`Cli`] is the clap entry point parsed by main, and [`Command`] is
//! the flat command grammar it dispatches to. Every command taking an
//! account resolves it from the TOML configuration first, then hands it
//! to the client or the server.
//!
//! Everything the CLI needs lives under this module, so the `cli`
//! feature gates one subtree rather than a scattering of items:
//! [`config`] reads the TOML document, [`hook`] runs the reactions bound
//! to timer events, and [`client`] and [`server`] hold one module per
//! command.

pub mod client;
pub mod config;
pub mod hook;
pub mod server;

use alloc::{format, vec::Vec};
use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::{CommandFactory, Parser, Subcommand};
use log::trace;
use pimalaya_cli::{
    clap::{
        args::{AccountFlag, JsonFlag, LogFlags},
        commands::{CompletionCommand, ManualCommand},
        parsers::path_parser,
    },
    long_version,
    printer::Printer,
};
use pimalaya_config::toml::TomlConfig;

use crate::cli::{
    client::{
        get::TimerGetCommand, pause::TimerPauseCommand, resume::TimerResumeCommand,
        set::TimerSetCommand, start::TimerStartCommand, stop::TimerStopCommand,
        watch::TimerWatchCommand,
    },
    config::{ComodoroAccountConfig, ComodoroConfig},
    server::TimerServerCommand,
};

/// The Comodoro command-line interface.
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author, version, about)]
#[command(long_version = long_version!())]
#[command(propagate_version = true, infer_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
    #[command(flatten)]
    pub config: ComodoroConfigPathsArg,
    #[command(flatten)]
    pub account: AccountFlag,
    #[command(flatten)]
    pub json: JsonFlag,
    #[command(flatten)]
    pub log: LogFlags,
}

/// The commands Comodoro exposes.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage the timer servers.
    #[command(arg_required_else_help = true)]
    #[command(subcommand)]
    #[command(visible_alias = "srv")]
    Server(TimerServerCommand),
    /// Start the timer.
    Start(TimerStartCommand),
    /// Get the timer.
    Get(TimerGetCommand),
    /// Watch the timer.
    Watch(TimerWatchCommand),
    /// Pause the timer.
    Pause(TimerPauseCommand),
    /// Resume the timer.
    Resume(TimerResumeCommand),
    /// Stop the timer.
    Stop(TimerStopCommand),
    /// Set the remaining duration of the current cycle.
    Set(TimerSetCommand),
    /// Generate the man pages.
    #[command(arg_required_else_help = true, alias = "mans")]
    Manuals(ManualCommand),
    /// Generate the shell completion scripts.
    #[command(arg_required_else_help = true, alias = "cpl")]
    Completions(CompletionCommand),
}

impl Command {
    /// Resolves the account when the command needs one, then runs it.
    pub fn execute(
        self,
        printer: &mut impl Printer,
        config_paths: &[PathBuf],
        account_name: Option<&str>,
    ) -> Result<()> {
        trace!("config paths: {config_paths:?}");
        trace!("account name: {account_name:?}");

        match self {
            Self::Server(cmd) => {
                let mut account = take_account(config_paths, account_name)?;
                cmd.execute(&mut account)
            }
            Self::Start(cmd) => {
                let account = take_account(config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Get(cmd) => {
                let account = take_account(config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Watch(cmd) => {
                let account = take_account(config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Set(cmd) => {
                let account = take_account(config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Pause(cmd) => {
                let account = take_account(config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Resume(cmd) => {
                let account = take_account(config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Stop(cmd) => {
                let account = take_account(config_paths, account_name)?;
                cmd.execute(printer, &account)
            }

            Self::Manuals(cmd) => cmd.execute(printer, Cli::command()),
            Self::Completions(cmd) => cmd.execute(printer, Cli::command()),
        }
    }
}

/// Path(s) to the TOML configuration file(s).
#[derive(Debug, Default, Parser)]
pub struct ComodoroConfigPathsArg {
    /// Override the default configuration file path.
    ///
    /// The given paths are shell-expanded then canonicalized (if
    /// applicable). Other paths are merged with the first one, which
    /// allows you to separate your public config from your private
    /// one(s).
    #[arg(long = "config", short = 'c', global = true)]
    #[arg(name = "config_paths", value_name = "PATH", value_parser = path_parser)]
    pub paths: Vec<PathBuf>,
}

fn take_account(
    config_paths: &[PathBuf],
    account_name: Option<&str>,
) -> Result<ComodoroAccountConfig> {
    let Some(mut config) = ComodoroConfig::from_paths_or_default(config_paths)? else {
        bail!("Config file not found");
    };

    let Some((_, account)) = config.take_account(account_name)? else {
        bail!("Account not found");
    };

    Ok(account)
}
