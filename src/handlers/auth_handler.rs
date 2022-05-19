use actix_web::{post, web, HttpResponse, Responder, Result};
use base64;
use chrono;
use diesel::prelude::*;
use dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::db::connect::Pool;
use crate::errors::ServerError;
use crate::models::user;
use crate::utils::argon2;

const JWT_TOKEN_DURATION: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    exp: i64,
}

// Check if token is valid with function from jsonwebtoken
pub fn validate_token(token: &str) -> Result<bool, ServerError> {
    dotenv::dotenv().ok();
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let decoded_token = std::string::String::from_utf8(base64::decode(token)?)?;

    let token_data = decode::<Claims>(
        &decoded_token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )?;

    // Additional checks
    if token_data.claims.iss == "C2-Server".to_owned() {
        Ok(true)
    } else {
        Ok(false)
    }
}

// FIXME: Helper endpoint. Remove it before final release.
#[post("/api/register")]
pub async fn register(
    db: web::Data<Pool>,
    req_body: web::Json<user::UserAuthRequest>,
) -> Result<impl Responder, ServerError> {
    web::block(move || add_user(db, req_body)).await??;

    Ok(HttpResponse::Ok().body("everything went well"))
}

fn add_user(
    db: web::Data<Pool>,
    req_body: web::Json<user::UserAuthRequest>,
) -> Result<(), ServerError> {
    use crate::schema::users::dsl::users;

    let conn = db.get()?;

    let argon2_hash = argon2::generate_argon2_hash(&req_body.password)?;

    let user = user::User::new(req_body.username.clone(), argon2_hash);

    diesel::dsl::insert_into(users)
        .values(&user)
        .execute(&conn)?;

    Ok(())
}

#[post("/api/auth")]
pub async fn auth(
    db: web::Data<Pool>,
    req_body: web::Json<user::UserAuthRequest>,
) -> Result<impl Responder, ServerError> {
    // TODO: (Optional): Check for username and password good practices
    // Check if username and password matches username and password in db
    let is_matches =
        web::block(move || check_if_matches(&req_body.username, &req_body.password, db)).await??;

    if !is_matches {
        // Send 401 Unauthorized to the user
        return Ok(HttpResponse::Unauthorized().finish());
    }

    // Generate JWT token
    dotenv::dotenv().ok();
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let exp = (chrono::Local::now() + chrono::Duration::hours(JWT_TOKEN_DURATION)).timestamp();

    let my_claims = Claims {
        iss: "C2-Server".to_owned(),
        exp,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?;

    // Base64 encode token
    let token_base64 = base64::encode(token.as_bytes());

    // Send the JWT token to the user
    let auth_response = user::UserAuthResponse::new(token_base64);

    Ok(HttpResponse::Ok().json(auth_response))
}

fn check_if_matches(
    req_username: &String,
    req_password: &String,
    db: web::Data<Pool>,
) -> Result<bool, ServerError> {
    use crate::schema::users::dsl::{username, users};

    let conn = db.get()?;

    let mut items = users
        .filter(username.eq(req_username))
        .load::<user::User>(&conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = argon2::verify_argon2(req_password, &user.password) {
            if matching {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

#[post("/testauth")]
pub async fn testauth() -> impl Responder {
    HttpResponse::Ok().body("Testing auth baby!")
}
