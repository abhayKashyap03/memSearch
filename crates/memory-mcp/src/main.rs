//! # memory-mcp
//!
//! An MCP (Model Context Protocol) server over stdio that exposes the
//! `memory` crate as tools an agent can call.
//!
//! Spec: https://modelcontextprotocol.io/specification
//! Transport: newline-delimited JSON-RPC 2.0 on stdin/stdout. Logs go to
//! stderr only, never stdout, or you corrupt the protocol stream.
//!
//! Lifecycle to support, in order:
//! 1. `initialize` request -> reply with protocol version + capabilities
//!    (`tools`), then accept the `notifications/initialized` notification.
//! 2. `tools/list` -> describe the tools in [`tools`] with JSON Schemas.
//! 3. `tools/call` -> dispatch to the store, return text content.
//!
//! Decision for you: hand-roll JSON parsing (educational, painful) or add
//! `serde` + `serde_json` to the workspace (normal). Either is fine.

mod protocol;
mod tools;

fn main() {
    // TODO(you):
    // - build a MemoryStore (FlatIndex at first, HNSW once it works)
    // - loop over stdin lines, parse each as a protocol::Request
    // - route with protocol::handle, write the Response line to stdout
    todo!("stdio JSON-RPC loop")
}
