//! Deserialized account config module.
//!
//! This module contains the raw deserialized representation of an
//! account in the accounts section of the user configuration file.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DeserializedAccountConfig {
    pub default: Option<bool>,
    pub work_duration: Option<usize>,
    pub short_break_duration: Option<usize>,
    pub long_break_duration: Option<usize>,
}
