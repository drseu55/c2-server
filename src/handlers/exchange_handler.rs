use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use base64;
use diesel::prelude::*;
use hex;

use crate::db::connect::Pool;
use crate::errors::ServerError;
use crate::models::exchange_model;
use crate::models::implant;
use crate::utils;

#[post("/api/exchange")]
pub async fn exchange(
    db: web::Data<Pool>,
    req_body: web::Json<exchange_model::ExchangeRequest>,
) -> Result<impl Responder, actix_web::Error> {
    // TODO: Check if action is correct, else terminate

    // Get data from request body and save it in database
    web::block(move || add_implant(db, req_body)).await??;

    // Generate server public key and send it to implant
    let keypair = utils::x25519::generate_keypair();
    let public_key_hex = hex::encode(keypair.1.to_bytes());

    let public_key_base64 = base64::encode(public_key_hex.as_bytes());

    let auth_response =
        exchange_model::ExchangeResponse::new("exchange".to_string(), public_key_base64);

    Ok(web::Json(auth_response))
}

fn add_implant(
    db: web::Data<Pool>,
    req_body: web::Json<exchange_model::ExchangeRequest>,
) -> Result<(), ServerError> {
    use crate::schema::implants::dsl::*;

    let conn = db.get()?;

    let implant = implant::Implant::new(&req_body.public_key);

    diesel::dsl::insert_into(implants)
        .values(&implant)
        .execute(&conn)?;

    Ok(())
}
