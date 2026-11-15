//! # Error codes
//!
//! A stable, machine-readable name for the failures a caller is expected
//! to act on, printed as `code` beside the message under `--json`.
//!
//! The message stays for humans and may be reworded at any time; the
//! code is the contract. A failure with no code prints `code: null`. The
//! mechanism (`CodedError`, `code_of`, `with_code`, the report) lives in
//! `pimalaya_cli::error`; only the vocabulary is Comodoro's.
//!
//! The client is library code free of pimalaya-cli, so it attaches a
//! [`TimerClientError`] to its failures, and pimalaya-config fails the
//! account resolution with an [`AccountError`]: [`code`] reads the code
//! off either, for `pimalaya_cli::error::with_code` to give the chain.

use anyhow::Error;
use pimalaya_config::toml::AccountError;
use schemars::JsonSchema;
use serde::Serialize;

use crate::client::std::TimerClientError;

/// The stable codes, each serialised in kebab-case.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorCode {
    /// No account has the name `-a` gave.
    AccountMissing,
    /// No account is marked default, and `-a` named none.
    AccountDefaultMissing,
    /// Several accounts are marked default, and `-a` named none.
    AccountDefaultSeveral,
    /// Nothing listens at the transport's address: `comodoro server
    /// start` starts a server.
    ServerUnreachable,
    /// The server closed the connection before answering.
    ServerClosed,
    /// The server answered with a JSON-RPC error.
    ServerError,
    /// The server's answer cannot be read, as from a server of another
    /// Comodoro version: restarting it with this version fixes it.
    ServerMismatch,
}

impl From<TimerClientError> for ErrorCode {
    fn from(err: TimerClientError) -> Self {
        match err {
            TimerClientError::Unreachable => Self::ServerUnreachable,
            TimerClientError::Closed => Self::ServerClosed,
            TimerClientError::Server => Self::ServerError,
            TimerClientError::Mismatch => Self::ServerMismatch,
        }
    }
}

impl From<&AccountError> for ErrorCode {
    fn from(err: &AccountError) -> Self {
        match err {
            AccountError::Missing { .. } => Self::AccountMissing,
            AccountError::NoDefault => Self::AccountDefaultMissing,
            AccountError::SeveralDefaults(_) => Self::AccountDefaultSeveral,
        }
    }
}

/// The code of the client or account cause a failure carries, if any.
pub fn code(err: &Error) -> Option<ErrorCode> {
    if let Some(cause) = err.downcast_ref::<TimerClientError>() {
        return Some(ErrorCode::from(*cause));
    }

    err.downcast_ref::<AccountError>().map(ErrorCode::from)
}

#[cfg(test)]
mod tests {
    use anyhow::{Context, anyhow};
    use pimalaya_cli::error::{code_of, with_code};

    use crate::{
        cli::error::{ErrorCode, code},
        client::std::TimerClientError,
    };

    #[test]
    fn a_client_cause_under_context_is_coded() {
        let err = Err::<(), _>(anyhow!("Connection refused"))
            .context("Connect to timer server at /tmp/c.sock error")
            .context(TimerClientError::Unreachable)
            .context("Get timer error")
            .unwrap_err();

        let err = with_code(err, code);

        assert_eq!(
            code_of::<ErrorCode>(&err),
            Some(ErrorCode::ServerUnreachable)
        );
    }

    #[test]
    fn a_failure_with_no_known_cause_has_no_code() {
        assert_eq!(code(&anyhow!("Something else")), None);
    }
}
