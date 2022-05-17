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
    fn from(err: argon2::password_hash::Error) -> Self {
        ServerError::InternalServerError
    }
}
