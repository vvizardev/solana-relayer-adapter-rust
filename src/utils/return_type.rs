use serde::Deserialize;
use serde_json::{self, Value};

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<u32>,
    pub result: Option<String>,
    pub error: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct BlockRazorResponse {
    pub signature: Option<String>,
    pub error: Option<String>,
}
