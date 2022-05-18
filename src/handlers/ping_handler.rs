use actix_web::{get, HttpResponse, Responder};

#[get("/api/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
