#[macro_use]
extern crate diesel;

use actix_web::{dev::ServiceRequest, middleware, web, App, HttpServer, Result};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv;

mod db;
mod errors;
mod handlers;
mod models;
mod schema;
mod utils;

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match handlers::auth_handler::validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::connect::connect(database_url);

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(handlers::ping_handler::ping)
            .service(handlers::exchange_handler::init_exchange)
            .service(handlers::auth_handler::auth)
            .service(handlers::auth_handler::register)
            .service(handlers::task_handler::implant_tasks)
            .service(handlers::result_handler::post_task)
            .service(
                web::scope("/api/web")
                    .wrap(auth)
                    .service(handlers::task_handler::tasks_post)
                    .service(handlers::task_handler::tasks_get),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
