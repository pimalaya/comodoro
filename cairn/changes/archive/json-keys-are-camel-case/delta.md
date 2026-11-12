---
cairn: delta
change: json-keys-are-camel-case
---

Folds into cairn/spec/protocol.md with the rename.

## ADDED Requirements

### Requirement: JSON keys are camelCase

Every JSON object Comodoro emits, the `--json` payloads and the JSON-RPC results and notifications alike, SHALL spell its keys in camelCase, so each key stays reachable by dot access in jq and JavaScript and so the payload matches the camelCase the method and event names already use. A field carrying a provider spelling verbatim SHALL keep its explicit `#[serde(rename)]`, since names like `@odata.nextLink` and `nextPageToken` are the provider's rather than derivable ones.

Configuration types SHALL stay kebab-case: they are read from TOML, and what the loader reads is not what the printer emits. `TimerPrecision` keeps its kebab-case spelling, renaming variant names that travel as values rather than as keys.

## MODIFIED Requirements

### Requirement: The timer is one object, and it says what it runs

The timer's `started_at` is spelled `startedAt`. Every other key it carries is a single word and unchanged.

## REMOVED Requirements
