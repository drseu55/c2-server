use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScreenshotResponse {
    pub bitflipped_bytes: Vec<u8>,
    pub width: u32,
    pub height: u32,
}
