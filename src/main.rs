#[macro_use]
extern crate diesel;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use dotenv;

mod db;
mod errors;
mod handlers;
mod models;
mod schema;
mod utils;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::connect::connect(database_url);

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(handlers::ping_handler::ping)
            .service(echo)
            .service(handlers::exchange_handler::exchange)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
