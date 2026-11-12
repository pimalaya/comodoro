//! # Error codes
//!
//! A stable, machine-readable name for the failures a caller is expected
//! to act on, printed as `code` beside the message under `--json`.
//!
//! The message stays for humans and may be reworded at any time; the
//! code is the contract. A failure with no code prints `code: null`. The
//! mechanism (`CodedError`, `code_of`, the report) lives in
//! `pimalaya_cli::error`; only the vocabulary is Comodoro's.
//!
//! The client is library code free of pimalaya-cli, so it attaches a
//! [`TimerClientError`] to its failures, and [`coded`] gives the chain
//! the matching code.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use anyhow::Error;
use pimalaya_cli::error::CodedError;
use schemars::JsonSchema;
use serde::Serialize;

use crate::client::std::TimerClientError;

/// The stable codes, each serialised in kebab-case.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorCode {
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

/// Gives a failure the code of the client cause it carries, its messages
/// unchanged, and leaves any other failure as it is.
///
/// The chain is rebuilt with a [`CodedError`] at its root, since that is
/// where the shared report looks for a code, each message kept at its
/// place.
pub fn coded(err: Error) -> Error {
    let Some(cause) = err.downcast_ref::<TimerClientError>() else {
        return err;
    };

    let code = ErrorCode::from(*cause);
    let mut messages: Vec<String> = err.chain().map(ToString::to_string).collect();
    let root = messages.pop().unwrap_or_default();
    let mut coded = Error::new(CodedError::new(code, root));

    for message in messages.into_iter().rev() {
        coded = coded.context(message);
    }

    coded
}

#[cfg(test)]
mod tests {
    use alloc::{format, vec::Vec};

    use anyhow::{Context, anyhow};
    use pimalaya_cli::error::code_of;

    use crate::{
        cli::error::{ErrorCode, coded},
        client::std::TimerClientError,
    };

    #[test]
    fn a_client_cause_is_coded_with_its_messages_kept() {
        let err = Err::<(), _>(anyhow!("Connection refused"))
            .context("Connect to timer server at /tmp/c.sock error")
            .context(TimerClientError::Unreachable)
            .unwrap_err();
        let before = format!("{err:#}");

        let err = coded(err);

        assert_eq!(
            code_of::<ErrorCode>(&err),
            Some(ErrorCode::ServerUnreachable)
        );
        assert_eq!(format!("{err:#}"), before);
        assert_eq!(err.chain().count(), 3);
    }

    #[test]
    fn a_failure_with_no_client_cause_stays_uncoded() {
        let err = coded(anyhow!("Something else"));
        assert_eq!(code_of::<ErrorCode>(&err), None);
        assert_eq!(err.chain().collect::<Vec<_>>().len(), 1);
    }
}
