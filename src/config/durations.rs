use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DurationsConfig {
    pub work_duration: Option<usize>,
    pub short_break_duration: Option<usize>,
    pub long_break_duration: Option<usize>,
}
