use clap::Parser;
use std::ops::Deref;

use crate::protocol::Protocol;

/// The protocols name argument parser.
#[derive(Debug, Parser)]
pub struct ProtocolsArg {
    /// Protocols use to accept requests.
    #[arg(name = "protocols", value_name = "PROTOCOL")]
    pub value: Vec<Protocol>,
}

impl Deref for ProtocolsArg {
    type Target = Vec<Protocol>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<ProtocolsArg> for Vec<Protocol> {
    fn from(arg: ProtocolsArg) -> Self {
        arg.value
    }
}
