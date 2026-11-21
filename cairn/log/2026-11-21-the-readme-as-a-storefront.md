---
cairn: log
change: the-readme-as-a-storefront
landed: 2026-11-21
---

# The README as a storefront

The README follows GUIDELINES of 2026-11-21 (feeedfa), on ortie's and neverest's model.

- Header: the docs.rs badge goes, no library being published any more (readme-004); a terminal block shows `comodoro start --json` and its output (readme-006), checked by tests/docs.rs against a `server start` child, `startedAt` compared as a UTC instant only.
- Features: eight one-line bullets of 120 characters at most (readme-008), the layers block of an io- library gone (readme-005) and the cargo-feature tip with it.
- Installation: the `<details>` sections of ortie, apt and dnf from pkg.pimalaya.org and the Homebrew tap first (packages/catalogue.toml lists comodoro on all three), then the installer, two cargo lines, Nix and a `cargo build` from sources (readme-010).
- Configuration: the paths template and the tip, then one paragraph on presets and the TCP listener (readme-010, readme-017).
- Usage: four steps and one block of use cases (readme-017); the REPL and MCP subsections, the exit codes, the logging notes and the MCP client JSON are gone, all of them in `--help` (readme-002).
- tests/docs.rs also checks the Features budget and the section order, as neverest's does.
