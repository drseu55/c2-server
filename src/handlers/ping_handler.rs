use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};

#[get("/api/ping")]
pub async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}
