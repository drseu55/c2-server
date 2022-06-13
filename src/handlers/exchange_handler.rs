use actix_web::{post, web, Responder, Result};
use base64;
use diesel::prelude::*;
use uuid;
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
    let keypair = utils::network_encryption::generate_keypair();

    // Generate server public key and send it to implant
    let (private_key_base64, public_key_base64) = encode_keypairs(keypair);

    // Get data from request body and save it in database
    let implant_id = web::block(move || add_implant(db, req_body, private_key_base64)).await??;

    let exchange_response =
        exchange::ExchangeResponse::new(public_key_base64, implant_id.to_string());

    Ok(web::Json(exchange_response))
}

fn add_implant(
    db: web::Data<Pool>,
    req_body: web::Json<exchange::ExchangeRequest>,
    server_private_key: String,
) -> Result<uuid::Uuid, ServerError> {
    use crate::schema::implants::dsl::implants;

    let conn = db.get()?;

    let implant = implant::Implant::new(req_body.public_key.clone(), server_private_key);

    diesel::dsl::insert_into(implants)
        .values(&implant)
        .execute(&conn)?;

    Ok(implant.implant_id)
}

fn encode_keypairs(
    keypair: (x25519_dalek::StaticSecret, x25519_dalek::PublicKey),
) -> (String, String) {
    // Encode private key
    let private_key_base64 = base64::encode(keypair.0.to_bytes());

    // Encode public key
    let public_key_base64 = base64::encode(keypair.1.as_bytes());

    (private_key_base64, public_key_base64)
}
