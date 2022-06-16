use actix_web::{get, web, HttpResponse, Responder};

use crate::errors::ServerError;
use crate::models::coordinates::Coordinates;
use crate::utils::parse_countries;

const FILE_PATH: &str = "countriesCoordinates.json";

#[get("/coordinates/{country}")]
pub async fn country_get(path: web::Path<String>) -> Result<impl Responder, ServerError> {
    let all_coordinates = parse_countries::load_countries_file(FILE_PATH)?;

    let country = path.into_inner();

    let country_coordinates = parse_countries::parse_countries(all_coordinates, country)?;

    let response = Coordinates::new(country_coordinates.0, country_coordinates.1);

    Ok(HttpResponse::Ok().json(response))
}
