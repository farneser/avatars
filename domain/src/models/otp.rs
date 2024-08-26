use chrono::{DateTime, Utc};
use sqlx::FromRow;

pub const OTP_LENGTH: usize = 8;
pub const OTP_LIFETIME_MINUTES: i64 = 5;

#[derive(Debug, Clone, FromRow, PartialEq)]
pub struct OTP {
    pub id: i64,
    pub user_id: i64,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl OTP {
    pub fn new(value: String, user_id: i64) -> Result<Self, String> {
        let created_at = Utc::now();
        let expires_at = created_at + chrono::Duration::minutes(OTP_LIFETIME_MINUTES);

        if value.len() != OTP_LENGTH {
            return Err("Invalid OTP length".to_owned());
        }

        Ok(Self  { id: -1, value, user_id, created_at, expires_at })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::otp::{OTP, OTP_LIFETIME_MINUTES};

    #[tokio::test]
    pub async fn test_otp_model_valid() {
        // Given
        let otp = OTP::new("12345678".to_owned(), 1).unwrap();

        // Then
        assert_eq!(otp.id, -1);
        assert_eq!(otp.value, "12345678");
        assert_eq!(otp.user_id, 1);
        assert_eq!(otp.created_at, otp.expires_at - chrono::Duration::minutes(OTP_LIFETIME_MINUTES));
    }

    #[tokio::test]
    pub async fn test_otp_model_invalid() {
        // Given
        let otp = OTP::new("1234567".to_owned(), 1);

        // Then
        assert_eq!(otp, Err("Invalid OTP length".to_owned()));
    }
}