use std::fmt::{self, Display, Formatter};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    AuthError(String),
    InternalError(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppError::AuthError(msg) => write!(f, "Auth error: {}", msg),
        }
    }
}
