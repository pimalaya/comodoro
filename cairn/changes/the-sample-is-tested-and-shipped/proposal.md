---
cairn: change
id: the-sample-is-tested-and-shipped
status: active
created: 2026-11-12
---

# The sample is tested and shipped

## Why

The welcome of the-shared-kit copies share/doc/comodoro/config.sample.toml when the package carries it and downloads it otherwise, but comodoro ships none, so every first run downloads. The sample is prose the code never reads: a renamed field breaks it silently, and the three cycle presets the wizard offered live nowhere a person copying the file can pick from. Ortie and Himalaya test theirs through `pimalaya_config::sample::variants` and ship it (ortie providers-in-the-sample and sample-shipped-in-share, himalaya the-sample-is-tested-and-shipped).

## What

- config.sample.toml takes Ortie's shape: no header; `[accounts.example]` the only table, with dotted keys; the shared fields first, the defaults active and the alternatives as `#key = value`; one sentence per line, aligned with literal tabs at column 40 or the next tab stop past the block's longest line; every command an `exit 127` placeholder.
- The top lists `default`, `cycles-count`, `precision` and its three values, `socket.path`, `socket.default`, `tcp.host`, `tcp.port`, `tcp.default`, and commented hooks.
- The cycle presets become sections exactly like Ortie's providers (owner, 2026-11-12): `# Pomodoro`, `# 52/17` and `# 112/26` at the end of the file, each a `# <Name>` heading, a bare `#` line, commented code only with the cycles as a multi-line `#cycles = [` / `#  { name = "Work", duration = 1500 },` / `#]` array, and a blank line. No `cycles` is active at the top, so a section sets no key the top keeps active, and the copied file holds a working account once one section is uncommented.
- `the_sample_configuration_parses_with_each_variant` in src/cli/config.rs runs each `pimalaya_config::sample::variants` of the sample through `Config::load`, asserts the single `[accounts.example]` header and resolves `example`. pimalaya-config `sample` joins the dev-dependencies.
- package.nix installs it as share/doc/comodoro/config.sample.toml (`install -Dm644`, as ortie's), install.sh copies it under its prefix when the archive holds it, and the release archives carry it with the share/ of their build through pimalaya/nix's releases.yml.

## Scope / non-goals

The hooks' variables are hook-templates-from-config's, which updates the sample's hook lines.
