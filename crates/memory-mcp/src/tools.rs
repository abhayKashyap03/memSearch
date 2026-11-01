//! The tools the agent sees. Tool names and descriptions are part of your
//! product: the model decides when to call them from this text alone.
#![allow(dead_code)]

/// `remember`: store a fact for later.
pub struct RememberArgs {
    pub text: String,
    pub tags: Vec<String>,
}

/// `recall`: retrieve relevant memories.
pub struct RecallArgs {
    pub query: String,
    pub k: Option<usize>,
    pub since_ms: Option<u64>,
    pub tags: Vec<String>,
}

/// `forget`: delete a memory by id.
pub struct ForgetArgs {
    pub id: u64,
}

pub const TOOL_NAMES: &[&str] = &["remember", "recall", "forget"];

/// JSON Schema for each tool's `inputSchema` in `tools/list`.
pub fn input_schema_json(tool: &str) -> Option<String> {
    let _ = tool;
    todo!()
}

/// Format recall hits for the model. Keep it compact: every byte here is
/// paid for on every turn. Measuring that cost is part of `bench`.
pub fn format_recall(hits: &[memory::RecallHit]) -> String {
    let _ = hits;
    todo!()
}
