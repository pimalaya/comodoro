---
cairn: log
change: a-refused-line-exits-two
landed: 2026-11-14
---

# A refused line exits two

Comodoro follows pimalaya-cli's exit contract (pimalaya-cli a-refused-line-exits-two, eb37962), as Himalaya, neverest, cardamum and calendula did on 2026-11-13. pimalaya-cli is pinned at 0be1b1f by `[patch.crates-io]` until pimalaya-cli 0.4.1, and the lock took pimalaya-config 0.4.1; deny.toml allows the pimalaya GitHub organisation again for the git source. The proposal's CHANGELOG Fixed line for pimalaya-config 0.4.1 was dropped: 0.4.1 changes only `Secret`, which Comodoro never builds.

src/main.rs parses through `pimalaya_cli::clap::parse::<Cli>()`: a refused command line still exits 2, before any configuration is read, and with a `--json` word before any `--` prints the failure object coded `usage-invalid` on stdout instead of clap's text on stderr. src/cli/mcp.rs maps the clap error of a tool call's argv through `pimalaya_cli::error::usage_error`, so a value the grammar refuses (`set` with `seconds: -1`) is coded `usage-invalid` like the undeclared arguments pimalaya-cli codes itself; an unknown tool is unchanged. A refused REPL line is coded by pimalaya-cli alone. `comodoro-error`'s `code` admits `usage-invalid` through pimalaya-cli's `ReportCode`, with no change to the registry. No code was added and no `errors` list: Comodoro's server is its own and no command acts on several timers.

Tests: tests/exit.rs (new, on the built binary, its `[[test]]` entry requiring `cli` like the other binary suites: `--json get --nope` coded on stdout with stderr empty and exit 2, the same without `--json` on stderr, `--json` after `--` left a positional); tests/repl.rs (a refused line coded `usage-invalid` without the `error: ` prefix, in JSON and in text); tests/mcp.rs (an undeclared argument and `seconds: -1` coded `usage-invalid`); src/cli/json_schema.rs (`comodoro-error` admits `usage-invalid`). With the pin in and this change's two source edits reverted, the `--json` exit test and the `seconds: -1` MCP test failed; the REPL, undeclared-argument and schema tests already passed with the pin alone, and fail against pimalaya-cli 0.4.0, which codes neither a refused REPL line nor an undeclared argument and knows no `usage-invalid`.

Capabilities moved: output ("A refused command line exits 2" added; "A JSON error carries a stable code" gains `usage-invalid`), repl ("A piped session answers once per line"), mcp ("A tool's input is the command's grammar"). CHANGELOG (a **Breaking:** line, the pimalaya-cli line now naming the pin), MIGRATION.md ("Output"), README (exit statuses) and the src/main.rs header follow.

Left: a REPL line setting a session flag stays refused by pimalaya-cli uncoded, as in Himalaya.
