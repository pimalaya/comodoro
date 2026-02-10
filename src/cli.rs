use std::path::PathBuf;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use pimalaya_toolbox::{
    config::TomlConfig,
    long_version,
    terminal::{
        clap::{
            args::{AccountArg, ConfigPathsArg, JsonFlag, LogFlags},
            commands::{CompletionCommand, ManualCommand},
        },
        printer::Printer,
    },
};

#[cfg(feature = "client")]
use crate::client::command::{
    get::GetTimerCommand, pause::PauseTimerCommand, resume::ResumeTimerCommand,
    start::StartTimerCommand, stop::StopTimerCommand,
};
use crate::config::Config;
#[cfg(feature = "server")]
use crate::server::command::ServerSubcommand;

#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(author, version, about)]
#[command(long_version = long_version!())]
#[command(propagate_version = true, infer_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: ComodoroCommand,
    #[command(flatten)]
    pub config: ConfigPathsArg,
    #[command(flatten)]
    pub account: AccountArg,
    #[command(flatten)]
    pub json: JsonFlag,
    #[command(flatten)]
    pub log: LogFlags,
}

#[derive(Subcommand, Debug)]
pub enum ComodoroCommand {
    #[cfg(feature = "server")]
    #[command(arg_required_else_help = true)]
    #[command(subcommand)]
    #[command(alias = "servers", alias = "srvs", alias = "srv")]
    Server(ServerSubcommand),

    #[cfg(feature = "client")]
    Start(StartTimerCommand),
    #[cfg(feature = "client")]
    Get(GetTimerCommand),
    #[cfg(feature = "client")]
    Pause(PauseTimerCommand),
    #[cfg(feature = "client")]
    Resume(ResumeTimerCommand),
    #[cfg(feature = "client")]
    Stop(StopTimerCommand),

    #[command(arg_required_else_help = true, alias = "mans")]
    Manuals(ManualCommand),
    #[command(arg_required_else_help = true)]
    Completions(CompletionCommand),
}

impl ComodoroCommand {
    pub fn execute(
        self,
        printer: &mut impl Printer,
        config_paths: &[PathBuf],
        account_name: Option<&str>,
    ) -> Result<()> {
        match self {
            #[cfg(feature = "client")]
            Self::Start(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(printer, &account)
            }
            #[cfg(feature = "client")]
            Self::Get(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(printer, &account)
            }
            #[cfg(feature = "client")]
            Self::Pause(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(printer, &account)
            }
            #[cfg(feature = "client")]
            Self::Resume(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(printer, &account)
            }
            #[cfg(feature = "client")]
            Self::Stop(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(printer, &account)
            }
            #[cfg(feature = "server")]
            Self::Server(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(&account)
            }
            Self::Manuals(cmd) => cmd.execute(printer, Cli::command()),
            Self::Completions(cmd) => cmd.execute(printer, Cli::command()),
        }
    }
}
