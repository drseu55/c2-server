use actix_web::error::BlockingError;
use actix_web::HttpResponse;
use actix_web::ResponseError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error("Unauthorized")]
    Unauthorized,
}

impl std::convert::From<argon2::password_hash::Error> for ServerError {
    fn from(_err: argon2::password_hash::Error) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<r2d2::Error> for ServerError {
    fn from(_err: r2d2::Error) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<actix_web::Error> for ServerError {
    fn from(_err: actix_web::Error) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<diesel::result::Error> for ServerError {
    fn from(_err: diesel::result::Error) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<BlockingError> for ServerError {
    fn from(_err: BlockingError) -> Self {
        ServerError::InternalServerError
    }
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServerError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServerError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}
