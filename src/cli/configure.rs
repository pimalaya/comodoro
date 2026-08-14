//! Command generating a timer account.
//!
//! The wizard generates, it never edits: it asks for a cycle preset and
//! for the endpoints to serve the timer over, names the account after
//! the preset, then hands the resulting `[accounts.<name>]` table back
//! as a file to create, a block to append, or a document on stdout.
//! Everything it does not cover, meaning custom cycles, a socket path
//! or a TCP endpoint other than the default one, the display precision
//! and the hooks, is written by hand against the documented sample.
//!
//! It runs from `comodoro configure`, and from the offer a bare
//! `comodoro` or a command needing an account raises when it finds no
//! configuration file. That offer is the only place the wizard
//! introduces itself, with a welcome naming the file that is missing:
//! the command asked for by name goes straight to the prompts.
//!
//! Appending is a plain text append rather than a re-serialization of
//! the whole file, so comments, ordering and hand-written formatting
//! come out untouched. Two rules guard it: the account name must be
//! free, since two `[accounts.<name>]` tables make the whole document
//! fail to parse, and the generated account claims the default only
//! when no other account does, since two defaults resolve to whichever
//! one the account map hands over first.

use core::fmt;

use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

use std::{
    collections::HashMap,
    eprintln,
    fs::{self, OpenOptions},
    io::{IsTerminal, Write, stdin, stdout},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use clap::Parser;
use pimalaya_cli::{printer::Printer, prompt};
use pimalaya_config::toml::TomlConfig;
use serde::Serialize;

use crate::{
    cli::config::{AccountConfig, CONFIG_SAMPLE_URL, Config, SocketConfig, TcpConfig},
    timer::{TimerCycle, TimerPrecision},
};

/// Configure a timer account.
///
/// This command asks for one of the documented cycle presets and for
/// the endpoints to serve the timer over, then saves the resulting
/// account to the configuration file, appends it to the one already
/// there, or prints it for you to place by hand. Anything the presets
/// do not cover is written by hand.
#[derive(Debug, Parser)]
pub struct ConfigureCommand;

impl ConfigureCommand {
    /// Asks the two questions, then saves, appends or prints the
    /// account.
    ///
    /// No welcome: whoever typed the command knows what it does. The
    /// banner belongs to the offer a missing
    /// configuration raises, which is where the wizard meets someone
    /// who did not ask for it. The account name is not asked either,
    /// since it is only the TOML table key, and renaming it is one edit
    /// in the file the wizard just wrote.
    ///
    /// A redirected stdout (`comodoro configure > config.toml`) and the
    /// JSON output both stay non-interactive: the document goes to
    /// stdout and no file is touched. The prompts render on stderr, so
    /// they stay out of the redirected document.
    pub fn execute(self, printer: &mut impl Printer, config_paths: &[PathBuf]) -> Result<()> {
        if !stdin().is_terminal() {
            bail!(
                "Configuring needs a terminal to prompt on, \
		 write the configuration by hand instead: {CONFIG_SAMPLE_URL}"
            );
        }

        let preset = prompt::item(
            "Timer style:",
            TimerPreset::ALL,
            Some(TimerPreset::Pomodoro),
        )?;

        let path = Config::target_path(config_paths)?;
        let existing = ExistingConfig::read(&path)?;
        let name = account_name(preset, existing.as_ref());

        // NOTE: a second `default = true` would make the account every
        // command picks depend on map ordering, so the generated one
        // claims the default only when no other account does.
        let default = !existing.as_ref().is_some_and(|config| config.has_default);
        let account = preset.account(default);

        let config = GeneratedConfig {
            document: account.render(&name),
            name,
            default,
        };

        if printer.is_json() || !stdout().is_terminal() {
            return printer.out(config);
        }

        match existing {
            Some(_) => append_or_print(printer, &path, config),
            None => save_or_print(printer, &path, config),
        }
    }
}

/// The cycle presets the wizard offers, the three documented in the
/// sample configuration.
///
/// A timer that follows none of them is written by hand: prompting for
/// an arbitrary list of named durations would be a worse text editor
/// than the one the user already has.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TimerPreset {
    /// Three 25 minute rounds of work, 5 minutes of rest between them,
    /// then a 30 minute rest.
    Pomodoro,
    /// 52 minutes of work, 17 minutes of rest.
    FiftyTwoSeventeen,
    /// 112 minutes of work, 26 minutes of rest.
    HundredTwelveTwentySix,
}

impl TimerPreset {
    /// Every preset, in the order the prompt lists them.
    const ALL: [Self; 3] = [
        Self::Pomodoro,
        Self::FiftyTwoSeventeen,
        Self::HundredTwelveTwentySix,
    ];

    /// The account this preset generates.
    ///
    /// Nothing but the cycles: every transport field has a default, so
    /// an account that adjusts none of them writes none of them, and
    /// which transport a server binds is what `server start` decides.
    fn account(self, default: bool) -> AccountConfig {
        AccountConfig {
            default,
            socket: SocketConfig::default(),
            tcp: TcpConfig::default(),
            cycles: self.cycles(),
            cycles_count: None,
            precision: TimerPrecision::default(),
            hooks: HashMap::new(),
        }
    }

    /// The account name proposed for this preset.
    fn account_name(self) -> &'static str {
        match self {
            Self::Pomodoro => "pomodoro",
            Self::FiftyTwoSeventeen => "52-17",
            Self::HundredTwelveTwentySix => "112-26",
        }
    }

    /// The ordered cycles this preset runs through.
    fn cycles(self) -> Vec<TimerCycle> {
        match self {
            Self::Pomodoro => vec![
                TimerCycle::new("Work", 1500),
                TimerCycle::new("Rest", 300),
                TimerCycle::new("Work", 1500),
                TimerCycle::new("Rest", 300),
                TimerCycle::new("Work", 1500),
                TimerCycle::new("Long rest", 1800),
            ],
            Self::FiftyTwoSeventeen => {
                vec![TimerCycle::new("Work", 3120), TimerCycle::new("Rest", 1020)]
            }
            Self::HundredTwelveTwentySix => {
                vec![TimerCycle::new("Work", 6720), TimerCycle::new("Rest", 1560)]
            }
        }
    }
}

impl fmt::Display for TimerPreset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pomodoro => write!(f, "Pomodoro: 3 x (25 min of work, 5 of rest), 30 of rest"),
            Self::FiftyTwoSeventeen => write!(f, "52/17: 52 min of work, 17 of rest"),
            Self::HundredTwelveTwentySix => write!(f, "112/26: 112 min of work, 26 of rest"),
        }
    }
}

/// What a configuration file already on disk constrains in the
/// generated account: the names it takes, and whether one of its
/// accounts already claims the default.
struct ExistingConfig {
    names: Vec<String>,
    has_default: bool,
}

impl ExistingConfig {
    /// Reads the configuration at the given path, or `None` when no
    /// file is there.
    ///
    /// A file that fails to parse is an error rather than a `None`:
    /// appending to a broken document would bury the actual problem
    /// under a second one.
    fn read(path: &Path) -> Result<Option<Self>> {
        if !path.exists() {
            return Ok(None);
        }

        let config = Config::from_paths(&[path.to_path_buf()])
            .with_context(|| format!("Read the configuration at {}", path.display()))?;

        Ok(Some(Self {
            names: config.accounts.keys().cloned().collect(),
            has_default: config.accounts.values().any(|account| account.default),
        }))
    }
}

/// The generated account, as the printer takes it.
#[derive(Serialize)]
struct GeneratedConfig {
    /// The account name, which is the `[accounts.<name>]` table key.
    name: String,
    /// Whether the account claims the default.
    default: bool,
    /// The rendered TOML document.
    document: String,
}

impl fmt::Display for GeneratedConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // NOTE: the trailing newline terminates the document, and it is
        // also what flushes the line-buffered stdout.
        writeln!(f, "{}", self.document.trim_end())
    }
}

/// Frames Comodoro, names the configuration file that is missing, and
/// points at the sample for everything the wizard does not cover.
///
/// Printed before the offer a bare `comodoro` or a command needing an
/// account raises when it finds no configuration, so the wizard
/// introduces itself to someone who did not ask for it. `configure`
/// skips it, since it was asked for by name.
///
/// On stderr, so a redirected stdout holds the document alone.
pub(crate) fn print_welcome(path: &Path) {
    eprintln!();
    eprintln!("Welcome to Comodoro, the CLI to manage timers.");
    eprintln!();
    eprintln!("Comodoro runs a shared timer: one server owns it, any number of clients");
    eprintln!("drive it and watch it. It needs one account to know which cycles to run,");
    eprintln!("and no configuration file was found at:");
    eprintln!();
    eprintln!("  {}", path.display());
    eprintln!();
    eprintln!("The wizard sets up one account from a documented preset. Custom cycles,");
    eprintln!("the socket path, the display precision and the hooks are written by hand,");
    eprintln!("and every field is documented at:");
    eprintln!();
    eprintln!("  {CONFIG_SAMPLE_URL}");
    eprintln!();
    eprintln!("At anytime, you can create a new account with the command:");
    eprintln!();
    eprintln!("  comodoro configure");
    eprintln!();
}

/// The name the preset proposes, suffixed until the configuration does
/// not already hold it.
///
/// Not prompted: the name is only the TOML table key, and whoever wants
/// another one renames it in the file. It still has to be free, since a
/// second `[accounts.<name>]` table makes the whole document fail to
/// parse, taking the accounts that used to work down with it.
fn account_name(preset: TimerPreset, existing: Option<&ExistingConfig>) -> String {
    let taken = existing
        .map(|config| config.names.as_slice())
        .unwrap_or(&[]);
    let base = preset.account_name();

    if !taken.iter().any(|name| name == base) {
        return base.to_string();
    }

    let mut suffix = 2;

    loop {
        let name = format!("{base}-{suffix}");

        if !taken.contains(&name) {
            return name;
        }

        suffix += 1;
    }
}

/// Offers to write the generated account to a configuration file that
/// does not exist yet, printing it instead when the offer is declined.
fn save_or_print(printer: &mut impl Printer, path: &Path, config: GeneratedConfig) -> Result<()> {
    let prompt = format!("Save this account to {}?", path.display());

    if !prompt::bool(prompt, true)? {
        return printer.out(config);
    }

    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .with_context(|| format!("Create the config directory {}", parent.display()))?;
    }

    fs::write(path, config.to_string())
        .with_context(|| format!("Write the config file {}", path.display()))?;

    print_saved(path, &config);

    Ok(())
}

/// Offers to append the generated account to the configuration file
/// already there, printing it instead when the offer is declined.
fn append_or_print(printer: &mut impl Printer, path: &Path, config: GeneratedConfig) -> Result<()> {
    let prompt = format!("Append account `{}` to {}?", config.name, path.display());

    if !prompt::bool(prompt, true)? {
        return printer.out(config);
    }

    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .with_context(|| format!("Open the config file {}", path.display()))?;

    // NOTE: appending text keeps every comment and every hand-written
    // line of the file as they are, which parsing and re-serializing
    // the whole document would not. The leading newline separates the
    // two tables, and terminates the last line when the file ends
    // without one.
    write!(file, "\n{config}")
        .with_context(|| format!("Append to the config file {}", path.display()))?;

    print_saved(path, &config);

    Ok(())
}

/// Tells where the account landed, under which name, and what to run
/// next.
///
/// The name matters here because it was never asked for: an account
/// that did not claim the default is only reachable through `-a`.
fn print_saved(path: &Path, config: &GeneratedConfig) {
    let name = &config.name;

    eprintln!();
    eprintln!("Account `{name}` saved to {}.", path.display());

    if !config.default {
        eprintln!("Another account holds the default, so name this one with `-a {name}`.");
    }

    eprintln!("Run `comodoro server start` to own the timer, then `comodoro start` to");
    eprintln!("start it and `comodoro watch` to follow it.");
}

#[cfg(test)]
mod tests {
    use std::{
        env, fs,
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use super::*;

    static NEXT_CONFIG: AtomicUsize = AtomicUsize::new(0);

    /// A path in the temporary directory no other test writes to.
    fn config_path() -> PathBuf {
        let id = NEXT_CONFIG.fetch_add(1, Ordering::Relaxed);
        env::temp_dir().join(format!("comodoro-configure-{id}.toml"))
    }

    #[test]
    fn a_generated_account_parses_back() {
        let document = TimerPreset::Pomodoro.account(true).render("pomodoro");
        let config: Config = toml::from_str(&document).expect("parse the generated config");
        let account = &config.accounts["pomodoro"];

        assert_eq!(config.accounts.len(), 1);
        assert_eq!(account.cycles, TimerPreset::Pomodoro.cycles());
        assert!(account.default);

        // Every transport field has a default, so the generated account
        // holds its cycles and nothing else: no address is frozen into
        // the file, and `server start` still reaches either transport.
        assert!(!document.contains("socket"));
        assert!(!document.contains("tcp"));
        assert!(!document.contains("precision"));
        assert!(!document.contains("hooks"));
        assert_eq!(account.socket, SocketConfig::default());
        assert_eq!(account.tcp, TcpConfig::default());

        // One cycle per line, in the order and the shape the sample
        // configuration documents.
        assert!(document.contains("  { name = \"Work\", duration = 1500 },\n"));
        assert_eq!(document.lines().count(), 10);
    }

    #[test]
    fn an_appended_account_keeps_the_existing_one() {
        let path = config_path();

        // No trailing newline, the shape an appended block has to
        // survive without merging into the last line.
        fs::write(
            &path,
            "# my timers\n[accounts.work]\ndefault = true\ncycles = [{ name = \"Work\", duration = 3120 }]",
        )
        .expect("write the existing config");

        let existing = ExistingConfig::read(&path)
            .expect("read the existing config")
            .expect("an existing config");

        assert_eq!(existing.names, ["work"]);
        assert!(existing.has_default);

        let document = TimerPreset::Pomodoro
            .account(!existing.has_default)
            .render("pomodoro");
        let mut file = OpenOptions::new().append(true).open(&path).expect("open");
        write!(file, "\n{document}").expect("append the generated account");
        drop(file);

        let content = fs::read_to_string(&path).expect("read back");
        let config: Config = toml::from_str(&content).expect("parse the appended config");

        assert_eq!(config.accounts.len(), 2);
        assert_eq!(config.accounts["work"].cycles.len(), 1);
        assert_eq!(
            config.accounts["pomodoro"].cycles,
            TimerPreset::Pomodoro.cycles()
        );

        // Exactly one default, and the comment is still there.
        let defaults = config
            .accounts
            .values()
            .filter(|account| account.default)
            .count();
        assert_eq!(defaults, 1);
        assert!(config.accounts["work"].default);
        assert!(content.starts_with("# my timers"));

        fs::remove_file(&path).expect("remove the config");
    }

    #[test]
    fn a_taken_name_gets_a_suffix() {
        let existing = ExistingConfig {
            names: vec!["pomodoro".to_string(), "pomodoro-2".to_string()],
            has_default: true,
        };

        assert_eq!(account_name(TimerPreset::Pomodoro, None), "pomodoro");
        assert_eq!(
            account_name(TimerPreset::Pomodoro, Some(&existing)),
            "pomodoro-3"
        );
        assert_eq!(
            account_name(TimerPreset::FiftyTwoSeventeen, Some(&existing)),
            "52-17"
        );
    }

    #[test]
    fn a_missing_configuration_constrains_nothing() {
        let existing = ExistingConfig::read(&config_path()).expect("read a missing config");

        assert!(existing.is_none());
    }
}
