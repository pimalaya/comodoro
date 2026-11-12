//! # REPL tests
//!
//! `comodoro repl` piped, against an in-process timer server on a scratch
//! socket.

mod common;

use serde_json::Value;

use crate::common::{comodoro, serve};

/// The JSON values a piped `--json` session printed on stdout.
fn json_lines(stdout: &[u8]) -> Vec<Value> {
    let stdout = String::from_utf8_lossy(stdout);
    stdout
        .lines()
        .map(|line| serde_json::from_str(line).unwrap_or_else(|err| panic!("{err}: {line}")))
        .collect()
}

/// Each line gets one answer, a failure as the failure object, and the
/// session goes on after it.
#[test]
fn a_piped_json_session_answers_each_line_and_goes_on_after_a_failure() {
    let scratch = serve();
    let out = comodoro(
        &scratch.config,
        &["--json", "repl"],
        b"get\nfrobnicate\nstart\n",
    );

    assert!(out.status.success(), "{out:?}");

    let lines = json_lines(&out.stdout);
    assert_eq!(lines.len(), 3, "{lines:?}");
    assert_eq!(lines[0]["state"], "stopped");
    assert!(lines[1]["error"].is_string(), "{lines:?}");
    assert_eq!(lines[2]["state"], "running");
}

/// A line's `--json` switches that line to JSON, the next ones staying
/// on the session's text output.
#[test]
fn a_line_json_flag_applies_to_that_line() {
    let scratch = serve();
    let out = comodoro(&scratch.config, &["repl"], b"get --json\nget\n");

    assert!(out.status.success(), "{out:?}");

    let stdout = String::from_utf8(out.stdout).unwrap();
    let mut lines = stdout.lines();
    let timer: Value = serde_json::from_str(lines.next().unwrap()).unwrap();
    assert_eq!(timer["state"], "stopped", "stdout: {stdout}");
    assert_eq!(lines.next(), Some("OFF"), "stdout: {stdout}");
}

/// A flag the session fixed when it started is refused by name rather
/// than as an unexpected argument, and the session goes on.
#[test]
fn a_session_flag_on_a_line_is_refused_by_name() {
    let scratch = serve();
    let input = b"get -a other\nget --config=x.toml\nget --log-level debug\nget\n";
    let out = comodoro(&scratch.config, &["--json", "repl"], input);

    assert!(out.status.success(), "{out:?}");

    let lines = json_lines(&out.stdout);
    assert_eq!(lines.len(), 4, "{lines:?}");

    for (line, flag) in lines.iter().zip(["-a", "--config", "--log-level"]) {
        let error =
            format!("Cannot set `{flag}` inside a session, start a new session with it instead");
        assert_eq!(line["error"], error, "{lines:?}");
    }

    assert_eq!(lines[3]["state"], "stopped");
}

/// A `#` line is a comment, and a command prefix is inferred as on the
/// command line.
#[test]
fn a_comment_line_is_skipped_and_prefixes_are_inferred() {
    let scratch = serve();
    let out = comodoro(
        &scratch.config,
        &["--json", "repl"],
        b"# start\n  # stop\nge\n",
    );

    assert!(out.status.success(), "{out:?}");

    let lines = json_lines(&out.stdout);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert_eq!(lines[0]["state"], "stopped");
}

/// The help is no failure: a `--json` session prints it on stderr, so
/// stdout keeps one JSON value per line. Its usage line names the binary.
#[test]
fn the_help_goes_to_stderr_in_a_json_session() {
    let scratch = serve();
    let out = comodoro(&scratch.config, &["--json", "repl"], b"set --help\nget\n");

    assert!(out.status.success(), "{out:?}");

    let lines = json_lines(&out.stdout);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert_eq!(lines[0]["state"], "stopped");

    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("Usage: comodoro set"), "stderr: {stderr}");
}

/// A command running until interrupted would end the session with it,
/// and a session holds no other one: each is refused by name.
#[test]
fn sessions_and_commands_running_until_interrupted_are_refused() {
    let scratch = serve();
    let input = b"watch\nserver start\nrepl\nmcp\nget\n";
    let out = comodoro(&scratch.config, &["--json", "repl"], input);

    assert!(out.status.success(), "{out:?}");

    let lines = json_lines(&out.stdout);
    assert_eq!(lines.len(), 5, "{lines:?}");
    assert_eq!(
        lines[0]["error"],
        "`watch` runs until interrupted, run it outside a session"
    );
    assert_eq!(
        lines[1]["error"],
        "`server start` runs until interrupted, run it outside a session"
    );
    assert_eq!(lines[2]["error"], "Cannot start a session inside a session");
    assert_eq!(lines[3]["error"], "Cannot start a session inside a session");
    assert_eq!(lines[4]["state"], "stopped");
}
