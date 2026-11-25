---
cairn: log
change: config-sample-on-himalayas-shape
landed: 2026-11-25
---

# The sample on Himalaya's shape

config.sample.toml follows himalaya/config.sample.toml, the model the maintainer chose, and GUIDELINES inline-007. The three cycle presets stay commented, as the-sample-takes-no-decision ruled.

## What landed

- The hook notification is written as the flat keys `hooks.on-work-end.notify.summary` and `hooks.on-work-end.notify.body`, no inline table.
- The hook name notes no longer end in brackets, which Himalaya's test reads as alternative values: the timer events and the cycle events are listed in prose.
- Each preset's name is one heading line, with no `#` separator line under it.
- src/cli/config.rs: `the_sample_configuration_parses_with_each_variant` is Himalaya's, ported with its helpers (`SampleLine`, `sample_paragraphs`, `sample_ranks`, `sample_variant`, `load_sample`), through `cli::take_account`. Adapted three ways: as written the sample is asserted refused for its missing `cycles` alone, since no preset is active; a commented paragraph setting a field the one above sets replaces nothing, so the three presets take one rank each through the field's values instead of chaining as replacements; a build without `notify` drops the `notify` lines. The `sample` feature of the pimalaya-config dev-dependency is no longer used and the dev-dependency is gone.

## Spec

cairn/spec/config.md: "One sample configuration, with a section per preset" describes the flat notification keys, the bracket rule and the new test.
