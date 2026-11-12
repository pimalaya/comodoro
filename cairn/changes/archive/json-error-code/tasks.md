---
cairn: tasks
change: json-error-code
---

# Tasks

- [x] Test: `get --json` against a scratch socket nobody listens on prints `"code": "server-unreachable"`; seen failing first
- [x] Tests: a fake server closing before answering, answering a JSON-RPC error, and answering the 2.x shape print `server-closed`, `server-error` and `server-mismatch`
- [x] src/client/std.rs: `TimerClientError`, built where the client fails
- [x] src/cli/error.rs: `ErrorCode`; src/main.rs on `eval_coded`
- [x] src/cli/json_schema.rs: `comodoro-error` typed by the code vocabulary
- [x] Check whether pimalaya-cli 0.4 prints `code: null` or omits it for an uncoded failure, and spell the requirement accordingly
- [x] CHANGELOG: Added entry
- [x] `cargo fmt --check`, clippy (default, all features, `--no-default-features --features client`), `cargo test --all-features`; `json-schema -d` diff showing only `comodoro-error`
- [x] Fold the delta into spec/output.md, log, land, archive
