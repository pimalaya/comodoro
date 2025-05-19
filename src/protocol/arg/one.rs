use std::ops::Deref;

use clap::Parser;

use crate::protocol::Protocol;

/// The protocol name argument parser.
#[derive(Debug, Parser)]
pub struct ProtocolArg {
    /// Protocol used to send requests.
    #[arg(name = "protocol", value_name = "PROTOCOL")]
    pub value: Option<Protocol>,
}

impl Deref for ProtocolArg {
    type Target = Option<Protocol>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
