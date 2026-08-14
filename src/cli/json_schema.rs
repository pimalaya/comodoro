//! JSON Schema registry for the `json-schema <DIR>` meta command.
//!
//! For every command emitting structured `--json` output, this module
//! maps a CLI-invocation key (the command path joined with hyphens and
//! prefixed `comodoro-`, mirroring how the man pages are named
//! `comodoro-<cmd>.1`) to the JSON Schema describing that command's
//! payload. [`JsonSchemaCommand`] writes one `<key>.json` file per
//! entry.
//!
//! The commands driving the timer, `start`, `pause`, `resume`, `stop`
//! and `set`, report a confirmation rather than data, so they carry no
//! schema: what they did travels as timer events on the wire, described
//! in cairn/spec/protocol.md.
//!
//! [`JsonSchemaCommand`]: pimalaya_cli::clap::commands::JsonSchemaCommand

use alloc::string::{String, ToString};

use std::collections::BTreeMap;

use schemars::schema_for;
use serde_json::Value;

use crate::{cli::configure::GeneratedConfig, timer::Timer};

/// Builds the command-to-schema map consumed by `json-schema <DIR>`.
///
/// Each value is the JSON Schema of the concrete Rust type the command
/// hands to `printer.out(...)`, which is the same value serialized
/// under `--json`.
pub fn generate() -> BTreeMap<String, Value> {
    let mut schemas = BTreeMap::new();

    macro_rules! insert {
        ($key:expr, $ty:ty) => {
            schemas.insert(
                $key.to_string(),
                serde_json::to_value(schema_for!($ty)).unwrap(),
            );
        };
    }

    // `get` prints the timer once, `watch` prints the same timer on
    // every change, so both describe the same payload.
    insert!("comodoro-get", Timer);
    insert!("comodoro-watch", Timer);
    insert!("comodoro-configure", GeneratedConfig);

    schemas
}
