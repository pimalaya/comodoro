use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TcpConfig {
    #[serde(rename = "tcp-host")]
    pub host: String,
    #[serde(rename = "tcp-port")]
    pub port: u16,
}
