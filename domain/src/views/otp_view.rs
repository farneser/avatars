use crate::models::otp::Otp;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct OtpView {
    pub id: String,
    pub user_id: i64,
    pub expires_at: DateTime<Utc>,
}

impl OtpView {
    pub fn new(otp: Otp) -> Self {
        Self {
            id: otp.id,
            user_id: otp.user_id,
            expires_at: otp.expires_at,
        }
    }
}
