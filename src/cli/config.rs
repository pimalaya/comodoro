//! # Configuration
//!
//! The TOML configuration of the Comodoro CLI. [`Config`] is the whole
//! document, a table of named accounts.
//!
//! Each [`AccountConfig`] describes the transports the two sides meet on,
//! the timer cycles, and the hooks bound to timer events. Only the cycles
//! are required, and config.sample.toml is the annotated reference.
//!
//! Every table, a cycle included, refuses a key it does not read, rather
//! than ignoring a typo, a key an earlier version read included.
//!
//! What lives here is the document alone: its shape and the way it is
//! read, Comodoro never writing it. What a command runs against is
//! [`Account`], the resolved view built from these types.
//!
//! [`Account`]: crate::cli::account::Account

use std::{collections::HashMap, path::PathBuf};

use pimalaya_config::toml::TomlConfig;
use serde::Deserialize;

use crate::{
    cli::{hook::TimerHook, transport::Transport},
    timer::{TimerCycle, TimerPrecision},
    transport::{TimerAddress, default_socket_path},
};

/// The whole Comodoro configuration.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Config {
    /// The accounts, by name.
    pub accounts: HashMap<String, AccountConfig>,
}

impl TomlConfig for Config {
    type Account = AccountConfig;

    fn project_name() -> &'static str {
        env!("CARGO_PKG_NAME")
    }

    fn sample_url() -> &'static str {
        concat!(
            env!("CARGO_PKG_REPOSITORY"),
            "/blob/master/config.sample.toml"
        )
    }

    fn accounts(&self) -> impl Iterator<Item = (&str, bool)> {
        let accounts = self.accounts.iter();
        accounts.map(|(name, account)| (name.as_str(), account.default))
    }

    fn take_named_account(&mut self, name: &str) -> Option<(String, Self::Account)> {
        self.accounts.remove_entry(name)
    }
}

/// The configuration of a single Comodoro account.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct AccountConfig {
    /// Whether this account is picked when none is given.
    #[serde(default)]
    pub default: bool,
    /// Where the client and the server meet over a local socket.
    ///
    /// Every field has a default, so an account describes a socket
    /// without holding a `socket` table at all: the table adjusts the
    /// address, never whether the transport exists. Holding one declares
    /// the transport to `account list` and `account check`.
    ///
    /// Also spelled `unix-socket`, the name Comodoro 1.x used, so a 1.x
    /// account file loads unchanged.
    #[serde(default, alias = "unix-socket")]
    pub socket: Option<SocketConfig>,
    /// Where the client and the server meet over TCP.
    ///
    /// Defaulted like the socket, so `comodoro server start tcp` opens
    /// a port on an account that says nothing about TCP.
    #[serde(default)]
    pub tcp: Option<TcpConfig>,
    /// The ordered cycles the timer runs through.
    pub cycles: Vec<CycleConfig>,
    /// How many full loops the timer runs, unbounded when absent.
    pub cycles_count: Option<usize>,
    /// How precisely the remaining duration is displayed.
    #[serde(default)]
    pub precision: TimerPrecision,
    /// The hooks to run, by event name.
    #[serde(default)]
    pub hooks: HashMap<String, TimerHook>,
}

impl AccountConfig {
    /// The transport a command talks over when it names none.
    ///
    /// The local socket wins a tie, being the transport that opens no
    /// port: `tcp.default` counts only when `socket.default` is unset.
    pub fn default_transport(&self) -> Transport {
        let socket = self.socket.as_ref().is_some_and(|socket| socket.default);
        let tcp = self.tcp.as_ref().is_some_and(|tcp| tcp.default);

        if tcp && !socket {
            Transport::Tcp
        } else {
            Transport::UnixSocket
        }
    }

    /// The transports the account declares, the socket first: those it
    /// holds a table for, and the default one, which every command
    /// naming none talks over.
    pub fn transports(&self) -> Vec<Transport> {
        let default = self.default_transport();

        let socket = self.socket.is_some() || default == Transport::UnixSocket;
        let tcp = self.tcp.is_some() || default == Transport::Tcp;

        let declared = [(Transport::UnixSocket, socket), (Transport::Tcp, tcp)];
        let declared = declared.into_iter().filter(|(_, declared)| *declared);
        declared.map(|(transport, _)| transport).collect()
    }
}

/// One cycle of an account: a name and a duration in seconds.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct CycleConfig {
    /// The name of the cycle, which its hook names are built from.
    pub name: String,
    /// How long the cycle lasts, in seconds.
    pub duration: usize,
}

impl From<CycleConfig> for TimerCycle {
    fn from(cycle: CycleConfig) -> Self {
        TimerCycle::new(cycle.name, cycle.duration)
    }
}

/// Where a client and a server meet over a local socket.
#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct SocketConfig {
    /// Whether a command talks over this transport when it names none.
    #[serde(default)]
    pub default: bool,
    /// The socket path to bind and connect to.
    ///
    /// Defaults to comodoro.sock inside `$XDG_RUNTIME_DIR`, or inside the
    /// platform temporary directory when that variable is unset.
    #[serde(default = "default_socket_path")]
    pub path: PathBuf,
}

impl SocketConfig {
    /// The address this configuration points at.
    pub fn address(&self) -> TimerAddress {
        TimerAddress::UnixSocket(self.path.clone())
    }
}

impl Default for SocketConfig {
    fn default() -> Self {
        Self {
            default: false,
            path: default_socket_path(),
        }
    }
}

/// Where a client and a server meet over TCP.
///
/// The listener it describes is unauthenticated, so whoever reaches the
/// port drives the timer. Hence the loopback host, the one default an
/// account should have to overrule deliberately.
#[derive(Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct TcpConfig {
    /// Whether a command talks over this transport when it names none.
    #[serde(default)]
    pub default: bool,
    /// The host to bind and connect to.
    #[serde(default = "localhost")]
    pub host: String,
    /// The port to bind and connect to.
    #[serde(default = "tcp_port")]
    pub port: u16,
}

impl TcpConfig {
    /// The address this configuration points at.
    pub fn address(&self) -> TimerAddress {
        TimerAddress::Tcp {
            host: self.host.clone(),
            port: self.port,
        }
    }
}

impl Default for TcpConfig {
    fn default() -> Self {
        Self {
            default: false,
            host: localhost(),
            port: TCP_PORT,
        }
    }
}

/// The host a `tcp` table falls back to.
///
/// The listener is unauthenticated, so an account naming no host stays
/// where only this machine can reach it.
pub const LOCALHOST: &str = "127.0.0.1";

/// The port a `tcp` table falls back to.
pub const TCP_PORT: u16 = 9999;

fn localhost() -> String {
    LOCALHOST.to_string()
}

fn tcp_port() -> u16 {
    TCP_PORT
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs, path::PathBuf, slice};

    use tempfile::TempDir;

    use crate::cli::{account::Account, take_account};

    /// Checks config.sample.toml is refused as written for its missing
    /// `cycles` alone, then loads once per rank fully uncommented, each
    /// preset taking its rank, `example` the default each time.
    ///
    /// A build without notifications drops the `notify` lines, which it
    /// refuses as it should.
    #[test]
    fn the_sample_configuration_parses_with_each_variant() {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.sample.toml");
        let sample = fs::read_to_string(&path).unwrap();
        let headers: Vec<_> = sample.lines().filter(|l| l.starts_with('[')).collect();
        assert_eq!(headers, ["[accounts.example]"]);

        let dir = TempDir::new().unwrap();

        let err = load_sample(&dir, &sample).unwrap_err();
        let err = format!("{err:?}");
        assert!(err.contains("missing field `cycles`"), "as written: {err}");

        let paragraphs = sample_paragraphs(&sample);
        let mut presets = Vec::new();

        for rank in 1..=sample_ranks(&paragraphs) {
            let mut text = sample_variant(&paragraphs, rank);

            if cfg!(not(feature = "notify")) {
                let lines = text.lines().filter(|line| !line.contains(".notify."));
                text = lines.collect::<Vec<_>>().join("\n");
            }

            let account = load_sample(&dir, &text).unwrap_or_else(|err| {
                panic!("config.sample.toml, variant {rank}: {err:?}\n{text}")
            });

            assert_eq!(account.name, "example", "variant {rank}");
            presets.push(account.schedule.cycles[0].duration);
        }

        assert_eq!(presets, [1500, 3120, 6720]);
    }

    /// One line of config.sample.toml.
    struct SampleLine<'a> {
        /// The line as written.
        line: &'a str,
        /// The code before the comment, one `#` off when commented.
        code: &'a str,
        /// Whether the code is commented.
        commented: bool,
        /// The field the code sets, a secret's `raw` and `command` being
        /// one.
        field: Option<&'a str>,
        /// The other values the comment lists in brackets.
        values: Vec<&'a str>,
    }

    impl<'a> SampleLine<'a> {
        fn parse(line: &'a str) -> Self {
            let (code, comment) = line.split_once('\t').unwrap_or((line, ""));
            let commented = code.starts_with('#');
            let code = code.strip_prefix('#').unwrap_or(code);

            let field = code
                .split_once(" = ")
                .map(|(key, _)| key)
                .filter(|key| {
                    key.chars()
                        .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
                })
                .map(|key| {
                    let secret = key.strip_suffix(".raw").or(key.strip_suffix(".command"));
                    secret.unwrap_or(key)
                });

            let values = comment
                .trim_end()
                .strip_suffix(']')
                .and_then(|comment| comment.rsplit_once('['))
                .map(|(_, values)| values.split(", ").collect())
                .unwrap_or_default();

            Self {
                line,
                code,
                commented,
                field,
                values,
            }
        }

        /// The first two segments of the field, the table a replacing
        /// paragraph shares with the one above it (`hooks.on-work-end`).
        fn table(&self) -> Option<&'a str> {
            let field = self.field?;
            let end = field
                .match_indices('.')
                .nth(1)
                .map_or(field.len(), |(i, _)| i);
            Some(&field[..end])
        }

        /// The values the line offers: its code as written, then each
        /// bracketed value in its place, typed like the written one.
        fn options(&self) -> Vec<String> {
            let (key, value) = self.code.split_once(" = ").unwrap();
            let mut options = vec![self.code.to_owned()];

            for other in &self.values {
                if value.starts_with('"') {
                    options.push(format!("{key} = \"{other}\""));
                } else {
                    options.push(format!("{key} = {other}"));
                }
            }

            options
        }
    }

    /// Writes `text` to config.toml in `dir` and takes its default account
    /// through the loader `-c` uses.
    fn load_sample(dir: &TempDir, text: &str) -> anyhow::Result<Account> {
        let path = dir.path().join("config.toml");
        fs::write(&path, text).unwrap();
        take_account(slice::from_ref(&path), None)
    }

    fn sample_paragraphs(sample: &str) -> Vec<Vec<SampleLine<'_>>> {
        let paragraphs = sample.split("\n\n");
        let paragraphs = paragraphs.map(|p| p.lines().map(SampleLine::parse).collect());
        paragraphs.collect()
    }

    /// Whether `below`, all commented, replaces `above`, setting keys under
    /// a table one of its lines sets.
    ///
    /// A paragraph setting a field `above` sets, as a preset's `cycles`,
    /// replaces nothing: the field's values take their ranks instead.
    fn sample_replaces(above: &[SampleLine], below: &[SampleLine]) -> bool {
        let mut code = below.iter().filter(|line| line.field.is_some());
        let Some(first) = code.next() else {
            return false;
        };

        if !first.commented || code.any(|line| !line.commented) {
            return false;
        }

        if above.iter().any(|line| line.field == first.field) {
            return false;
        }

        above
            .iter()
            .filter_map(SampleLine::table)
            .any(|t| Some(t) == first.table())
    }

    /// The values each field takes in turn, the lines setting it in their
    /// order, each followed by its bracketed values.
    fn sample_options<'a>(lines: &[&'a SampleLine<'a>]) -> HashMap<&'a str, Vec<String>> {
        let mut options: HashMap<&str, Vec<String>> = HashMap::new();

        for line in lines {
            if let Some(field) = line.field {
                options.entry(field).or_default().extend(line.options());
            }
        }

        options
    }

    /// How many variants give every value its turn: the most values of
    /// one field, two when a paragraph replaces another.
    fn sample_ranks(paragraphs: &[Vec<SampleLine>]) -> usize {
        let lines: Vec<&SampleLine> = paragraphs.iter().flatten().collect();
        let options = sample_options(&lines);
        let values = options.values().map(Vec::len).max().unwrap_or(0);

        let replaces = paragraphs.windows(2).any(|w| sample_replaces(&w[0], &w[1]));

        if replaces { values.max(2) } else { values }
    }

    /// The sample at `rank`, every line uncommented, each field set to its
    /// value of that rank (the last when it has fewer).
    ///
    /// A replacing paragraph stays out at rank 1 and replaces the one
    /// above it from rank 2.
    fn sample_variant(paragraphs: &[Vec<SampleLine>], rank: usize) -> String {
        let mut dropped = vec![false; paragraphs.len()];

        for i in 1..paragraphs.len() {
            if sample_replaces(&paragraphs[i - 1], &paragraphs[i]) {
                let i = if rank == 1 { i } else { i - 1 };
                dropped[i] = true;
            }
        }

        let kept = paragraphs
            .iter()
            .zip(&dropped)
            .filter(|(_, dropped)| !**dropped);
        let lines: Vec<&SampleLine> = kept.flat_map(|(lines, _)| lines).collect();

        let mut options = sample_options(&lines);
        let mut text = Vec::with_capacity(lines.len());

        for line in lines {
            let Some(field) = line.field else {
                text.push(line.line.to_owned());
                continue;
            };

            if let Some(values) = options.remove(field) {
                text.push(values[rank.min(values.len()) - 1].clone());
            }
        }

        text.join("\n")
    }
}
