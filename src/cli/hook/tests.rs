//! # Hook tests
//!
//! End-to-end coverage of the reactions bound to timer events: the TOML
//! shapes config.sample.toml documents, and what running one actually
//! does to the machine.

use std::{
    env, fs,
    path::PathBuf,
    process,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{
    cli::hook::TimerHook,
    timer::{TimerCycle, TimerEvent},
};

static NEXT_PATH: AtomicUsize = AtomicUsize::new(0);

/// Deserializes a hook from the TOML shape an account file carries.
fn hook(toml: &str) -> TimerHook {
    toml::from_str(toml).expect("deserialize hook")
}

/// A path unique to this test, under the platform temporary directory.
fn path(name: &str) -> PathBuf {
    let id = NEXT_PATH.fetch_add(1, Ordering::Relaxed);
    let path = env::temp_dir().join(format!("comodoro-hook-{}-{id}-{name}", process::id()));
    let _ = fs::remove_file(&path);
    path
}

/// The redirection is the point: it only works if a shell parses the line
/// rather than the kernel executing it as a program name.
#[test]
fn a_string_command_runs_through_a_shell() {
    let path = path("shell");

    let hook = hook(&format!(r#"command = "echo hooked >> {}""#, path.display()));
    hook.execute(&[]);

    assert_eq!(fs::read_to_string(&path).unwrap().trim(), "hooked");
    let _ = fs::remove_file(path);
}

/// `cmd`, the 1.x spelling, reads as the alias of `command`, as wherever
/// the family configures a command.
#[test]
fn a_cmd_key_runs_as_the_command() {
    let path = path("cmd");

    let hook = hook(&format!(r#"cmd = "echo hooked >> {}""#, path.display()));
    hook.execute(&[]);

    assert_eq!(fs::read_to_string(&path).unwrap().trim(), "hooked");
    let _ = fs::remove_file(path);
}

/// A shell would read the `>` as a redirection. Executed directly, it is
/// one more character in the file name.
#[cfg(unix)]
#[test]
fn an_array_command_executes_with_no_shell() {
    let path = path("no shell > redirection");

    let hook = hook(&format!(r#"command = ["touch", "{}"]"#, path.display()));
    hook.execute(&[]);

    assert!(path.exists(), "{}", path.display());
    let _ = fs::remove_file(path);
}

/// A hook is a reaction, never a step of the timer, so a failure is
/// logged and returned from as if nothing happened. The assertion is the
/// absence of a panic: there is no error left to catch.
#[cfg(unix)]
#[test]
fn a_command_exiting_non_zero_is_inert() {
    hook(r#"command = ["false"]"#).execute(&[]);
}

/// Inert for the same reason a non-zero exit is: a hook never takes the
/// timer down with it.
#[test]
fn a_missing_program_is_inert() {
    hook(r#"command = ["comodoro-hook-does-not-exist"]"#).execute(&[]);
}

/// Sending is left alone on purpose: it needs a notification daemon,
/// which a test machine has no business requiring.
#[cfg(feature = "notify")]
#[test]
fn a_notification_carries_its_summary_and_body() {
    let hook = hook(r#"notify = { summary = "Comodoro", body = "Work started!" }"#);

    let notification = hook.notify.expect("notification hook");

    assert_eq!(notification.summary(), "Comodoro");
    assert_eq!(notification.body(), "Work started!");
}

#[cfg(not(feature = "notify"))]
#[test]
fn a_notification_is_refused_when_no_backend_is_built_in() {
    let toml = r#"notify = { summary = "Comodoro", body = "Work started!" }"#;
    let err = toml::from_str::<TimerHook>(toml).expect_err("deserialize notify hook");

    assert!(err.to_string().contains("notify"), "{err}");
}

#[test]
fn an_empty_command_is_refused() {
    assert!(toml::from_str::<TimerHook>(r#"command = """#).is_err());
    assert!(toml::from_str::<TimerHook>(r#"command = []"#).is_err());
}

#[test]
fn events_name_the_hook_they_fire() {
    let cycle = TimerCycle::new("Long rest", 1800);

    assert_eq!(TimerEvent::Started.hook_name(), "on-timer-start");
    assert_eq!(TimerEvent::Stopped.hook_name(), "on-timer-stop");
    assert_eq!(
        TimerEvent::Began(cycle.clone()).hook_name(),
        "on-long-rest-begin"
    );
    assert_eq!(
        TimerEvent::Ticked(cycle.clone()).hook_name(),
        "on-long-rest-tick"
    );
    assert_eq!(
        TimerEvent::Set(cycle.clone()).hook_name(),
        "on-long-rest-set"
    );
    assert_eq!(
        TimerEvent::Paused(cycle.clone()).hook_name(),
        "on-long-rest-pause"
    );
    assert_eq!(
        TimerEvent::Resumed(cycle.clone()).hook_name(),
        "on-long-rest-resume"
    );
    assert_eq!(TimerEvent::Ended(cycle).hook_name(), "on-long-rest-end");
}

/// One spelling serves both: the variables a notification expands reach
/// a command as environment variables of the same names.
#[cfg(unix)]
#[test]
fn a_command_reads_the_cycle_from_its_environment() {
    let path = path("env");
    let line = format!(
        r#"command = "printf '%s %s' \"$cycleName\" \"$cycleDuration\" > {}""#,
        path.display()
    );
    let event = TimerEvent::Ended(TimerCycle::new("Long rest", 0));

    hook(&line).execute(&event.hook_variables());

    assert_eq!(fs::read_to_string(&path).unwrap(), "Long rest 0");
    let _ = fs::remove_file(path);
}

#[cfg(feature = "notify")]
#[test]
fn a_hook_may_run_a_command_and_show_a_notification() {
    let hook = hook(
        r#"
        command = "true"
        notify = { summary = "$cycleName ended" }
        "#,
    );

    let cycles = [TimerCycle::new("Work", 1500)];
    hook.validate("on-work-end", &cycles).unwrap();
    assert!(hook.command.is_some() && hook.notify.is_some());
}

#[test]
fn a_hook_doing_nothing_is_refused() {
    let cycles = [TimerCycle::new("Work", 1500)];
    let err = hook("").validate("on-work-end", &cycles).unwrap_err();
    assert!(err.to_string().contains("does nothing"), "{err}");
}

#[cfg(feature = "notify")]
#[test]
fn a_default_takes_the_shell_spelling_only() {
    let cycles = [TimerCycle::new("Work", 1500)];

    let toml = r#"notify = { summary = "${cycleName:-none}" }"#;
    hook(toml).validate("on-work-end", &cycles).unwrap();

    let toml = r#"notify = { summary = "${cycleName:none}" }"#;
    let err = toml::from_str::<TimerHook>(toml).unwrap_err();
    assert!(err.to_string().contains("${cycleName:none}"), "{err}");
}

#[test]
fn a_hook_name_lists_the_cycles_when_no_event_fires_it() {
    let cycles = [
        TimerCycle::new("Work", 1500),
        TimerCycle::new("Long rest", 300),
    ];
    let err = hook(r#"command = "true""#)
        .validate("on-work-done", &cycles)
        .unwrap_err();
    let err = err.to_string();

    assert!(err.contains("on-work-done"), "{err}");
    assert!(err.contains("work, long-rest"), "{err}");
    assert!(err.contains("tick"), "{err}");
}
