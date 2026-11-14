---
cairn: tasks
change: a-refused-line-exits-two
---

# Tasks

## Working rules

- Read first: this change's proposal.md and delta.md; cli/cairn/log/2026-11-13-a-refused-line-exits-two.md, cli/src/clap/mod.rs (`parse`) and cli/src/error.rs (`usage_error`, `CommonCode`, `ReportCode`); himalaya/cairn/log/2026-11-13-a-refused-line-exits-two.md, himalaya/tests/exit.rs and himalaya/src/mcp.rs (the tool call's clap error through `usage_error`); calendula/cairn/log/2026-11-13-a-refused-line-exits-two.md.
- Every cargo command runs as `nix develop --command cargo ...`. `cargo fmt` after each phase.
- An agent edits, builds and tests; it runs no git write command. The session owning the repository commits each phase with the message given here, no AI trailer.
- Never adjust production code to fit a test. A regression test must fail against the code before the phase.
- No em dashes; code comments wrap at 80 columns; markdown is never hard-wrapped.
- Phase checks, each one: `cargo test --all-features`; the CI clippy loop with `--locked --all-targets -- -D warnings` over the default set, `--all-features`, `--no-default-features`, and `--no-default-features --features` client, server, `client,server` and cli; `cargo test --locked --no-default-features --features cli`; `cargo deny check`.

## Phase 1, the libraries (commit `build: pin pimalaya-cli 0be1b1f, update deps`)

- [x] Cargo.toml `[patch.crates-io]`: pimalaya-cli 0be1b1f08d19ccb9d16ee8e1d4266c27c96e0cdd, no comment (or pimalaya-cli 0.4.1 from crates.io if released, no pin).
- [x] `cargo update`; the lock holds pimalaya-config 0.4.1.
- [x] Tests pinning the help footer or the manual pages follow pimalaya-cli's wording. None pins them; the help ends on the new Pimalaya paragraph and `manual` writes 15 pages, comodoro-server-start.1 among them.
- [x] ~~CHANGELOG: Fixed, a prompting credential command no longer hangs (pimalaya-config 0.4.1).~~ Dropped: 0.4.1 changes only `Secret` (src/secret.rs), which Comodoro never builds; its hooks run through `CommandConfig`, untouched.
- [x] Phase checks pass. deny.toml regains `allow-org.github = ["pimalaya"]`, dropped with the previous pin, so `sources` accepts the git pin.

## Phase 2, the refused line (commit `feat!: print a refused command line as a failure object under --json`)

- [ ] src/main.rs:27: `pimalaya_cli::clap::parse::<Cli>()`.
- [ ] src/cli/mcp.rs: a tool call whose argv clap refuses answers its failure through `pimalaya_cli::error::usage_error`; an unknown tool (:192) unchanged.
- [ ] tests/exit.rs (new): the three cases of the proposal.
- [ ] tests/repl.rs: a piped refused line prints `"code":"usage-invalid"` and no `error: ` prefix.
- [ ] tests/mcp.rs: an undeclared argument and a refused value (`set` with `seconds: -1`, or the grammar's equivalent) answer `isError` coded `usage-invalid`.
- [ ] src/json_schema.rs: the registry test still passes, `comodoro-error` admitting `usage-invalid`.
- [ ] Phase checks pass.

## Landing

- [ ] CHANGELOG (**Breaking:** as in the proposal), MIGRATION.md ("From v2 to v3", "Output"), README exit statuses, the src/main.rs header.
- [ ] Delta folded into cairn/spec/output.md, repl.md and mcp.md (their requirements are `##` headings, their scenarios `###`); log cairn/log/<date>-a-refused-line-exits-two.md (frontmatter `cairn: log`, `change`, `landed:`); status landed; the folder moved to cairn/changes/archive/ as Comodoro's landed changes are.
