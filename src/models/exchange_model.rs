use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ExchangeResponse {
    pub action: String,
    pub public_key: String,
}

impl ExchangeResponse {
    pub fn new(action: String, public_key: String) -> Self {
        ExchangeResponse { action, public_key }
    }
}

#[derive(Deserialize)]
pub struct ExchangeRequest {
    pub action: String,
    pub public_key: String,
}
