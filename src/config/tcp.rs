use serde::{Deserialize, Serialize};

use super::DurationsConfig;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct TcpConfig {
    #[serde(flatten)]
    pub durations: DurationsConfig,
    pub host: String,
    pub port: u16,
}
