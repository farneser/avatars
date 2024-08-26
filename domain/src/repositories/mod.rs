use std::fmt;
use std::fmt::{Display, Formatter};

pub mod user_repository;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum DbError {
    NotFound(String),
    InternalError(String),
}

impl Display for DbError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DbError::NotFound(msg) => write!(f, "Not found: {}", msg),
            DbError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}