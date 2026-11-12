---
cairn: change
id: json-error-code
status: active
created: 2026-11-12
---

# A JSON error carries a stable code

## Why

Himalaya, calendula and cardamum print a stable `code` beside `error` in the failure object, so a script or an agent acts on the cause without matching free wording (himalaya and cardamum cairn/spec/commands.md, "A JSON error carries a stable code"). comodoro evaluates with `ErrorReport::eval` and prints none, though its failures are few and each calls for a different move: no server listening means running `comodoro server start`, a server of another version means restarting it (wire-values-are-kebab-case), a JSON-RPC error means the request was wrong.

## What

- src/client/std.rs fails with a typed `TimerClientError` behind the `anyhow::Error` it returns, so the library stays free of pimalaya-cli: `Unreachable` (connecting failed), `Closed` (the connection ended before the answer, src/client/std.rs:147), `Server` (a JSON-RPC error answer, :161) and `Mismatch` (an answer the client cannot read, the version error of wire-values-are-kebab-case).
- A kebab-case `ErrorCode` in a new src/cli/error.rs, implementing pimalaya-cli's `ErrorCode`, maps them found anywhere in the chain: `server-unreachable`, `server-closed`, `server-error`, `server-mismatch`.
- src/main.rs evaluates with `ErrorReport::<ErrorCode>::eval_coded`; the MCP server of the-shared-repl-and-mcp uses `exit_on_failure::<ErrorCode, _>` and `ToolResult::printed::<ErrorCode>`.
- `comodoro-error` describes the code vocabulary.

## Scope / non-goals

No code for configuration failures: pimalaya-config's `AccountError` and `ConfigMissingError` keep their messages, as in Himalaya. The text output does not change.
