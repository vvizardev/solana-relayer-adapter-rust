use serde::Deserialize;
use serde_json::{self, Value};

#[derive(Debug, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: Option<String>, // Made optional to handle responses without this field
    pub id: Option<u32>,
    pub result: Option<String>,
    pub error: Option<Value>,
}

// Additional response types for different service formats
#[derive(Debug, Deserialize)]
pub struct ServiceErrorResponse {
    pub error: Option<String>,
    pub instance: Option<String>,
    #[serde(rename = "request-id")]
    pub request_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SimpleErrorResponse {
    pub code: Option<i32>,
    pub message: Option<String>,
    pub details: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize)]
pub struct BlockRazorResponse {
    pub signature: String,
    pub error: String,
}
