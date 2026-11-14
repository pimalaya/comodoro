---
cairn: change
id: a-refused-line-exits-two
status: landed
created: 2026-11-14
---

# A refused line exits two

## Why

On 2026-11-13 pimalaya-cli settled the family's exit contract (cli/cairn/log/2026-11-13-a-refused-line-exits-two.md, commit eb37962): 0 success, 1 the failure object, 2 a command line the grammar refuses, 3 and above a partial outcome. Under `--json`, a refused line prints the failure object coded `usage-invalid` on stdout, so a script parsing stdout meets one JSON value whatever went wrong. Himalaya, neverest, cardamum and calendula adopted it on 2026-11-13; on 2026-11-14 Himalaya and neverest coded the server refusals they meet and pinned pimalaya-cli 0be1b1f.

Comodoro landed its family standards on 2026-11-12, the day before: src/main.rs:27 calls clap's own `Cli::parse()`, so `comodoro --json get --nope` prints clap's text on stderr and nothing on stdout, and a refused REPL line or MCP argument carries no code. Its failures are otherwise coded (cairn/spec/output.md, "A JSON error carries a stable code": `server-unreachable`, `server-closed`, `server-error`, `server-mismatch`), and it has no partial outcome.

The rest of the 2026-11-14 migration does not reach Comodoro: it links no io-* crate, talks to no server but its own JSON-RPC one, and has no command listing or acting on several items, so it gains neither the shared refusal codes (`permission-denied`, `quota-exceeded`, `rate-limited`) nor `errors` lists.

## What

Two phases, each one commit, in this order. Neither needs anything outside this repository.

### Phase 1, the libraries

- Cargo.toml: `[patch.crates-io]` `pimalaya-cli` 0be1b1f08d19ccb9d16ee8e1d4266c27c96e0cdd (no comment; the commit Himalaya and neverest pin: eb37962's `clap::parse`, `usage_error` and `EXIT_USAGE`, plus `ItemError`, unused here; until pimalaya-cli 0.4.1). The build-dependency resolves to the same patched crate.
- `cargo update`, which takes pimalaya-config 0.4.1: a credential command prompting for a password (a hook or a server command reading a secret) reads the terminal again (pimalaya/config#2).
- The pin brings pimalaya-cli's other unreleased changes: `EXIT_PARTIAL` moving from 2 to 3 (Comodoro returns no `PartialOutcome`), the help footer's wording, and a manual page per command at every depth. Tests pinning either follow.
- CHANGELOG: a Fixed line for the prompting secret command.

### Phase 2, a refused command line prints its failure object

- src/main.rs:27: `pimalaya_cli::clap::parse::<Cli>()`. A refused line still exits 2; with a `--json` word before any `--` it prints on stdout `{"code":"usage-invalid","error":…,"sources":[],"backtrace":null}`, clap's message without `error: ` nor styles, and nothing on stderr; without it, clap's rendering on stderr as today. Help and version print and exit 0 as clap does.
- REPL: a piped refused line is coded `usage-invalid` by pimalaya-cli itself and reads `Error: unrecognized subcommand …` in text instead of `Error: error: …`. The session-flag refusal ("Session flags are refused by name") stays pimalaya-cli's, uncoded.
- MCP: an argument a tool does not declare is refused by pimalaya-cli coded `usage-invalid`, and a value the grammar refuses (a negative `seconds` for `set`) goes through `pimalaya_cli::error::usage_error`, as himalaya/src/mcp.rs maps a tool call's clap error. An unknown tool (src/cli/mcp.rs:192) stays a JSON-RPC error, as the spec says.
- `json-schema`: `comodoro-error`'s `code` is typed by `ErrorCode` and pimalaya-cli's `usage-invalid` (the report resolves it through its `ReportCode`), which the registry already does once the pin is in.
- tests/exit.rs (new, on the built binary): `--json get --nope` prints the coded object on stdout, stderr empty, exit 2; without `--json`, clap's text on stderr, stdout empty, exit 2; `--json` after `--` stays an argument. tests/repl.rs and tests/mcp.rs gain the coded refusals.

### Docs

CHANGELOG Unreleased: **Breaking:** a command line the grammar refuses under `--json` prints the failure object coded `usage-invalid` on stdout, where clap printed its text on stderr; it still exits 2, the family keeping 2 for a refused line and 3 for a partial outcome. A refused REPL line and an MCP tool call whose arguments the grammar refuses carry `usage-invalid` too. MIGRATION.md, "From v2 to v3", "Output": the same, one line. README: an exit status line (0, 1 with the failure object, 2 a refused line). The src/main.rs header names exit 2. No version bump (3.0.0 waits for the owner).

## Decisions

- **The family's id**: pimalaya-cli, Himalaya, neverest, cardamum and calendula each landed a change named a-refused-line-exits-two.
- **No code is added**: the shared refusal codes name what an HTTP or IMAP server refuses; Comodoro's server is its own, and its refusals are already `server-error`.
- **No `errors` list**: no command lists or acts on several timers.

## Scope / non-goals

- The JSON-RPC protocol and its error codes (cairn/spec/protocol.md) are untouched.
- No version bump; no release.

## Order and parallel work

This change runs in parallel with cardamum's and calendula's a-result-carries-its-errors, ortie's and carillon's a-refusal-carries-its-code and carillon's the-family-line: none touches this repository, and this one needs no shared prerequisite.
