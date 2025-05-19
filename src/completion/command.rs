use std::{fmt, fs, path::PathBuf};

use anyhow::Result;
use clap::{value_parser, CommandFactory, Parser};
use clap_complete::Shell;
use log::debug;
use pimalaya_tui::terminal::cli::{
    arg::path_parser,
    printer::{Message, Printer},
};
use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::cli::Cli;

/// Generate completion script for the give shell(s) to the given
/// directory.
///
/// This command allows you to generate completion script for a given
/// shell. The script is printed to the standard output. If you want
/// to write it to a file, just use unix redirection.
#[derive(Debug, Parser)]
pub struct CompletionGenerateCommand {
    /// Shell for which completion script should be generated for.
    #[arg(value_parser = value_parser!(Shell))]
    pub shells: Vec<Shell>,

    /// Save completion script to the given directory.
    #[arg(short, long, value_name = "PATH", value_parser = path_parser, default_value = "./")]
    pub dir: PathBuf,
}

impl CompletionGenerateCommand {
    pub fn execute(self, printer: &mut impl Printer) -> Result<()> {
        let dir = self.dir.canonicalize().unwrap_or(self.dir);
        fs::create_dir_all(&dir)?;

        let mut cmd = Cli::command();
        let cmd_name = cmd.get_name().to_string();
        let mut completions = Vec::with_capacity(5);

        for shell in self.shells {
            let path = clap_complete::generate_to(shell.clone(), &mut cmd, &cmd_name, &dir)?;
            let path = path.canonicalize().unwrap_or(path);

            debug!("generated {shell} completion script at {}", path.display());
            printer.log(format!(
                "Generated {shell} completion script at {}\n",
                path.display()
            ))?;

            completions.push(Script { shell, path })
        }

        printer.out(Completions {
            dir,
            scripts: completions,
        })
    }
}

/// Defines a struct-wrapper to provide a JSON output.
#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct Completions {
    dir: PathBuf,
    scripts: Vec<Script>,
}

impl fmt::Display for Completions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.scripts.len();
        let msg = Message::new(format!(
            "{n} completion script(s) successfully generated in {}",
            &self.dir.display()
        ));

        write!(f, "{msg}")
    }
}

/// Defines a struct-wrapper to provide a JSON output.
struct Script {
    pub shell: Shell,
    pub path: PathBuf,
}

impl Serialize for Script {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Script", 2)?;
        s.serialize_field("shell", &self.shell.to_string())?;
        s.serialize_field("path", &self.path)?;
        s.end()
    }
}
