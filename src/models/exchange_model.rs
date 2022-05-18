use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ExchangeResponse {
    pub public_key: String,
}

impl ExchangeResponse {
    pub fn new(public_key: String) -> Self {
        ExchangeResponse { public_key }
    }
}

#[derive(Deserialize)]
pub struct ExchangeRequest {
    pub public_key: String,
}
