---
cairn: spec
capability: packaging
---

# Packaging

How Comodoro is built and shipped.

## Requirement: The sample ships beside the binary

The Nix package SHALL install config.sample.toml as share/doc/comodoro/config.sample.toml, the release archives SHALL carry it with the share/ of the build they come from, and install.sh SHALL copy it to the same path under its prefix when the downloaded archive holds it, so the first-run welcome copies it rather than downloading it.
