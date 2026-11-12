---
cairn: tasks
change: json-error-code
---

# Tasks

- [ ] Test: `get --json` against a scratch socket nobody listens on prints `"code": "server-unreachable"`; seen failing first
- [ ] Tests: a fake server closing before answering, answering a JSON-RPC error, and answering the 2.x shape print `server-closed`, `server-error` and `server-mismatch`
- [ ] src/client/std.rs: `TimerClientError`, built where the client fails
- [ ] src/cli/error.rs: `ErrorCode`; src/main.rs on `eval_coded`
- [ ] src/cli/json_schema.rs: `comodoro-error` typed by the code vocabulary
- [ ] Check whether pimalaya-cli 0.4 prints `code: null` or omits it for an uncoded failure, and spell the requirement accordingly
- [ ] CHANGELOG: Added entry
- [ ] `cargo fmt --check`, clippy (default, all features, `--no-default-features --features client`), `cargo test --all-features`; `json-schema -d` diff showing only `comodoro-error`
- [ ] Fold the delta into spec/output.md, log, land, archive
