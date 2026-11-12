---
cairn: change
id: json-keys-are-camel-case
status: landed
created: 2026-08-29
---

# JSON keys become camelCase at 3.0

## Why

The Pimalaya family standardises the object keys of `--json` on camelCase. Comodoro is 2.0.0 and both of its JSON surfaces are published contracts, so the rename is breaking and waits for 3.0. This entry records the decision so it is neither rediscovered nor forgotten.

camelCase is what the formats these tools sit next to already speak: JMAP objects are camelCase by RFC 8620, and so are Microsoft Graph and the Google APIs. Comodoro's own protocol is already half there. Its method and notification names are camelCase (`timer.durationSet` in src/protocol.rs), and `TimerEvent` carries `#[serde(rename_all = "camelCase")]` for its event names. The payload keys are the last snake_case thing on the wire.

The second reason is the consumer these payloads exist for. Neither JavaScript nor jq can use dot access on a key holding a hyphen, so a kebab-case key like `messages-total` has to be written `."messages-total"` in jq and `obj["messages-total"]` in JavaScript. Comodoro's status-bar users pipe `comodoro watch --json` into exactly those two tools, so the family settles on the one spelling both read unquoted.

## What changes at 3.0

Output types only, meaning what the commands hand to `printer.out` and register in src/cli/json_schema.rs:

- `TimerOutput` in src/cli/client/timer.rs, which serializes the library `Timer` as its whole payload, so the rename lands on `Timer` in src/timer.rs and its `started_at` becomes `startedAt`. The other keys (`schedule`, `state`, `cycle`, `elapsed`, `cycles`, `loops`, `name`, `duration`) are single words and read the same either way.
- `ConfigureOutput` in src/cli/configure.rs, whose `name`, `default` and `document` are single words too. It takes the attribute for uniformity and its payload does not move.

So the visible break is one key, `started_at`, and it is worth doing precisely because it is one key.

The break is wider than `--json`, and that is the point to weigh at 3.0. `TimerOutput` serializes the same `Timer` the JSON-RPC protocol carries in the `timer.get` result and in every event notification, described in cairn/spec/protocol.md. Renaming the field renames it on the socket and TCP wire as well, for every client, not only for `--json` readers. Both are Comodoro's published contracts and both are gated on the same major, so they move together or not at all. The spec example and prose in cairn/spec/protocol.md move with them.

Config types stay kebab-case. src/cli/config.rs is deserialized from TOML, where hyphenated keys are the family convention and no jq expression reaches them. `TimerPrecision` in src/timer.rs keeps its kebab-case rename too: what it renames are variant names read out of the configuration, which are values rather than keys.

Provider passthrough keeps its own spelling. Comodoro talks to no provider today, but the family rule applies if it ever does: a field carrying a wire name verbatim keeps its explicit `#[serde(rename = "...")]`, `@odata.nextLink` and `nextPageToken` being the standing examples, since those are the provider's names and not names derivable from a Rust field. `rename_all` leaves an explicit `rename` alone, which is correct.

## The alias trap

`#[serde(alias = "...")]` is a deserialization-only attribute. Comodoro already uses it that way, on `TimerPrecision`, where `seconds`, `secs`, `sec` and `s` are all accepted for one variant, and that is exactly the habit that makes it look like the answer here. It is not: an alias teaches `Deserialize` a second spelling and does nothing at all to `Serialize`, so it cannot make a payload emit both `started_at` and `startedAt` through a transition. A `Timer` on its way out only serializes.

The two real options are to twin the keys (emit both spellings, which grows every tick payload and makes the published schema describe two names for one value) or to accept the break at the major. The decision is to accept it.

## Out of scope

No CHANGELOG entry: nothing changes for a user until 3.0 lands the rename. The schema files `json-schema` writes change with the keys, so anything pinned on them regenerates then.
