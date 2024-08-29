use crate::models::otp::{Otp, OTP_LENGTH};
use crate::models::session::Session;
use crate::models::user::User;
use crate::repositories::id_provider::IdProvider;
use crate::repositories::otp_repository::OtpRepository;
use crate::repositories::session_repository::SessionRepository;
use crate::repositories::user_repository::UserRepository;

pub struct UserService<UR, SR, OR, IP>
where
    UR: UserRepository + Sync + Send,
    SR: SessionRepository + Sync + Send,
    OR: OtpRepository + Sync + Send,
    IP: IdProvider + Sync + Send,
{
    user_repository: UR,
    session_repository: SR,
    otp_repository: OR,
    id_provider: IP,
}


impl<UR, SR, OR, IP> UserService<UR, SR, OR, IP>
where
    UR: UserRepository + Sync + Send,
    SR: SessionRepository + Sync + Send,
    OR: OtpRepository + Sync + Send,
    IP: IdProvider + Sync + Send,
{
    pub fn new(user_repository: UR, session_repository: SR, otp_repository: OR, id_provider: IP) -> Self {
        UserService { user_repository, session_repository, id_provider, otp_repository }
    }

    pub async fn find_by_login(&self, login: &str) -> Result<User, String> {
        let user = self.user_repository.find_by_login(login).await;

        match user {
            Some(user) => Ok(user),
            None => Err("User not found".to_string()),
        }
    }

    pub async fn save(&mut self, user: User) -> Result<User, String> {
        match self.user_repository.save(user).await {
            Ok(user) => Ok(user),
            Err(_) => Err("Error saving user".to_string()),
        }
    }

    pub async fn validate_otp(&self, login: &str, otp: &str) -> Result<User, String> {
        let user = self.user_repository.find_by_login(login).await;

        // TODO count login attempts

        match user {
            Some(user) => {
                let otp = self.otp_repository.find_by_id(otp).await;

                match otp {
                    Some(_) => Ok(user),
                    None => Err("Invalid OTP".to_string()),
                }
            }
            None => Err("User not found".to_string()),
        }
    }

    pub async fn generate_session_id(&mut self, login: &str) -> Result<Session, String> {
        let user = self.user_repository.find_by_login(login).await;

        match user {
            Some(user) => {
                let session_id = self.id_provider.get_id(32);

                let session = Session::new(session_id.clone(), user.id.to_string(), 300000);

                self.session_repository.save(&session).await;

                Ok(session)
            }
            None => Err("User not found".to_string()),
        }
    }

    pub async fn save_otp(&mut self, user_id: i64) -> Result<String, String> {
        let otp_id = self.id_provider.get_numeric_id(OTP_LENGTH);

        let otp = Otp::new(otp_id.clone(), user_id, 300).map_err(|e| e.to_string())?;

        self.otp_repository.save(otp).await.map_err(|_| "Error saving OTP".to_string())?;

        Ok(otp_id)
    }
}

