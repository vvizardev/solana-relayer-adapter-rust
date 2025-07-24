use serde::Deserialize;
use serde_json::{self, Value};

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
}