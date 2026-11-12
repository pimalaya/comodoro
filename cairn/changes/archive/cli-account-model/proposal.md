---
cairn: change
id: cli-account-model
status: landed
created: 2026-08-14
---

# Separate what the document holds from what a command runs

## Why

`AccountConfig` is both the TOML shape and the thing every command reads, so resolution lives on a serde type: `address`, `addresses` and the socket-over-TCP tie-break are recomputed on every call from fields that only the document cares about. A command asking for an address has to know that `socket.default` beats `tcp.default`, and the server rebuilds a `TimerConfig` out of loose fields on every start.

The naming compounds it. Every item under `cli` carries a `Comodoro` prefix earned by the guideline for public library API, but nothing under `cli` is meant to be consumed as a library, and the crate name already namespaces it. Meanwhile the library calls its own timer settings `TimerConfig`, which reads as the configuration the CLI actually has, and wraps its cycles in a `TimerCycles` newtype that serializes transparently and dereferences to the vector it wraps.

A hook returning `Result` is the last one: the call site catches and logs it today, which is right, but nothing stops the next caller from propagating a failed notification into the tick loop and stopping the timer with it.

## What

`cli::account::Account` becomes the merged, command-ready view, built from the configuration with `From<AccountConfig>` as ortie, himalaya and cardamum already do. It resolves the addresses, the schedule and the precision once, and drops what only the document cares about, starting with `default`. Commands take `&Account`. `cli::config` keeps the document alone: the shapes, the loader, and `AccountConfig::render`, which writes an account back and is the direction a resolved account cannot take, having lost the difference between a field left out and a field written to today's default.

Items under `cli` lose the product prefix. Library items keep their domain one.

`TimerConfig` becomes `TimerSchedule { cycles, loops }`, saying what a timer runs rather than where it came from, and `loops` stops reading as a count of cycles. `TimerCycles` goes, along with `Timer::cycles_count`, which mirrored the schedule's own loop count and was never decremented despite saying so.

`TimerHook::execute` returns nothing and logs what fails, so a hook cannot stop the timer by construction.
