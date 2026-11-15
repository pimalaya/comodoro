//! # MCP tests
//!
//! `comodoro mcp` over stdio, against an in-process timer server on a
//! scratch socket, or against none.

mod common;

use std::{
    io::{BufRead, BufReader, Write},
    path::Path,
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

use serde_json::{Value, json};

use crate::common::{scratch, serve};

/// A server over a child process's stdio.
struct Server {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
}

impl Server {
    fn start(config: &Path, flags: &[&str]) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_comodoro"))
            .args(["-c", config.to_str().unwrap(), "mcp"])
            .args(flags)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());

        Self {
            child,
            stdin,
            stdout,
            next_id: 0,
        }
    }

    /// Sends one raw line and returns the response it gets.
    fn send(&mut self, line: &str) -> Value {
        writeln!(self.stdin, "{line}").unwrap();

        let mut line = String::new();
        self.stdout.read_line(&mut line).unwrap();
        serde_json::from_str(&line).unwrap_or_else(|err| panic!("{err}: {line}"))
    }

    /// Sends one request and returns its response.
    fn request(&mut self, method: &str, params: Value) -> Value {
        self.next_id += 1;
        let request =
            json!({ "jsonrpc": "2.0", "id": self.next_id, "method": method, "params": params });
        self.send(&request.to_string())
    }

    /// Calls a tool and returns its result.
    fn call(&mut self, name: &str, arguments: Value) -> Value {
        let params = json!({ "name": name, "arguments": arguments });
        self.request("tools/call", params)["result"].clone()
    }

    /// The tools listed, by name.
    fn tools(&mut self) -> Vec<Value> {
        let tools = self.request("tools/list", json!({}))["result"]["tools"].clone();
        tools.as_array().unwrap().clone()
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn names(tools: &[Value]) -> Vec<&str> {
    tools
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect()
}

#[test]
fn only_get_is_listed_without_flags() {
    let scratch = serve();
    let mut server = Server::start(&scratch.config, &[]);
    let tools = server.tools();

    assert_eq!(names(&tools), ["get"]);
    assert_eq!(tools[0]["annotations"]["readOnlyHint"], true);
    assert_eq!(tools[0]["annotations"]["openWorldHint"], false);
    assert_eq!(tools[0]["inputSchema"]["additionalProperties"], false);
    assert!(tools[0]["outputSchema"].is_object(), "{}", tools[0]);
}

#[test]
fn allow_write_lists_the_controls_stop_destructive() {
    let scratch = serve();
    let mut server = Server::start(&scratch.config, &["--allow-write"]);
    let tools = server.tools();

    assert_eq!(
        names(&tools),
        ["get", "start", "pause", "resume", "set", "stop"]
    );

    for tool in &tools {
        let name = tool["name"].as_str().unwrap();
        let annotations = &tool["annotations"];
        assert_eq!(annotations["openWorldHint"], false, "{name}");
        assert_eq!(annotations["readOnlyHint"], name == "get", "{name}");
        assert_eq!(annotations["destructiveHint"], name == "stop", "{name}");
    }
}

#[test]
fn initialize_names_the_tools_and_the_flag() {
    let scratch = serve();
    let mut server = Server::start(&scratch.config, &[]);
    let params = json!({ "protocolVersion": "2025-06-18" });
    let result = server.request("initialize", params)["result"].clone();
    let instructions = result["instructions"].as_str().unwrap();

    assert!(instructions.contains("get reads it"), "{instructions}");
    assert!(instructions.contains("--allow-write"), "{instructions}");
    assert!(
        instructions.contains("server-unreachable"),
        "{instructions}"
    );
}

#[test]
fn a_malformed_message_is_refused_and_the_server_reads_on() {
    let scratch = serve();
    let mut server = Server::start(&scratch.config, &[]);

    assert_eq!(server.send("not json")["error"]["code"], -32700);
    assert_eq!(server.send("[]")["error"]["code"], -32600);
    assert_eq!(server.request("ping", json!({}))["result"], json!({}));
}

#[test]
fn get_and_start_answer_the_timer_as_structured_content() {
    let scratch = serve();
    let mut server = Server::start(&scratch.config, &["--allow-write"]);

    let get = server.call("get", json!({}));
    assert_eq!(get["structuredContent"]["state"], "stopped", "{get}");
    assert!(get.get("isError").is_none(), "{get}");

    let start = server.call("start", json!({}));
    assert_eq!(start["structuredContent"]["state"], "running", "{start}");
    assert!(
        start["structuredContent"]["startedAt"].is_string(),
        "{start}"
    );

    let set = server.call("set", json!({ "seconds": 60 }));
    assert_eq!(set["structuredContent"]["cycle"]["duration"], 60, "{set}");
}

/// The failure object a refused call answers.
fn failure(result: &Value) -> Value {
    assert_eq!(result["isError"], true, "{result}");
    let text = result["content"][0]["text"].as_str().unwrap();
    serde_json::from_str(text).unwrap_or_else(|err| panic!("{err}: {text}"))
}

#[test]
fn an_unknown_argument_is_a_tool_error_coded_usage_invalid() {
    let scratch = serve();
    let mut server = Server::start(&scratch.config, &["--allow-write"]);
    let failure = failure(&server.call("set", json!({ "duration": 60 })));

    assert_eq!(failure["code"], "usage-invalid", "{failure}");
    let error = failure["error"].as_str().unwrap();
    assert!(error.contains("`duration`"), "{error}");
    assert!(error.contains("`seconds`"), "{error}");
}

/// A value the command's grammar refuses is coded like an undeclared
/// argument, named as the tool's input names it, and the server reads
/// on.
#[test]
fn a_value_the_grammar_refuses_is_a_tool_error_coded_usage_invalid() {
    let scratch = serve();
    let mut server = Server::start(&scratch.config, &["--allow-write"]);
    let failure = failure(&server.call("set", json!({ "seconds": -1 })));

    assert_eq!(failure["code"], "usage-invalid", "{failure}");
    let error = failure["error"].as_str().unwrap();
    assert_eq!(
        error,
        "Invalid value `-1` for `seconds`: invalid digit found in string"
    );

    let get = server.call("get", json!({}));
    assert_eq!(get["structuredContent"]["state"], "stopped", "{get}");
}

/// No timer server running is a failed call naming the code, and the
/// MCP server goes on.
#[test]
fn no_timer_server_answers_unreachable_and_the_server_reads_on() {
    let scratch = scratch();
    let mut server = Server::start(&scratch.config, &[]);
    let result = server.call("get", json!({}));

    assert_eq!(result["isError"], true, "{result}");
    let text = result["content"][0]["text"].as_str().unwrap();
    let failure: Value = serde_json::from_str(text).unwrap();
    assert_eq!(failure["code"], "server-unreachable", "{failure}");

    assert_eq!(server.request("ping", json!({}))["result"], json!({}));
}
