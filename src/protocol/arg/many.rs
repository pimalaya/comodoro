use clap::Parser;
use std::ops::Deref;

use crate::protocol::Protocol;

/// The protocols name argument parser.
#[derive(Debug, Parser)]
pub struct ProtocolsArg {
    /// Protocols used to accept requests from clients.
    #[arg(name = "protocols", value_name = "PROTOCOLS")]
    pub value: Option<Vec<Protocol>>,
}

impl Deref for ProtocolsArg {
    type Target = Option<Vec<Protocol>>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl From<ProtocolsArg> for Option<Vec<Protocol>> {
    fn from(arg: ProtocolsArg) -> Self {
        arg.value
    }
}
