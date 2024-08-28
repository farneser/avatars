use crate::models::otp::OTP;
use std::fmt::Display;
use async_trait::async_trait;

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    InternalError(String),
    InvalidMail(String),
}

impl Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EmailError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            EmailError::InvalidMail(msg) => write!(f, "Invalid mail: {}", msg),
        }
    }
}

#[async_trait]
pub trait MailService {
    async fn send(&self, email: &str, subject: &str, html_body: &str, plain_body: &str) -> Result<(), EmailError>;
    async fn send_otp(&self, email: &str, otp: OTP) -> Result<(), EmailError>;
}
