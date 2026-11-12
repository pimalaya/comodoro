---
cairn: spec
capability: config
---

# Configuration

How Comodoro finds the account a command runs against. The fields of an account are documented in config.sample.toml; the transports they describe are cairn/spec/protocol.md.

## Requirement: Config never written

Comodoro SHALL NOT write a configuration file: the user writes it, from config.sample.toml.

## Requirement: A bare invocation shows the help

A bare `comodoro` SHALL print the help. A command needing an account and finding no configuration SHALL fail naming the path it looked at, the one `-c` or `COMODORO_CONFIG` gave or else the default location, and pointing at config.sample.toml. Comodoro SHALL NOT prompt to create a configuration.

### Scenario: A first run

With no configuration at any default location, `comodoro get` run in a terminal fails naming $XDG_CONFIG_HOME/comodoro/config.toml and the config.sample.toml URL, and asks nothing.
