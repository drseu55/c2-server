use actix_web::{get, post, web, Responder, Result};
use base64;
use diesel::prelude::*;
use hex;
use x25519_dalek;

use crate::db::connect::Pool;
use crate::errors::ServerError;
use crate::models::exchange;
use crate::models::implant;
use crate::utils;

#[post("/api/exchange")]
pub async fn init_exchange(
    db: web::Data<Pool>,
    req_body: web::Json<exchange::ExchangeRequest>,
) -> Result<impl Responder, ServerError> {
    // Generate keypair
    let keypair = utils::x25519::generate_keypair();

    // Generate server public key and send it to implant
    let (private_key_base64, exchange_response) = generate_and_encode(keypair);

    // Get data from request body and save it in database
    web::block(move || add_implant(db, req_body, private_key_base64)).await??;

    Ok(web::Json(exchange_response))
}

fn add_implant(
    db: web::Data<Pool>,
    req_body: web::Json<exchange::ExchangeRequest>,
    server_private_key: String,
) -> Result<(), ServerError> {
    use crate::schema::implants::dsl::implants;

    let conn = db.get()?;

    let implant = implant::Implant::new(req_body.public_key.clone(), server_private_key);

    diesel::dsl::insert_into(implants)
        .values(&implant)
        .execute(&conn)?;

    Ok(())
}

fn generate_and_encode(
    keypair: (x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
) -> (String, exchange::ExchangeResponse) {
    // Encode private key
    let private_key_base64 = base64::encode(keypair.0.to_bytes());

    // Encode public key
    let public_key_hex = hex::encode(keypair.1.to_bytes());

    let public_key_base64 = base64::encode(public_key_hex.as_bytes());

    let exchange_response = exchange::ExchangeResponse::new(public_key_base64);

    (private_key_base64, exchange_response)
}
