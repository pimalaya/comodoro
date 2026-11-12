---
cairn: log
change: json-error-code
landed: 2026-11-12
---

# A JSON error carries a stable code

The client (src/client/std.rs) attaches a `TimerClientError` to each failure as anyhow context: `Unreachable` on connect, `Closed` on a failed write or read and on end of stream before the answer, `Server` on a JSON-RPC error answer, `Mismatch` on an answer it cannot read or of the wrong kind. The library stays free of pimalaya-cli.

src/cli/error.rs holds the kebab-case `ErrorCode` and `coded`, which `Command::run` applies to every result: when the chain carries a `TimerClientError`, it is rebuilt with a `CodedError` at its root, where pimalaya-cli's `code_of` finds it, every message kept at its place, so the text and `sources` print as before. src/main.rs evaluates with `ErrorReport::<ErrorCode>::eval_coded`, and `comodoro-error` describes the codes. pimalaya-cli 0.4 prints `code: null` for an uncoded failure, so the spec says so.

tests/output.rs drives the binary against no server, a fake server closing without a word, one answering a JSON-RPC error and one answering the 2.x shape: the four failed on the old code first, printing `code: null`.

Capabilities: output (A JSON error carries a stable code ADDED).
