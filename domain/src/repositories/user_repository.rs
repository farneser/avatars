use crate::models::otp::OTP;
use crate::models::user::User;
use crate::repositories::DbError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_by_login(&self, login: &str) -> Option<User>;
    async fn save(&self, user: User) -> Result<User, DbError>;
    async fn find_otp_by_username(&self, username: &str, otp: &str) -> Option<OTP>;
}