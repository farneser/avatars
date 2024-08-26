use crate::models::otp::OTP;

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    InternalError(String),
    InvalidMail(String),
}

pub trait MailService {
    fn send(&self, email: &str, subject: &str, html_body: &str, plain_body: &str) -> Result<(), EmailError>;
    fn send_otp(&self, email: &str, otp: OTP) -> Result<(), EmailError>;
}
