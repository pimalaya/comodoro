---
cairn: tasks
change: cli-account-model
---

# Tasks

- [x] Add `cli::account::Account`, built from the configuration, resolving the addresses, the schedule and the precision
- [x] Move `address`, `addresses` and the transport tie-break off `AccountConfig` onto it
- [x] Take `&Account` in every command, and the schedule in `TimerServer`
- [x] Keep `cli::config` to the document: shapes, loader, and `AccountConfig::render`
- [x] Drop the product prefix from every item under `cli`
- [x] Rename `TimerConfig` to `TimerSchedule`, and its `cycles_count` field to `loops`
- [x] Delete `TimerCycles`, which was a transparent, dereferencing wrapper around its vector
- [x] Delete `Timer::cycles_count`, which mirrored the schedule and was never decremented
- [x] Return nothing from `TimerHook::execute`, logging what fails
- [x] Move the timer rendering to `cli::client::timer`
- [x] Pin the timer object in cairn/spec/protocol.md, which only said "the timer"
- [x] Update CHANGELOG.md, README.md, CONTRIBUTING.md and config.sample.toml
