use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub result: String,
    pub id: u32,
}
