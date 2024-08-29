use crate::views::otp_view::OtpView;
use async_trait::async_trait;
use std::fmt::Display;

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
    async fn send_otp(&self, email: &str, otp: OtpView) -> Result<(), EmailError>;
}

pub struct InMemoryMailService {}

impl InMemoryMailService {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl MailService for InMemoryMailService {
    async fn send(&self, email: &str, subject: &str, html_body: &str, plain_body: &str) -> Result<(), EmailError> {
        println!("email: {}, subject: {}, html_body: {}, plain_body: {}", email, subject, html_body, plain_body);

        Ok(())
    }

    async fn send_otp(&self, email: &str, otp: OtpView) -> Result<(), EmailError> {
        println!("email: {}, otp: {:?}", email, otp);

        Ok(())
    }
}