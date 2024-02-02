use clap::Parser;
use std::ops::Deref;

use crate::protocol::Protocol;

/// The protocol name argument parser.
#[derive(Debug, Parser)]
pub struct ProtocolArg {
    /// Protocol used to send requests.
    #[arg(name = "protocol", value_name = "PROTOCOL")]
    pub value: Protocol,
}

impl Deref for ProtocolArg {
    type Target = Protocol;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
