---
cairn: log
change: the-family-standards
landed: 2026-11-12
---

# comodoro took the family standards

The eleven changes landed on 2026-11-12 in the order the proposal set, each logged under its own id: the-wizard-leaves, wire-values-are-kebab-case, the-outputs-become-data, the-shared-kit, json-error-code, the-sample-is-tested-and-shipped, hook-templates-from-config, dependencies-trimmed, every-feature-set-builds-clean, the-shared-repl-and-mcp and release-polish. cli-alignment-leftovers was abandoned, its A1 and F6 landing in the-shared-kit.

Where the landing departed from the proposals, each log says so: the sample's presets are one-line arrays and its hook example a lone summary (pimalaya-config's `sample::variants` reads no inline-table element), an empty `cycles` is refused (found on the way), the per-second hook is `on-<cycle>-tick` (the owner's answer pending), the REPL and MCP modules live under src/cli/, and the instructions test checks every tool is named.

Open for the owner: `running` versus `tick`; teaching `sample::variants` inline-table elements in pimalaya-config, after which the presets can go multi-line; the 3.0.0 bump and the release with the family.

Capabilities: config, output, protocol, hooks, packaging, repl and mcp, as each change's log says; json-output folded into output.
