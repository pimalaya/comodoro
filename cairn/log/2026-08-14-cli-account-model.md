---
cairn: log
change: cli-account-model
landed: 2026-08-14
---

# Separated what the document holds from what a command runs

## Why

`AccountConfig` was both the TOML shape and the thing every command read, so a command asking for an address had to go through a serde type that also knew `socket.default` beats `tcp.default`. The server rebuilt a timer configuration out of loose account fields on every start. The naming had drifted the other way: items under `cli` carried a `Comodoro` prefix earned by the rule for public library API, while the library called its own timer settings `TimerConfig`, which is the one thing in the crate that is not a configuration.

## What landed

`cli::account::Account` is the merged, command-ready view, built with `From<AccountConfig>` as ortie, himalaya and cardamum already do. It resolves the addresses, the schedule and the precision once, and drops what only the document cares about, `default` first. `cli::config` keeps the document alone, plus `AccountConfig::render` writing an account back.

Every item under `cli` lost the product prefix. `TimerConfig` became `TimerSchedule { cycles, loops }`. `TimerCycles` and `Timer::cycles_count` are gone. `TimerHook::execute` returns nothing.

The timer object is finally in cairn/spec/protocol.md, which until now said only that `timer.get` "results in the timer", leaving a third-party client to read the Rust.

## What it cost

Render cannot live on `Account`, which is the one place the separation bites: a resolved account has lost the difference between a field left out and a field written to today's default, so rendering one would freeze a runtime socket path into the file. It stays on the document type, and the wizard builds a document rather than a model.

`Timer::cycles_count` was written on creation, start and reset, always from the configured value, read once, and never decremented despite its doc saying so. It also duplicated the same number under the same name at two nesting levels in every `timer.get` payload. `TimerCycles` was a newtype that serialized transparently and dereferenced to its vector, so it was already a vector everywhere but in the type name.

## What is still open

Items under `cli` keep their `Timer` domain prefix (`TimerGetCommand`, `TimerServerCommand`, `TimerHook`), which buys nothing in a CLI with one domain.
