use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TcpConfig {
    #[serde(default)]
    pub default: bool,
    #[serde(default = "localhost")]
    pub host: String,
    pub port: u16,
}

fn localhost() -> String {
    String::from("127.0.0.1")
}
