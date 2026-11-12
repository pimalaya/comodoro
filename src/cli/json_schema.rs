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
//! timer after their change, so every key describes that one type. The
//! one key naming no command is `comodoro-error`, the failure object any
//! command prints instead of its payload.
//!
//! [`JsonSchemaCommand`]: pimalaya_cli::clap::commands::json_schema::JsonSchemaCommand
//! [`TimerOutput`]: crate::cli::client::timer::TimerOutput

use pimalaya_cli::{clap::commands::json_schema::JsonSchemaRegistry, error::ErrorReport};

use crate::cli::client::timer::TimerOutput;

/// Builds the command-to-schema registry consumed by `json-schema`.
///
/// Each value is the JSON Schema of the Rust type the command hands to
/// `printer.out(...)`, which is what `--json` serializes, and
/// `comodoro-error` the failure object every command prints instead.
pub fn registry() -> JsonSchemaRegistry {
    let mut registry = JsonSchemaRegistry::new("comodoro", env!("CARGO_PKG_VERSION"));

    registry.insert::<TimerOutput>("comodoro-get");
    registry.insert::<TimerOutput>("comodoro-watch");
    registry.insert::<TimerOutput>("comodoro-start");
    registry.insert::<TimerOutput>("comodoro-pause");
    registry.insert::<TimerOutput>("comodoro-resume");
    registry.insert::<TimerOutput>("comodoro-stop");
    registry.insert::<TimerOutput>("comodoro-set");
    registry.insert::<ErrorReport>("comodoro-error");

    registry
}

#[cfg(test)]
mod tests {
    use alloc::{string::String, vec::Vec};

    use clap::CommandFactory;

    use crate::cli::{Cli, json_schema::registry};

    /// The commands printing nothing a schema could describe: a bare
    /// `comodoro` shows the help, and a server prints nothing while it
    /// runs.
    const SILENT: &[&str] = &["comodoro", "comodoro-server-start"];

    /// Every runnable command is described, and every key names one: a
    /// renamed subcommand cannot leave a schema nobody can ask for, nor a
    /// new one print undescribed data.
    #[test]
    fn the_registry_matches_the_command_tree() {
        assert_eq!(
            registry().check(&Cli::command(), SILENT),
            Vec::<String>::new()
        );
    }

    #[test]
    fn every_payload_is_an_object() {
        assert_eq!(registry().non_object_roots(), Vec::<String>::new());
    }
}
