use std::fmt::{self, Display, Formatter};

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum AppStatus {
    Ok(String),
    NotFound(String),
    BadRequest(String),
    AuthError(String),
    InternalError(String),
}

impl Display for AppStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppStatus::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppStatus::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            AppStatus::InternalError(msg) => write!(f, "Internal error: {}", msg),
            AppStatus::AuthError(msg) => write!(f, "Auth error: {}", msg),
            AppStatus::Ok(msg) => write!(f, "Ok: {}", msg),
        }
    }
}
