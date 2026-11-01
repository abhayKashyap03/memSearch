//! JSON-RPC 2.0 framing. Keep this module free of memory logic so it can be
//! tested on its own.
#![allow(dead_code)]

/// A request id is a number or a string. Notifications have none.
#[derive(Debug, Clone, PartialEq)]
pub enum RequestId {
    Number(i64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Request {
    pub id: Option<RequestId>,
    pub method: String,
    /// Raw JSON of `params`, until you pick a JSON representation.
    pub params_json: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Response {
    Result {
        id: RequestId,
        result_json: String,
    },
    Error {
        id: Option<RequestId>,
        code: i32,
        message: String,
    },
}

/// Standard JSON-RPC error codes (see the JSON-RPC 2.0 spec, section 5.1).
pub mod codes {
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;
}

/// Parse one line from stdin.
pub fn parse_request(line: &str) -> Result<Request, Response> {
    let _ = line;
    todo!()
}

/// Serialise a response as one line (no trailing newline).
pub fn encode_response(resp: &Response) -> String {
    let _ = resp;
    todo!()
}

/// Route a request. Returns `None` for notifications (no reply allowed).
pub fn handle(req: &Request) -> Option<Response> {
    let _ = req;
    todo!("initialize, tools/list, tools/call, unknown -> METHOD_NOT_FOUND")
}
