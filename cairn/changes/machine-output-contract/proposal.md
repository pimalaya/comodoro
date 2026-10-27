---
cairn: change
id: machine-output-contract
status: landed
created: 2026-10-27
---

# Adopt the shared machine output contract

## Why

pimalaya-cli now owns the machine contract every Pimalaya binary prints (cli `machine-output-contract`, commit `55b6693`): a schema for the failure object, schemas stamped with the contract version, piped `--json` delimited by newlines and flushed, and a check that every payload is a JSON object, which is what an MCP client takes as structured output. Comodoro was on the 0.2.4 release, whose printer wrote piped JSON without a newline or a flush, so `comodoro watch --json` piped into a status bar printed nothing until the buffer filled, and then values glued together.

## What

- Pin pimalaya-cli at `55b6693` in `[patch.crates-io]` until its next release.
- src/cli/json_schema.rs returns a `JsonSchemaRegistry` (bin `comodoro`, the crate version), registers `comodoro-error` as `ErrorReport`, and tests that no payload root is anything but an object.

Comodoro has no stable failure codes, so it keeps the uncoded `ErrorReport::eval` and the error object prints exactly as before. No payload changes: every registered root already is an object. `watch` had no newline workaround for JSON to remove: the newline `DisplayTimer` writes only reaches the plain text output, which the printer still writes as given. Keys stay as they are; the camelCase switch is its own change (`json-keys-are-camel-case`).
