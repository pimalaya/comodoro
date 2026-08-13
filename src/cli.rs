//! Command-line interface of Comodoro.
//!
//! [`Cli`] is the clap entry point parsed by main, and [`Command`] is
//! the flat command grammar it dispatches to. Every command taking an
//! account resolves it from the TOML configuration first, then hands it
//! to the client or the server.
//!
//! Nothing works without a configuration, so the two invocations that
//! can find none, a bare `comodoro` and a command needing an account,
//! both offer to generate one rather than failing on it.
//!
//! Everything the CLI needs lives under this module, so the `cli`
//! feature gates one subtree rather than a scattering of items:
//! [`config`] reads the TOML document, [`configure`] generates one,
//! [`transport`] selects the one a command talks over, [`hook`] runs
//! the reactions bound to timer events, and [`client`] and [`server`]
//! hold one module per command.

pub mod client;
pub mod config;
pub mod configure;
pub mod hook;
pub mod server;
pub mod transport;

use alloc::{format, string::String, vec::Vec};

use std::{
    io::{IsTerminal, stdin},
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::{Result, bail};
use clap::{CommandFactory, Parser, Subcommand};
use log::trace;
use pimalaya_cli::{
    clap::{
        args::{AccountFlag, JsonFlag, LogFlags},
        commands::{CompletionCommand, ManualCommand},
        parsers::path_parser,
    },
    footer, long_version,
    printer::Printer,
    prompt,
};
use pimalaya_config::toml::TomlConfig;

use crate::cli::{
    client::{
        get::TimerGetCommand, pause::TimerPauseCommand, resume::TimerResumeCommand,
        set::TimerSetCommand, start::TimerStartCommand, stop::TimerStopCommand,
        watch::TimerWatchCommand,
    },
    config::{CONFIG_SAMPLE_URL, ComodoroAccountConfig, ComodoroConfig},
    configure::ComodoroConfigureCommand,
    server::TimerServerCommand,
};

/// The Comodoro command-line interface.
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author, version, about, long_version = long_version!())]
#[command(long_about = concat!(
    "CLI to manage timers.\n\n",
    "First time here? Run `comodoro` with no command: it offers to generate an ",
    "account from one of the documented presets, which `comodoro configure` does ",
    "again later. Everything else is written by hand.",
))]
#[command(after_help = footer!())]
#[command(propagate_version = true, infer_subcommands = true)]
pub struct Cli {
    /// The command to run.
    ///
    /// Omitted, a bare `comodoro` offers to generate a configuration
    /// when it finds none, since running the binary with no argument is
    /// what a newcomer does first, and shows this help otherwise.
    #[command(subcommand)]
    pub cmd: Option<Command>,
    /// The configuration file(s) to read the account from.
    #[command(flatten)]
    pub config: ComodoroConfigPathsArg,
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
    /// Configure an account interactively.
    #[command(visible_alias = "wizard")]
    Configure(ComodoroConfigureCommand),
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

impl Cli {
    /// Runs the parsed command, or meets a bare invocation.
    ///
    /// With no command there is nothing to run, so this is where a
    /// newcomer lands: a missing configuration raises the offer, and an
    /// existing one gets the help, which is also what a script or a
    /// JSON caller gets since neither can answer a prompt. A file that
    /// exists but fails to parse counts as a configuration, so the
    /// offer never proposes to write over a broken one: the parse error
    /// surfaces when a real command reads it.
    pub fn execute(self, printer: &mut impl Printer) -> Result<()> {
        let config_paths = self.config.paths.as_ref();
        let account_name = self.account.name.as_deref();

        let Some(cmd) = self.cmd else {
            let configured = ComodoroConfig::from_paths_or_default(config_paths)
                .ok()
                .flatten()
                .is_some();

            if configured || printer.is_json() || !stdin().is_terminal() {
                Cli::command().print_help()?;
                return Ok(());
            }

            let path = ComodoroConfig::target_path(config_paths)?;

            return offer_configuration(printer, config_paths, &path);
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
            Self::Configure(cmd) => cmd.execute(printer, config_paths),

            Self::Server(cmd) => {
                let mut account = take_account(printer, config_paths, account_name)?;
                cmd.execute(&mut account)
            }
            Self::Start(cmd) => {
                let account = take_account(printer, config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Get(cmd) => {
                let account = take_account(printer, config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Watch(cmd) => {
                let account = take_account(printer, config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Set(cmd) => {
                let account = take_account(printer, config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Pause(cmd) => {
                let account = take_account(printer, config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Resume(cmd) => {
                let account = take_account(printer, config_paths, account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Stop(cmd) => {
                let account = take_account(printer, config_paths, account_name)?;
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
    /// one(s). Multiple paths can also be given at once, delimited by
    /// `:` like `$PATH` in a POSIX shell.
    #[arg(long = "config", short = 'c', global = true, env = "COMODORO_CONFIG")]
    #[arg(name = "config_paths", value_name = "PATH", value_parser = path_parser, value_delimiter = ':')]
    pub paths: Vec<PathBuf>,
}

/// Welcomes, then offers to generate a first configuration, falling
/// back to the help when the offer is declined.
///
/// Raised from the two places nothing can happen without a
/// configuration: a bare invocation, and a command that needs an
/// account. Only the caller knows what to do afterwards, so this one
/// just runs the offer.
fn offer_configuration(
    printer: &mut impl Printer,
    config_paths: &[PathBuf],
    path: &Path,
) -> Result<()> {
    configure::print_welcome(path);

    if prompt::bool("Create a configuration with a default account?", true)? {
        return ComodoroConfigureCommand.execute(printer, config_paths);
    }

    Cli::command().print_help()?;

    Ok(())
}

/// Loads the configuration and takes the account the command runs
/// against, the one `-a` names or the one marked as default.
///
/// A missing configuration is met with the wizard rather than with an
/// error: the welcome frames what Comodoro is, then the offer either
/// generates a first account or falls back to the help, and the process
/// stops there either way. The two other failures name what is missing
/// and how to pick an account.
fn take_account(
    printer: &mut impl Printer,
    config_paths: &[PathBuf],
    account_name: Option<&str>,
) -> Result<ComodoroAccountConfig> {
    let Some(mut config) = ComodoroConfig::from_paths_or_default(config_paths)? else {
        // NOTE: the target path is where `-c` pointed, or the default
        // location when it named none, so a mistyped path shows up as
        // itself rather than as a generic first run.
        let path = ComodoroConfig::target_path(config_paths)?;

        // NOTE: nobody is there to answer a prompt in a script or a
        // cron job, and a JSON consumer wants a failure it can read, so
        // both get the pointer rather than the offer.
        if printer.is_json() || !stdin().is_terminal() {
            bail!(
                "No configuration found at {}, run `comodoro configure` to generate one or write it by hand: {CONFIG_SAMPLE_URL}",
                path.display(),
            );
        }

        offer_configuration(printer, config_paths, &path)?;

        // NOTE: the command that raised the offer is not resumed: the
        // account it wanted may not be the one just generated, and no
        // server is running behind it yet.
        exit(0);
    };

    // NOTE: an empty name and `default` both mean the default account,
    // which is the next block's business.
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

    Ok(account)
}
