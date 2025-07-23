use serde::Deserialize;
use serde_json;
#[derive(Debug, Deserialize)]
pub struct GetBundleStatusesResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: Option<GetBundleStatusesResult>, // can be null if bundle is not found
}

#[derive(Debug, Deserialize)]
pub struct GetBundleStatusesResult {
    pub context: Context,
    pub value: Vec<BundleStatus>,
}

#[derive(Debug, Deserialize)]
pub struct Context {
    pub slot: u64,
}

#[derive(Debug, Deserialize)]
pub struct BundleStatus {
    pub bundle_id: String,
    pub transactions: Vec<String>,
    pub slot: u64,

    #[serde(rename = "confirmation_status")]
    pub confirmation_status: String,

    pub err: Option<ErrorWrapper>, // err may be null
}

#[derive(Debug, Deserialize)]
pub struct ErrorWrapper {
    pub Ok: Option<serde_json::Value>, // Allows handling of null or other possible formats
}
