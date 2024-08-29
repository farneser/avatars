use chrono::{DateTime, Utc};
use sqlx::FromRow;

pub const OTP_LENGTH: usize = 8;

pub enum OtpError {
    InvalidLength,
}

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct Otp {
    pub id: String,
    pub user_id: i64,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Otp {
    pub fn new(id: String, user_id: i64, lifetime_seconds: usize) -> Result<Self, String> {
        let created_at = Utc::now();
        let expires_at = created_at + chrono::Duration::seconds(lifetime_seconds as i64);

        if id.len() != OTP_LENGTH {
            return Err("Invalid OTP length".to_owned());
        }

        Ok(Self { id, user_id, created_at, expires_at })
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::otp::Otp;

    #[tokio::test]
    pub async fn test_otp_model_valid() {
        // Given
        let otp = Otp::new("12345678".to_owned(), 1, 100).unwrap();

        // Then
        assert_eq!(otp.id, "12345678");
        assert_eq!(otp.user_id, 1);
        assert_eq!(otp.created_at, otp.expires_at - chrono::Duration::seconds(100));
    }

    #[tokio::test]
    pub async fn test_otp_model_invalid() {
        // Given
        let otp = Otp::new("1234567".to_owned(), 1, 100);

        // Then
        assert_eq!(otp, Err("Invalid OTP length".to_owned()));
    }

    #[tokio::test]
    pub async fn test_otp_is_expired() {
        // Given
        let otp = Otp::new("12345678".to_owned(), 1, 100).unwrap();

        // Then
        assert_eq!(otp.is_expired(), false);
    }
}