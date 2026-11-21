//! # Account list command
//!
//! The `account list` subcommand, listing the accounts a configuration
//! declares as pimalaya-cli's shared [`AccountListOutput`] prints them.

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::{
    account::{AccountListOutput, AccountRow},
    printer::Printer,
};
use pimalaya_config::toml::TomlConfig;

use crate::cli::config::Config;

/// List the accounts of the configuration.
///
/// One line per account: its name, the transports it declares (those it
/// holds a table for, and the default one), and `default` on the one a
/// command passing no `-a <NAME>` takes. Nothing is reached: this reads
/// the configuration only.
///
/// JSON output: `{"accounts": [{"name", "default", "backends",
/// "notBuilt"}]}`, `backends` the transports, `socket` and `tcp`, and
/// `notBuilt` always empty, every build serving both.
#[derive(Debug, Parser)]
pub struct AccountListCommand;

impl AccountListCommand {
    /// Loads the configuration, taking no account, and prints its
    /// accounts sorted by name.
    pub fn execute(self, printer: &mut impl Printer, config_paths: &[PathBuf]) -> Result<()> {
        let config = Config::load(config_paths)?;

        let mut accounts: Vec<AccountRow> = config
            .accounts
            .iter()
            .map(|(name, account)| AccountRow {
                name: name.clone(),
                default: account.default,
                backends: account.transports().into_iter().map(|t| t.name()).collect(),
                not_built: Vec::new(),
            })
            .collect();
        accounts.sort_by(|a, b| a.name.cmp(&b.name));

        printer.out(AccountListOutput { accounts })
    }
}
