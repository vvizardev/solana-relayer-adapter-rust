use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HealthResponse {
    pub result: String,
}
