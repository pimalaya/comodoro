use pimalaya_time::TimerCycle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(any(feature = "tcp-client", feature = "tcp-binder"))]
use crate::TcpConfig;

/// Represents the user config file.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PresetConfig {
    #[serde(flatten)]
    pub preset_or_cycles: PresetKindOrCyclesConfig,
    #[cfg(any(feature = "tcp-client", feature = "tcp-binder"))]
    #[serde(flatten)]
    pub tcp: Option<TcpConfig>,
    #[serde(flatten)]
    pub hooks: HashMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PresetKindOrCyclesConfig {
    #[serde(rename = "preset")]
    Preset(PresetKind),
    #[serde(rename = "cycles")]
    Cycles(Vec<TimerCycle>),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum PresetKind {
    #[serde(rename = "pomodoro")]
    PresetPomodoro,
    #[serde(rename = "52/17")]
    Preset52_17,
}
