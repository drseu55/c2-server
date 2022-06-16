use serde_json::Value;
use std::fs;

use crate::errors::ServerError;

pub fn load_countries_file(path: &str) -> Result<String, ServerError> {
    Ok(fs::read_to_string(path)?)
}

pub fn parse_countries(data: String, search_country: String) -> Result<(f64, f64), ServerError> {
    let json: Value = serde_json::from_str(&data)?;

    let coordinates = &json[search_country];

    // Using unwrap is safe here
    // because countriesCoordinates file does not change
    // and content is known before compilation
    let x = &coordinates["x"].as_f64().unwrap();
    let y = &coordinates["y"].as_f64().unwrap();

    Ok((*x, *y))
}
