use std::fmt;
use std::fmt::{Display, Formatter};

pub mod user_repository;
pub mod id_provider;
pub mod session_repository;
pub mod otp_repository;
pub const OTP_LENGTH: usize = 8;


#[derive(thiserror::Error, Debug, PartialEq)]
pub enum DbError {
    NotFound(String),
    InternalError(String),
    UniqueViolation(String),
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DbError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DbError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            DbError::UniqueViolation(msg) => write!(f, "Unique violation: {}", msg),
        }
    }
}