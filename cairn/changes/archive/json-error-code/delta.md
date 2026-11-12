---
cairn: delta
change: json-error-code
---

# Delta

## ADDED Requirements

### Requirement: A JSON error carries a stable code
(output) A failure a caller is expected to act on SHALL carry a stable code, printed under `--json` as a `code` string beside `error`, `sources` and `backtrace`. The code SHALL be found anywhere in the error chain, so added context does not hide it. The plain output SHALL stay unchanged. Codes are kebab-case and never renamed; `error` stays free wording.

| Code | Raised when |
|---|---|
| `server-unreachable` | nothing listens at the transport's address; `comodoro server start` starts a server |
| `server-closed` | the server closed the connection before answering |
| `server-error` | the server answered a JSON-RPC error |
| `server-mismatch` | the server's answer cannot be read, as from a server of another Comodoro version; restarting it with this version fixes it |

#### Scenario: No server is running
- GIVEN an account whose socket nobody listens on
- WHEN `comodoro get --json` runs
- THEN it prints the failure object with `"code": "server-unreachable"` and exits 1

## MODIFIED Requirements

None.

## REMOVED Requirements

None.
