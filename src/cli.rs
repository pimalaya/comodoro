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
//! [`config`] holds the TOML document, [`account`] the resolved view a
//! command runs against, [`configure`] the wizard writing a document,
//! [`transport`] the selection of the one a command talks over,
//! [`hook`] the reactions bound to timer events, and [`client`] and
//! [`server`] one module per command.

pub mod account;
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
    account::Account,
    client::{
        get::TimerGetCommand, pause::TimerPauseCommand, resume::TimerResumeCommand,
        set::TimerSetCommand, start::TimerStartCommand, stop::TimerStopCommand,
        watch::TimerWatchCommand,
    },
    config::{CONFIG_SAMPLE_URL, Config},
    configure::ConfigureCommand,
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
    /// Configure an account interactively.
    #[command(visible_alias = "wizard")]
    Configure(ConfigureCommand),
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
            let configured = Config::from_paths_or_default(config_paths)
                .ok()
                .flatten()
                .is_some();

            if !configured && !printer.is_json() && stdin().is_terminal() {
                let path = Config::target_path(config_paths)?;

                // NOTE: a bare invocation has nothing to run after the
                // offer, so a declined one falls back to the help. The
                // wizard already says what to run next when it ran.
                if offer_configuration(printer, config_paths, &path)? {
                    return Ok(());
                }
            }

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
pub struct ConfigPathsArg {
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

/// Welcomes, then offers to generate a first configuration. Returns
/// whether the wizard ran.
///
/// Raised from the two places nothing can happen without a
/// configuration: a bare invocation, and a command that needs an
/// account. It is a hook rather than a gate, so declining it decides
/// nothing: what happens next is the caller's business, and for a
/// command that is simply carrying on.
fn offer_configuration(
    printer: &mut impl Printer,
    config_paths: &[PathBuf],
    path: &Path,
) -> Result<bool> {
    configure::print_welcome(path);

    if !prompt::bool("Create a configuration with a default account?", true)? {
        return Ok(false);
    }

    ConfigureCommand.execute(printer, config_paths)?;

    Ok(true)
}

/// Loads the configuration, takes the account the command runs against,
/// the one `-a` names or the one marked as default, and resolves it into
/// the [`Account`] the command consumes.
///
/// A missing configuration is met with the wizard rather than with an
/// error: the welcome frames what Comodoro is and offers to generate an
/// account, then the command carries on either way. Accepting is what
/// gives it a chance to work; declining leaves it to fail on the
/// configuration it still has not got. The two other failures name what
/// is missing and how to pick an account.
fn take_account(
    printer: &mut impl Printer,
    config_paths: &[PathBuf],
    account_name: Option<&str>,
) -> Result<Account> {
    let mut config = match Config::from_paths_or_default(config_paths)? {
        Some(config) => config,
        None => {
            // NOTE: the target path is where `-c` pointed, or the
            // default location when it named none, so a mistyped path
            // shows up as itself rather than as a generic first run.
            let path = Config::target_path(config_paths)?;

            // NOTE: nobody is there to answer a prompt in a script or a
            // cron job, and a JSON consumer wants a failure it can
            // read, so both skip the offer and fail below.
            if !printer.is_json() && stdin().is_terminal() {
                offer_configuration(printer, config_paths, &path)?;
            }

            // NOTE: the wizard also prints the account instead of
            // writing it, so having run it proves nothing: the
            // configuration is looked up again, and the command fails
            // the ordinary way when nothing landed.
            match Config::from_paths_or_default(config_paths)? {
                Some(config) => config,
                None => bail!(
                    "No configuration found at {}, run `comodoro configure` to generate one or write it by hand: {CONFIG_SAMPLE_URL}",
                    path.display(),
                ),
            }
        }
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

    Ok(account.into())
}
