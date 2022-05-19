use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ExchangeResponse {
    pub public_key: String,
    pub implant_id: String,
}

impl ExchangeResponse {
    pub fn new(public_key: String, implant_id: String) -> Self {
        ExchangeResponse {
            public_key,
            implant_id,
        }
    }
}

#[derive(Deserialize)]
pub struct ExchangeRequest {
    pub public_key: String,
}
