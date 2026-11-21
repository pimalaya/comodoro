---
cairn: log
change: the-sample-takes-no-decision
landed: 2026-11-21
---

# The sample takes no decision

config.sample.toml follows himalaya's sample convention (himalaya 4d100c4b) and GUIDELINES inline-007: `default = true` is the one active line, every other key is commented at its default or an example, the other values of an enumeration listed in brackets at the end of its comment, so a copied sample decides nothing for the user beyond the preset uncommented.

- `precision` is commented at its default `minute`, `[second, hour]` in its comment, where it was active with its alternatives as more commented lines.
- `socket.default` is listed, the tie rule with `tcp.default` stated.
- The hook names are documented where they are keyed: `on-timer-<event>` `[start, stop]` and `on-<cycle>-<event>` `[begin, tick, set, pause, resume, end]`, the cycle in kebab-case; a cycle hook's command is listed beside the timer one, and the notification is one inline table with its summary and body.
- `default = true` reads as himalaya's, "the account commands run on without -a".

The sample test (src/cli/config.rs) parses every preset with every commented line on top, the new ones included.
