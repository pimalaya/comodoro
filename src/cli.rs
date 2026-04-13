// This file is part of Comodoro, a CLI to manage timers.
//
// Copyright (C) 2025-2026 Clément DOUIN <pimalaya.org@posteo.net>
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Affero General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this program. If not, see
// <https://www.gnu.org/licenses/>.

use std::path::PathBuf;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use log::trace;
use pimalaya_toolbox::{
    config::TomlConfig,
    long_version,
    terminal::{
        clap::{
            args::{AccountFlag, ConfigPathsArg, JsonFlag, LogFlags},
            commands::{CompletionCommand, ManualCommand},
        },
        printer::Printer,
    },
};

use crate::{
    client::{
        get::TimerGetCommand, pause::TimerPauseCommand, resume::TimerResumeCommand,
        start::TimerStartCommand, stop::TimerStopCommand,
    },
    config::Config,
    server::ServerSubcommand,
};

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
    pub account: AccountFlag,
    #[command(flatten)]
    pub json: JsonFlag,
    #[command(flatten)]
    pub log: LogFlags,
}

#[derive(Subcommand, Debug)]
pub enum ComodoroCommand {
    #[command(arg_required_else_help = true, alias = "mans")]
    Manuals(ManualCommand),
    #[command(arg_required_else_help = true, alias = "cpl")]
    Completions(CompletionCommand),

    #[command(arg_required_else_help = true)]
    #[command(subcommand)]
    #[command(alias = "srvs", visible_alias = "srv")]
    Servers(ServerSubcommand),

    Start(TimerStartCommand),
    Get(TimerGetCommand),
    Pause(TimerPauseCommand),
    Resume(TimerResumeCommand),
    Stop(TimerStopCommand),
}

impl ComodoroCommand {
    pub fn execute(
        self,
        printer: &mut impl Printer,
        config_paths: &[PathBuf],
        account_name: Option<&str>,
    ) -> Result<()> {
        trace!("config paths: {config_paths:?}");
        trace!("account name: {account_name:?}");

        match self {
            Self::Manuals(cmd) => cmd.execute(printer, Cli::command()),
            Self::Completions(cmd) => cmd.execute(printer, Cli::command()),

            Self::Servers(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(&account)
            }

            Self::Start(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Get(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Pause(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Resume(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(printer, &account)
            }
            Self::Stop(cmd) => {
                let config = Config::from_paths_or_default(config_paths)?;
                let (_, account) = config.get_account(account_name)?;
                cmd.execute(printer, &account)
            }
        }
    }
}
