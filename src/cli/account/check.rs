//! # Account check command
//!
//! The `account check` subcommand: whether a timer server answers on
//! each transport the account declares, one row each, on pimalaya-cli's
//! shared [`AccountCheckOutput`].

use alloc::{vec, vec::Vec};

use anyhow::Result;
use clap::Parser;
use pimalaya_cli::{
    account::{AccountCheck, AccountCheckOutput, BackendCheck},
    error::{PartialOutcome, with_code},
    printer::Printer,
};

use crate::{
    cli::{
        account::Account,
        error::{self, ErrorCode},
    },
    client::std::TimerClient,
};

/// Check that a timer server answers on the account's transports.
///
/// Prints one row per transport the account declares (those it holds a
/// table for, and the default one), `socket` then `tcp`, each OK or FAIL
/// with why: the row connects and reads the timer, changing nothing.
///
/// A failed row exits the command with status 3, the report being the
/// output rather than a failure object.
///
/// JSON output: `{"accounts": [{"account", "backends": [{"backend",
/// "ok", "code", "error", "tls"}]}]}`, `backend` being `socket` or
/// `tcp` and `tls` always `null`, no transport carrying TLS.
#[derive(Debug, Parser)]
pub struct AccountCheckCommand;

impl AccountCheckCommand {
    /// Reads the timer over each transport `account` declares, printing
    /// every outcome whatever the first one was.
    pub fn execute(self, printer: &mut impl Printer, account: &Account) -> Result<()> {
        let backends: Vec<BackendCheck<ErrorCode>> = account
            .transports
            .iter()
            .map(|transport| {
                let address = account.address(Some(*transport));
                let result = TimerClient::connect(&address).and_then(|mut client| {
                    client.get()?;
                    Ok(())
                });

                let result = result.map_err(|err| with_code(err, error::code));
                BackendCheck::new(transport.name(), result)
            })
            .collect();

        let output = AccountCheckOutput {
            accounts: vec![AccountCheck {
                account: account.name.clone(),
                backends,
            }],
        };

        let ok = output.ok();
        printer.out(output)?;

        if !ok {
            return Err(PartialOutcome::default().into());
        }

        Ok(())
    }
}
