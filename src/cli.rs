//! # CLI
//!
//! Command-line interface of Comodoro. [`Cli`] is the clap entry point
//! parsed by main, and [`Command`] the flat grammar it dispatches to.
//!
//! Every command taking an account takes it through the [`Session`],
//! which resolves it from the TOML configuration on first use and keeps
//! it for the lines of a REPL, then hands it to the client or the server.
//! The loading and the selection are pimalaya-config's, so a missing
//! configuration, an unknown account and several defaults fail the way
//! they fail across the family.
//!
//! Comodoro writes no configuration. A bare `comodoro` shows the help
//! when one exists, and a person running Comodoro without one meets the
//! welcome ([`welcome`], the shared one in Comodoro's words), which
//! copies config.sample.toml from the release or the package, or
//! downloads it.
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
pub mod error;
pub mod hook;
pub mod json_schema;
pub mod mcp;
pub mod repl;
pub mod server;
pub mod transport;

use alloc::format;

use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use log::trace;
use pimalaya_cli::{
    clap::{
        args::{AccountFlag, ConfigFlags, JsonFlag, LogFlags},
        commands::{
            completion::CompletionCommand, json_schema::JsonSchemaCommand, manual::ManualCommand,
        },
    },
    error::with_code,
    footer, long_version,
    mcp::exit_on_failure,
    printer::Printer,
    welcome::{Welcome, is_wanted},
};
use pimalaya_config::toml::{ConfigMissingError, TomlConfig};

use crate::cli::{
    account::Account,
    client::{
        get::TimerGetCommand, pause::TimerPauseCommand, resume::TimerResumeCommand,
        set::TimerSetCommand, start::TimerStartCommand, stop::TimerStopCommand,
        watch::TimerWatchCommand,
    },
    config::Config,
    error::ErrorCode,
    mcp::McpCommand,
    repl::ReplCommand,
    server::TimerServerCommand,
};

/// The sample configuration as a raw file, for `curl` to fetch.
const SAMPLE_RAW_URL: &str =
    "https://raw.githubusercontent.com/pimalaya/comodoro/master/config.sample.toml";

/// The Comodoro command-line interface.
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author, version, about, long_version = long_version!())]
#[command(after_help = footer!())]
#[command(propagate_version = true, infer_subcommands = true)]
pub struct Cli {
    /// The command to run, bare `comodoro` showing the help.
    #[command(subcommand)]
    pub cmd: Option<Command>,
    /// The configuration file(s) to read the account from.
    #[command(flatten)]
    pub config: ConfigFlags,
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
///
/// The version it carries is what a `-V` or `--version` line of a
/// session prints, as the command line does.
#[derive(Debug, Subcommand)]
#[command(version, long_version = long_version!())]
pub enum Command {
    /// Manage the timer servers.
    #[command(arg_required_else_help = true)]
    #[command(subcommand)]
    #[command(alias = "srv")]
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
    /// Start a session running commands over one account.
    Repl(ReplCommand),
    /// Start a Model Context Protocol server over stdio.
    Mcp(McpCommand),
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

impl Command {
    /// Runs the command over `session`, which takes the account on first
    /// use, a client failure carrying its code.
    pub fn execute(self, printer: &mut impl Printer, session: &mut Session) -> Result<()> {
        trace!("config paths: {:?}", session.config_paths);
        trace!("account name: {:?}", session.account_name);

        let result = match self {
            Self::Server(cmd) => cmd.execute(session.account()?),
            Self::Start(cmd) => cmd.execute(printer, session.account()?),
            Self::Get(cmd) => cmd.execute(printer, session.account()?),
            Self::Watch(cmd) => cmd.execute(printer, session.account()?),
            Self::Set(cmd) => cmd.execute(printer, session.account()?),
            Self::Pause(cmd) => cmd.execute(printer, session.account()?),
            Self::Resume(cmd) => cmd.execute(printer, session.account()?),
            Self::Stop(cmd) => cmd.execute(printer, session.account()?),
            Self::Repl(cmd) => cmd.execute(&*printer, session),
            Self::Mcp(cmd) => {
                // NOTE: stdout carries protocol messages only, so a
                // failure goes to stderr, a person missing a
                // configuration aside.
                if let Err(err) = session.account() {
                    if err.is::<ConfigMissingError>() && is_wanted(&*printer) {
                        return Err(err);
                    }

                    exit_on_failure::<ErrorCode, ()>(Err(err));
                }

                exit_on_failure::<ErrorCode, _>(cmd.execute(session));
                Ok(())
            }
            Self::Completion(cmd) => cmd.execute(printer, Cli::command()),
            Self::Manual(cmd) => cmd.execute(printer, Cli::command()),
            Self::JsonSchema(cmd) => cmd.execute(printer, json_schema::registry()),
        };

        result.map_err(|err| with_code(err, error::code))
    }
}

/// What the commands of one run share: the configuration paths, the
/// account `-a` names, and that account once taken, which a REPL session
/// keeps across its lines.
pub struct Session<'a> {
    /// The configuration paths, `-c` or the default ones.
    pub config_paths: &'a [PathBuf],
    /// The account `-a` names, the default one when `None`.
    pub account_name: Option<&'a str>,
    /// The account, taken on first use.
    pub account: Option<Account>,
}

impl<'a> Session<'a> {
    /// A session over the configuration at `config_paths`, its account
    /// not taken yet.
    pub fn new(config_paths: &'a [PathBuf], account_name: Option<&'a str>) -> Self {
        Self {
            config_paths,
            account_name,
            account: None,
        }
    }

    /// The session's account, taken from the configuration on first use.
    pub fn account(&mut self) -> Result<&mut Account> {
        let account = match self.account.take() {
            Some(account) => account,
            None => take_account(self.config_paths, self.account_name)?,
        };

        Ok(self.account.insert(account))
    }
}

/// Meets a bare `comodoro`: the help when a configuration exists, the
/// missing configuration otherwise, which a person meets as the welcome.
pub fn meet_bare_invocation(config_paths: &[PathBuf]) -> Result<()> {
    // NOTE: a broken file counts as a configuration, its error left to
    // the first command reading it.
    if let Err(err) = Config::load(config_paths)
        && err.is::<ConfigMissingError>()
    {
        return Err(err);
    }

    Cli::command().print_help()?;
    Ok(())
}

/// Takes the account `-a` names, or the one marked default without `-a`,
/// out of the configuration at `config_paths`, resolved into the
/// [`Account`] a command runs against. An account that cannot be taken
/// fails coded, `default` being a name like any other.
pub fn take_account(config_paths: &[PathBuf], account_name: Option<&str>) -> Result<Account> {
    let (_, account) = Config::load(config_paths)?
        .take_account(account_name)
        .map_err(|err| with_code(err.into(), error::code))?;
    Account::try_from(account)
}

/// The welcome a person meets when no configuration exists at `path`:
/// the commands creating it from `sample`, the shipped sample
/// configuration, or downloading it without one, then `server start`.
pub fn welcome<'a>(
    path: &'a Path,
    config_paths: &'a [PathBuf],
    sample: Option<&'a Path>,
) -> Welcome<'a> {
    Welcome {
        bin: "comodoro",
        intro: "Welcome to Comodoro, the CLI to manage timers.",
        path,
        config_paths,
        sample,
        sample_url: SAMPLE_RAW_URL,
        next: "Edit it and uncomment the options you need, then start the timer server:",
        command: "server start",
    }
}

#[cfg(test)]
mod tests {
    use alloc::{
        format,
        string::{String, ToString},
    };

    use std::{fs, path::PathBuf};

    use clap::Parser;
    use pimalaya_cli::error::code_of;
    use pimalaya_config::toml::ConfigMissingError;
    use tempfile::TempDir;

    use crate::cli::{Cli, error::ErrorCode, take_account};

    /// A directory holding a configuration file made of `toml`, and the
    /// path of that file.
    fn config(toml: &str) -> (TempDir, PathBuf) {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("config.toml");
        fs::write(&path, toml).unwrap();
        (dir, path)
    }

    fn account(name: &str, default: bool) -> String {
        format!(
            "[accounts.{name}]\ndefault = {default}\ncycles = [{{ name = \"Work\", duration = 1500 }}]\n"
        )
    }

    #[test]
    fn a_missing_configuration_is_typed_for_the_welcome() {
        let paths = [PathBuf::from("/nonexistent/comodoro/config.toml")];
        let err = take_account(&paths, None).unwrap_err();
        let missing = err.downcast_ref::<ConfigMissingError>().unwrap();
        assert_eq!(missing.path, paths[0]);
    }

    #[test]
    fn an_unknown_account_lists_the_held_ones() {
        let (_dir, path) = config(&(account("b", false) + &account("a", true)));
        let err = take_account(&[path], Some("c")).unwrap_err();
        let err = err.to_string();

        assert!(err.contains("Account `c` not found"), "{err}");
        assert!(err.contains("a, b"), "{err}");
    }

    /// Comodoro reports which accounts are marked default, so several are
    /// refused rather than one taken at random.
    #[test]
    fn several_default_accounts_are_refused_naming_them() {
        let (_dir, path) = config(&(account("b", true) + &account("a", true)));
        let err = take_account(&[path], None).unwrap_err();

        assert_eq!(
            code_of::<ErrorCode>(&err),
            Some(ErrorCode::AccountDefaultSeveral)
        );
        assert!(err.to_string().contains("(a, b)"), "{err}");
    }

    /// A `:` belongs to a path (`C:\` on Windows), so `-c` never splits on
    /// it.
    #[test]
    fn a_delimited_config_value_stays_one_path() {
        let cli = Cli::try_parse_from(["comodoro", "-c", "a.toml:b.toml"]).unwrap();
        assert_eq!(cli.config.paths, [PathBuf::from("a.toml:b.toml")]);
    }
}
