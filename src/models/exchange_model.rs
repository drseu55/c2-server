use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ExchangeResponse {
    pub value: String,
}

impl ExchangeResponse {
    pub fn new(value: String) -> Self {
        ExchangeResponse { value }
    }
}

#[derive(Deserialize)]
pub struct ExchangeRequest {
    pub action: String,
    pub key: String,
}
