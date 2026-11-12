//! # CLI
//!
//! Command-line interface of Comodoro. [`Cli`] is the clap entry point
//! parsed by main, and [`Command`] the flat grammar it dispatches to.
//!
//! Every command taking an account resolves it from the TOML
//! configuration first, then hands it to the client or the server.
//!
//! Comodoro writes no configuration: a command finding none fails naming
//! the path it looked at and pointing at config.sample.toml, and a bare
//! `comodoro` shows the help.
//!
//! Everything the CLI needs lives here, so the `cli` feature gates one
//! subtree: [`config`] holds the TOML document, [`account`] the resolved
//! view a command runs against, [`transport`] the selection of the one a
//! command talks over.
//!
//! [`hook`] holds the reactions bound to timer events, [`json_schema`]
//! the schemas of what the commands print, and [`client`] and [`server`]
//! one module per command.

pub mod account;
pub mod client;
pub mod config;
pub mod hook;
pub mod json_schema;
pub mod server;
pub mod transport;

use alloc::{format, string::String, vec::Vec};

use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::{CommandFactory, Parser, Subcommand};
use log::trace;
use pimalaya_cli::{
    clap::{
        args::{AccountFlag, JsonFlag, LogFlags},
        commands::{CompletionCommand, JsonSchemaCommand, ManualCommand},
        parsers::path_parser,
    },
    footer, long_version,
    printer::Printer,
};
use pimalaya_config::toml::TomlConfig;

use crate::cli::{
    account::Account,
    client::{
        get::TimerGetCommand, pause::TimerPauseCommand, resume::TimerResumeCommand,
        set::TimerSetCommand, start::TimerStartCommand, stop::TimerStopCommand,
        watch::TimerWatchCommand,
    },
    config::{CONFIG_SAMPLE_URL, Config},
    server::TimerServerCommand,
};

/// The Comodoro command-line interface.
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author, version, about, long_version = long_version!())]
#[command(after_help = footer!())]
#[command(propagate_version = true, infer_subcommands = true)]
pub struct Cli {
    /// The command to run.
    ///
    /// Omitted, a bare `comodoro` shows this help.
    #[command(subcommand)]
    pub cmd: Option<Command>,
    /// The configuration file(s) to read the account from.
    #[command(flatten)]
    pub config: ConfigPathsArg,
    /// The account the command applies to.
    #[command(flatten)]
    pub account: AccountFlag,
    /// Whether the output is rendered as JSON.
    #[command(flatten)]
    pub json: JsonFlag,
    /// How much the command logs, and where.
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
    /// Generate the shell completion scripts.
    #[command(aliases = ["completions", "cpl"])]
    Completion(CompletionCommand),
    /// Generate the man pages.
    #[command(aliases = ["manuals", "mans"])]
    Manual(ManualCommand),
    /// Generate the JSON Schemas of the command outputs.
    #[command(alias = "json-schemas")]
    JsonSchema(JsonSchemaCommand),
}

impl Cli {
    /// Runs the parsed command, or shows the help when there is none.
    pub fn execute(self, printer: &mut impl Printer) -> Result<()> {
        let config_paths = self.config.paths.as_ref();
        let account_name = self.account.name.as_deref();

        let Some(cmd) = self.cmd else {
            Cli::command().print_help()?;
            return Ok(());
        };

        cmd.execute(printer, config_paths, account_name)
    }
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

            Self::Completion(cmd) => cmd.execute(printer, Cli::command()),
            Self::Manual(cmd) => cmd.execute(printer, Cli::command()),
            Self::JsonSchema(cmd) => cmd.execute(printer, json_schema::registry()),
        }
    }
}

/// Path(s) to the TOML configuration file(s).
#[derive(Debug, Default, Parser)]
pub struct ConfigPathsArg {
    /// Override the default configuration file path.
    ///
    /// The given paths are shell-expanded then canonicalized when
    /// applicable. Several can be given at once, delimited by `:` like
    /// `$PATH`: the first one is the base and the rest are merged on top,
    /// which keeps a public configuration apart from the private ones.
    #[arg(long = "config", short = 'c', global = true, env = "COMODORO_CONFIG")]
    #[arg(name = "config_paths", value_name = "PATH", value_parser = path_parser, value_delimiter = ':')]
    pub paths: Vec<PathBuf>,
}

/// Resolves the account the command runs against, the one `-a` names or
/// the one marked as default, into the [`Account`] it consumes.
///
/// Each failure names what is missing: the path a missing configuration
/// was looked for at, the accounts a configuration does hold, and the two
/// ways to pick a default.
fn take_account(config_paths: &[PathBuf], account_name: Option<&str>) -> Result<Account> {
    let Some(mut config) = Config::from_paths_or_default(config_paths)? else {
        let path = Config::target_path(config_paths)?;

        bail!(
            "No configuration found at {}, write one by hand from the sample: {CONFIG_SAMPLE_URL}",
            path.display(),
        );
    };

    let named = account_name.filter(|name| !name.is_empty() && *name != "default");

    if let Some(name) = named.filter(|name| !config.accounts.contains_key(*name)) {
        let mut names: Vec<&str> = config.accounts.keys().map(String::as_str).collect();
        names.sort_unstable();

        bail!(
            "Account `{name}` not found, the configuration holds: {}",
            names.join(", "),
        );
    }

    let Some((_, account)) = config.take_account(account_name)? else {
        bail!(
            "No default account found, name one with `-a <NAME>` or mark one with `default = true`"
        );
    };

    Ok(account.into())
}
