//! # JSON Schema registry
//!
//! The schemas the `json-schema` command writes, one entry per command
//! emitting structured `--json` output.
//!
//! A key is the command path joined with hyphens and prefixed
//! `comodoro-`, mirroring how the man pages are named `comodoro-<cmd>.1`,
//! and [`JsonSchemaCommand`] writes one `<key>.json` file per entry.
//!
//! Every client command prints the same [`TimerOutput`], the controls the
//! timer after their change, so every key describes that one type.
//!
//! [`JsonSchemaCommand`]: pimalaya_cli::clap::commands::JsonSchemaCommand
//! [`TimerOutput`]: crate::cli::client::timer::TimerOutput

use alloc::string::ToString;

use std::collections::BTreeMap;

use pimalaya_cli::{clap::commands::JsonSchemaRegistry, error::ErrorReport};
use schemars::schema_for;

use crate::cli::client::timer::TimerOutput;

/// Builds the command-to-schema registry consumed by `json-schema`.
///
/// Each value is the JSON Schema of the Rust type the command hands to
/// `printer.out(...)`, which is what `--json` serializes, and
/// `comodoro-error` the failure object every command prints instead.
pub fn registry() -> JsonSchemaRegistry {
    let mut schemas = BTreeMap::new();

    macro_rules! insert {
        ($key:expr, $ty:ty) => {
            schemas.insert(
                $key.to_string(),
                serde_json::to_value(schema_for!($ty)).unwrap(),
            );
        };
    }

    insert!("comodoro-get", TimerOutput);
    insert!("comodoro-watch", TimerOutput);
    insert!("comodoro-start", TimerOutput);
    insert!("comodoro-pause", TimerOutput);
    insert!("comodoro-resume", TimerOutput);
    insert!("comodoro-stop", TimerOutput);
    insert!("comodoro-set", TimerOutput);
    insert!("comodoro-error", ErrorReport);

    JsonSchemaRegistry {
        bin: "comodoro",
        version: env!("CARGO_PKG_VERSION"),
        schemas,
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    #[test]
    fn every_payload_is_an_object() {
        assert_eq!(super::registry().non_object_roots(), Vec::<&str>::new());
    }
}
