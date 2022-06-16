use serde::Serialize;

#[derive(Serialize)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
}

impl Coordinates {
    pub fn new(x: f64, y: f64) -> Self {
        Coordinates { x, y }
    }
}
