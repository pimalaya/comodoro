---
cairn: change
id: release-polish
status: active
created: 2026-11-12
---

# Release polish

## Why

The ten changes before this one break 2.x usage in several places (`configure`, `COMODORO_CONFIG`, the `:`-split `-c`, the wire values, the control outputs, `startedAt`, the hook name check), and the docs describe 2.x. Ortie ran the same pass under this id before its 3.0, and calendula and cardamum use it too.

## What

- README: Configuration covers the welcome, the shipped sample and its preset sections, `-c` repeated and the layered defaults; Usage gains the REPL and MCP paragraphs Ortie's README has, and says `watch --json` streams one object per change.
- MIGRATION.md, 2.x to 3.0, as Himalaya, calendula and cardamum ship one: `configure` gone, `-c` and `COMODORO_CONFIG`, the wire values and the server restart after upgrading (any status bar running a 2.x `watch` included), the controls printing the timer, `startedAt` an RFC 3339 instant, the hook name check, a copied sample needing one preset section uncommented; jq recipes for the new outputs.
- SECURITY.md: supported versions 3.x.x.
- CONTRIBUTING, the src/main.rs and src/lib.rs headers on the current architecture.
- CHANGELOG `[Unreleased]` netted into Added, Changed, Removed and Fixed, BREAKING marked.
- A check against GUIDELINES cli-001, cli-002, cli-008 and cli-009, and a cairn conformance pass (CAIRN.md §8), each gap fixed or named in the log.

## Scope / non-goals

The version bump to 3.0.0 and the release are the owner's, with the family. cli-003 to cli-007 still describe the wizard and wait for the family guidelines refactor.
