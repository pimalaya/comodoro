use std::{fmt, fs, path::PathBuf};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_mangen::Man;
use pimalaya_tui::terminal::cli::{
    arg::path_parser,
    printer::{Message, Printer},
};
use serde::{ser::SerializeStruct, Serialize, Serializer};

use crate::cli::Cli;

/// Generate manual pages to the given directory.
///
/// This command allows you to generate manual pages (following the
/// man page format) to the given directory. If the directory does not
/// exist, it will be created. Any existing man pages will be
/// overriden.
#[derive(Debug, Parser)]
pub struct ManualGenerateCommand {
    /// Directory where man files should be generated in.
    #[arg(value_parser = path_parser, default_value = "./")]
    pub dir: PathBuf,
}

impl ManualGenerateCommand {
    pub fn execute(self, printer: &mut impl Printer) -> Result<()> {
        let dir = self.dir.canonicalize().unwrap_or(self.dir);
        fs::create_dir_all(&dir)?;

        let cmd = Cli::command();
        let cmd_name = cmd.get_name().to_string();
        let subcmds: Vec<_> = cmd.get_subcommands().cloned().collect();
        let mut buffer = Vec::new();
        let mut pages = Vec::new();

        buffer.clear();
        Man::new(cmd).render(&mut buffer)?;
        let path = dir.join(format!("{cmd_name}.1"));
        fs::write(&path, &buffer)?;
        printer.log(format!("Generated man page {}\n", path.display()))?;
        pages.push(Page {
            command: cmd_name.clone(),
            path,
        });

        for cmd in subcmds {
            let subcmd_name = cmd.get_name().to_string();
            buffer.clear();
            Man::new(cmd).render(&mut buffer)?;
            let path = dir.join(format!("{cmd_name}-{subcmd_name}.1"));
            printer.log(format!("Generated man page {}\n", path.display()))?;
            fs::write(&path, &buffer)?;
            pages.push(Page {
                command: subcmd_name,
                path,
            });
        }

        printer.out(Manuals { dir, pages })
    }
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
struct Manuals {
    dir: PathBuf,
    pages: Vec<Page>,
}

impl fmt::Display for Manuals {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.pages.len();
        let msg = Message::new(format!(
            "{n} man page(s) successfully generated in {}",
            &self.dir.display()
        ));

        write!(f, "{msg}")
    }
}

struct Page {
    pub command: String,
    pub path: PathBuf,
}

impl Serialize for Page {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("Page", 2)?;
        s.serialize_field("command", &self.command)?;
        s.serialize_field("path", &self.path)?;
        s.end()
    }
}
