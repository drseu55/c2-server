use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use base64;
use hex;

use crate::models::exchange_model;
use crate::utils;

#[post("/api/exchange")]
pub async fn exchange(
    req_body: web::Json<exchange_model::ExchangeRequest>,
) -> Result<impl Responder> {
    // TODO: Implement XChaCha20-Poly1305 for encypred communication
    // TODO: Implement Argon2 for password storing in db

    let keypair = utils::x25519::generate_keypair();
    let public_key_hex = hex::encode(keypair.1.to_bytes());

    let public_key_base64 = base64::encode(public_key_hex.as_bytes());

    let auth_response = exchange_model::ExchangeResponse::new(public_key_base64);

    // TODO: Change method to POST to use sender x25519 public key
    // HttpResponse::Ok().body(web::Json(auth_response));
    Ok(web::Json(auth_response))
}
