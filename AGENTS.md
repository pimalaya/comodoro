# AGENTS.md: Cairn activation

This repository follows **Cairn**, a language- and tool-agnostic convention for keeping a living spec, reviewable change proposals, and an honest history. The full format is the [Cairn standard](https://github.com/pimalaya/cairn) (`CAIRN.md` and `GUIDE.md` in that repository). No tooling is required. You create and check the structure by reading and following the rules.

If you are an agent working in this repository, do the following **by default, without being asked**.

## 1. Locate the Cairn root

Walk **up** from your working directory to the nearest ancestor containing a `cairn/` directory or a `cairn.toml` file. That directory is the root, and all Cairn artifacts live under its base (default `cairn/`). For this repository the root is the repository root and the base is `cairn/`.

## 2. Before non-trivial work, propose

For anything beyond a trivial fix, create `cairn/changes/<change-id>/` with:

- `proposal.md`: *why* and *what* (frontmatter: `cairn: change`, `id`, `status: active`, `created`).
- `tasks.md`: the checklist (frontmatter: `cairn: tasks`, `change`).
- `delta.md`: what this changes in the spec (frontmatter: `cairn: delta`, `change`; body uses only `## ADDED Requirements`, `## MODIFIED Requirements`, `## REMOVED Requirements`, with `### Requirement:` and `#### Scenario:` under them).

Let the human review intent **before** you write code. Trivial fixes may skip this and go straight to landing.

## 3. After work lands, fold and log (never skip)

- Fold the change's delta into `cairn/spec/<capability>.md` so the spec always reflects current truth (append ADDED, replace MODIFIED, delete REMOVED). A capability file holds current truth only, with no history.
- Append a dated entry `cairn/log/YYYY-MM-DD-<change-id>.md` describing what landed and which capabilities moved (frontmatter: `cairn: log`, `change`, `landed`; log entries are immutable).
- Set the change `status: landed`. Optionally move its folder to `cairn/changes/archive/`.

> **The forcing rule:** a change that affects behaviour is not *done* until the spec is updated and the log entry is written.

## 4. Stay conformant

Check the structure yourself against the strict rules (Cairn standard §8): a discoverable root, `spec/ changes/ log/` present, every Cairn file carrying a valid `cairn:` type, each change having `proposal.md` and `tasks.md`, kebab-case ids, literal delta headings, and a log entry for every landed change. Everything else (prose, naming, ordering, extra files) is free.
