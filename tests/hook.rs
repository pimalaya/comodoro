//! End-to-end coverage of the reactions bound to timer events: the TOML
//! shapes config.sample.toml documents, and what running one actually
//! does to the machine.

use std::{
    env, fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

use comodoro::{
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
    let path = env::temp_dir().join(format!("comodoro-hook-{}-{id}-{name}", std::process::id()));
    let _ = fs::remove_file(&path);
    path
}

#[test]
fn a_string_command_runs_through_a_shell() {
    let path = path("shell");

    // The redirection is the point: it only works if a shell parses the
    // line rather than the kernel executing it as a program name.
    let mut hook = hook(&format!(r#"command = "echo hooked >> {}""#, path.display()));
    hook.execute().unwrap();

    assert_eq!(fs::read_to_string(&path).unwrap().trim(), "hooked");
    let _ = fs::remove_file(path);
}

#[cfg(unix)]
#[test]
fn an_array_command_executes_with_no_shell() {
    let path = path("no shell > redirection");

    // A shell would read the `>` as a redirection. Executed directly,
    // it is one more character in the file name.
    let mut hook = hook(&format!(r#"command = ["touch", "{}"]"#, path.display()));
    hook.execute().unwrap();

    assert!(path.exists(), "{}", path.display());
    let _ = fs::remove_file(path);
}

#[cfg(unix)]
#[test]
fn a_command_exiting_non_zero_is_not_an_error() {
    let mut hook = hook(r#"command = ["false"]"#);

    // A hook is a reaction, not a step of the timer: a failing one is
    // logged and the timer carries on.
    assert!(hook.execute().is_ok());
}

#[test]
fn a_missing_program_is_an_error() {
    let mut hook = hook(r#"command = ["comodoro-hook-does-not-exist"]"#);

    let err = hook.execute().unwrap_err().to_string();
    assert_eq!(err, "Run hook command error");
}

#[test]
fn a_notification_carries_its_summary_and_body() {
    // Sending is left alone on purpose: it needs a notification daemon,
    // which a test machine has no business requiring.
    let hook = hook(r#"notify = { summary = "Comodoro", body = "Work started!" }"#);

    let TimerHook::Notify(notification) = hook else {
        panic!("expected a notification hook");
    };

    assert_eq!(notification.summary, "Comodoro");
    assert_eq!(notification.body, "Work started!");
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
        TimerEvent::Running(cycle.clone()).hook_name(),
        "on-long-rest-running"
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
