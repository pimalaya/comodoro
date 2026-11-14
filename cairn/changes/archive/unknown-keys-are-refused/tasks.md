---
cairn: tasks
change: unknown-keys-are-refused
---

# Tasks

- [x] Audit every configuration struct for `deny_unknown_fields` and `flatten`: only the cycle lacked the refusal, no flatten.
- [x] Tests through the binary, run against the code before the change: the cycle case of the unknown-key test and both removed-key tests failed; the root, account, `tcp`, `socket` and hook cases already passed.
- [x] `CycleConfig` with `deny_unknown_fields`, converted when the account resolves.
- [x] `on-<cycle>-running` and `cmd` refused naming their replacement and MIGRATION.md.
- [x] tests/hook.rs: the no-event test takes `on-work-done`, `on-work-running` now having its own refusal.
- [x] The shipped sample still loads (`the_sample_configuration_parses_with_each_variant`).
- [x] CHANGELOG, MIGRATION.md; delta folded into cairn/spec/config.md and hooks.md; log written; status landed; archived.
