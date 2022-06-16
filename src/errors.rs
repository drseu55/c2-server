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

impl std::convert::From<jsonwebtoken::errors::Error> for ServerError {
    fn from(_err: jsonwebtoken::errors::Error) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<base64::DecodeError> for ServerError {
    fn from(_err: base64::DecodeError) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<std::string::FromUtf8Error> for ServerError {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<uuid::Error> for ServerError {
    fn from(_err: uuid::Error) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<arrayvec::ArrayVec<u8, 32_usize>> for ServerError {
    fn from(_err: arrayvec::ArrayVec<u8, 32_usize>) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<arrayvec::ArrayVec<u8, 24_usize>> for ServerError {
    fn from(_err: arrayvec::ArrayVec<u8, 24_usize>) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<arrayvec::ArrayVec<u8, 16_usize>> for ServerError {
    fn from(_err: arrayvec::ArrayVec<u8, 16_usize>) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<Box<bincode::ErrorKind>> for ServerError {
    fn from(_err: Box<bincode::ErrorKind>) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<std::io::Error> for ServerError {
    fn from(_err: std::io::Error) -> Self {
        ServerError::InternalServerError
    }
}

impl std::convert::From<serde_json::Error> for ServerError {
    fn from(_err: serde_json::Error) -> Self {
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
