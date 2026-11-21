//! # MCP command
//!
//! The `mcp` subcommand, a Model Context Protocol server over stdin and
//! stdout exposing the timer commands as tools, one per command.
//!
//! pimalaya-cli's [`mcp`] module answers the protocol and checks each
//! call against its tool's input schema; this one lists the tools and
//! runs the calls. A tool's input is its clap command's [`Grammar`], and
//! a call turns back into an argv parsed by that same command, so the
//! CLI, the [`crate::cli::repl`] and MCP share one grammar. Commands run
//! against the account the server resolved once, each connecting to the
//! timer server anew, and print into a [`CapturePrinter`], whose one JSON
//! value becomes the call's structured result.
//!
//! [`mcp`]: pimalaya_cli::mcp

use anyhow::{Result, anyhow};
use clap::{CommandFactory, Parser};
use pimalaya_cli::{
    mcp::{ServerInfo, ToolCall, ToolResult, grammar::Grammar, serve, tool_output_schema},
    printer::CapturePrinter,
};
use serde_json::{Value, json};

use crate::cli::{Cli, Command, Session, error::ErrorCode, json_schema};

/// What the server tells the agent at initialization.
const INFO: ServerInfo = ServerInfo {
    name: "comodoro",
    version: env!("CARGO_PKG_VERSION"),
    instructions: Some(INSTRUCTIONS),
};

/// How to use the tools, for the client to hand its model.
const INSTRUCTIONS: &str = "Reads, and when the server allows it drives, \
the timer of one account. The timer belongs to a `comodoro server start` \
process running beside this one: get reads it, its state (running, paused \
or stopped), the current cycle with the seconds it has left, and when it \
was started. start, pause, resume, set (the seconds left in the current \
cycle) and stop change it and answer the timer as it is after the change; \
they are listed only when the server runs with --allow-write. A failed \
call answers isError with a comodoro-error object, its `code` naming the \
failure when it has one: server-unreachable means no timer server runs.";

/// The commands exposed as tools, with what each may do.
///
/// A command missing here is not exposed: `watch` prints until
/// interrupted, `server start` runs until interrupted, the `account`
/// commands describe the configuration rather than the timer, as across
/// the family, and the meta commands write files or print documentation.
/// `stop` alone is destructive: it drops the elapsed time for good.
const TOOLS: &[McpTool] = &[
    McpTool::read("get"),
    McpTool::write("start", false),
    McpTool::write("pause", false),
    McpTool::write("resume", false),
    McpTool::write("set", false),
    McpTool::write("stop", true),
];

/// Start a Model Context Protocol server over stdio.
///
/// Exposes the timer commands of one account as tools, for an agent to
/// read and drive the timer a `comodoro server start` process owns. Each
/// call connects to that server anew. Without flags only `get` is
/// listed.
#[derive(Debug, Parser)]
pub struct McpCommand {
    /// List the tools that change the timer too.
    ///
    /// Adds `start`, `pause`, `resume`, `set` and `stop`.
    #[arg(long)]
    pub allow_write: bool,
}

impl McpCommand {
    /// Serves JSON-RPC requests off stdin until EOF, answering each on
    /// stdout, the calls running over `session`.
    pub fn execute(self, session: &mut Session) -> Result<()> {
        let tools = McpToolSpec::all(self.allow_write);
        let definitions: Vec<Value> = tools.iter().map(|tool| tool.definition.clone()).collect();

        let mut server = McpServer { session, tools };

        serve(&INFO, &definitions, |call| server.call(call))
    }
}

/// A command as a tool, and what it may do.
struct McpTool {
    name: &'static str,
    write: bool,
    destructive: bool,
}

impl McpTool {
    const fn read(name: &'static str) -> Self {
        Self {
            name,
            write: false,
            destructive: false,
        }
    }

    const fn write(name: &'static str, destructive: bool) -> Self {
        Self {
            name,
            write: true,
            destructive,
        }
    }
}

/// A listed tool: its definition and the grammar a call maps onto.
struct McpToolSpec {
    grammar: Grammar,
    definition: Value,
}

impl McpToolSpec {
    /// The tools the gate allows.
    ///
    /// None is open-world: no timer control reaches anyone beyond the
    /// user, the one meaning `openWorldHint` carries across the family.
    fn all(allow_write: bool) -> Vec<Self> {
        let mut cli = Cli::command();
        cli.build();

        let schemas = json_schema::registry().schemas;
        let mut tools = Vec::new();

        for tool in TOOLS {
            if tool.write && !allow_write {
                continue;
            }

            let Some(cmd) = cli.find_subcommand(tool.name) else {
                continue;
            };

            let grammar = Grammar::new(&[tool.name], cmd);

            let description = cmd
                .get_long_about()
                .or(cmd.get_about())
                .map(ToString::to_string)
                .unwrap_or_default();

            let mut definition = json!({
                "name": tool.name,
                "description": description,
                "inputSchema": grammar.input_schema(),
                "annotations": {
                    "readOnlyHint": !tool.write,
                    "destructiveHint": tool.destructive,
                    "openWorldHint": false,
                },
            });

            let schema = schemas.get(&format!("comodoro-{}", tool.name));

            if let Some(schema) = schema.and_then(tool_output_schema) {
                definition["outputSchema"] = schema.clone();
            }

            tools.push(Self {
                grammar,
                definition,
            });
        }

        tools
    }
}

/// The server: the tools it lists and the session holding the account
/// they run against.
struct McpServer<'a, 'b> {
    session: &'a mut Session<'b>,
    tools: Vec<McpToolSpec>,
}

impl McpServer<'_, '_> {
    /// Runs a tool call, listed and holding only the arguments its tool
    /// declares, a failure being a result with `isError` rather than a
    /// protocol error, so the caller reads it. A value the command's
    /// grammar refuses is coded `usage-invalid`, naming the argument as
    /// the tool's input does.
    fn call(&mut self, call: &ToolCall) -> ToolResult {
        let Some(tool) = self
            .tools
            .iter()
            .find(|tool| tool.definition["name"] == *call.name)
        else {
            return ToolResult::error::<ErrorCode>(anyhow!("Unknown tool: {}", call.name));
        };

        let mut printer = CapturePrinter::default();

        let result = tool
            .grammar
            .argv(&call.arguments)
            .and_then(|argv| {
                McpLine::try_parse_from(argv).map_err(|err| tool.grammar.usage_error(&err))
            })
            .and_then(|line| line.cmd.execute(&mut printer, self.session));

        ToolResult::printed::<ErrorCode>(printer, result)
    }
}

/// One tool call parsed back, the CLI grammar minus the binary name and
/// the global flags.
#[derive(Debug, Parser)]
#[command(no_binary_name = true)]
struct McpLine {
    #[command(subcommand)]
    cmd: Command,
}

#[cfg(test)]
mod tests {
    use crate::cli::mcp::{INSTRUCTIONS, TOOLS};

    /// Every tool is named in the instructions, so a new one cannot be
    /// listed without the agent being told what it does.
    #[test]
    fn the_instructions_name_every_tool() {
        for tool in TOOLS {
            let named = INSTRUCTIONS
                .split(|c: char| !c.is_ascii_alphanumeric())
                .any(|word| word == tool.name);

            assert!(named, "{}", tool.name);
        }
    }
}
