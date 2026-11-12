---
cairn: log
change: the-sample-is-tested-and-shipped
landed: 2026-11-12
---

# The sample is tested and shipped

config.sample.toml took Ortie's shape: no header, `[accounts.example]` the only table, dotted keys, the defaults active (`default`, `precision = "minute"`) and the alternatives commented, one tab-aligned sentence per line, each block at column 40 or the tab stop past its longest line; then the sections `# Pomodoro`, `# 52/17` and `# 112/26`, no `cycles` active outside them.

Two deviations from the proposal, both forced by pimalaya-config's `sample::variants`, which takes a commented line for code only when it is a `key = value` field or an element opening on `"`:

- Each preset's `cycles` is a one-line array: a multi-line array of `{ name, duration }` tables would stay commented when its section is uncommented, the variant loading `cycles = []`. Teaching `variants` inline-table elements is a pimalaya-config change, left to the owner; the sample can go multi-line once it is released.
- The hook example is a lone `notify.summary`: a commented `notify.body` line is uncommented on its own as a variant, and a notification without a summary is refused.

The empty `cycles = []` those trials loaded is accepted by the loader today and only fails when the server builds its timer; hook-templates-from-config refuses it when the configuration is read.

`the_sample_configuration_parses_with_each_variant` (src/cli/config.rs) reads each variant through `Config::load` and resolves `example`, skipping the notification variants in a build without `notify`; it failed on the old sample first, which has no preset section. package.nix installs share/doc/comodoro/config.sample.toml and install.sh copies it; `nix build .#default` carries it, and the packaged binary's welcome copies it with `install -m 644` instead of running `curl`.

Capabilities: config (One sample configuration, with a section per preset ADDED); packaging created (The sample ships beside the binary).
