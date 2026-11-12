---
cairn: log
change: the-wizard-leaves
landed: 2026-11-12
---

# The wizard left

src/cli/configure.rs is gone with the `configure` command and its `wizard` alias, `offer_configuration` and the offer inside `take_account` (src/cli.rs), `AccountConfig::render`, the `Serialize` derives on `Config`, `AccountConfig`, `SocketConfig`, `TcpConfig` and `TimerHook`, `GeneratedConfig` and the `comodoro-configure` schema, and the pimalaya-cli `prompt` feature. The wire types keep their `Serialize`.

A bare `comodoro` prints the help. A command finding no configuration fails with "No configuration found at <path>, write one by hand from the sample: <URL>", Himalaya's wording, until the welcome of the-shared-kit replaces it.

Capabilities: config created (Config never written, A bare invocation shows the help); json-output (`comodoro-configure` gone from the keys). B4 and B9 of cli-alignment-leftovers lapsed with the wizard.
