use crate::models::user::User;
use crate::repositories::id_provider::IdProvider;
use crate::repositories::session_repository::{Session, SessionRepository};
use crate::repositories::user_repository::UserRepository;

pub struct UserService<UR, SR, I>
where
    UR: UserRepository,
    SR: SessionRepository,
    I: IdProvider,
{
    user_repository: UR,
    session_repository: SR,
    id_provider: I,
}


impl<UR, SR, I> UserService<UR, SR, I>
where
    UR: UserRepository,
    SR: SessionRepository,
    I: IdProvider,
{
    pub fn new(user_repository: UR, session_repository: SR, id_provider: I) -> Self {
        UserService { user_repository, session_repository, id_provider }
    }

    pub async fn find_by_login(&self, login: &str) -> Result<User, String> {
        let user = self.user_repository.find_by_login(login).await;

        match user {
            Some(user) => Ok(user),
            None => Err("User not found".to_string()),
        }
    }

    pub async fn save(&self, user: User) -> Result<User, String> {
        match self.user_repository.save(user).await {
            Ok(user) => Ok(user),
            Err(_) => Err("Error saving user".to_string()),
        }
    }

    pub async fn validate_otp(&self, login: &str, otp: &str) -> Result<User, String> {
        let user = self.user_repository.find_by_login(login).await;

        match user {
            Some(user) => {
                let otp = self.user_repository.find_otp_by_username(login, otp).await;

                match otp {
                    Some(_) => Ok(user),
                    None => Err("Invalid OTP".to_string()),
                }
            }
            None => Err("User not found".to_string()),
        }
    }

    pub async fn generate_session_id(&self, login: &str) -> Result<Session, String> {
        let user = self.user_repository.find_by_login(login).await;

        match user {
            Some(user) => {
                let session_id = self.id_provider.get_id(32).await;

                let session = Session::new(session_id.clone(), user.id.clone());

                match self.session_repository.save(&session).await {
                    Ok(_) => Ok(session),
                    Err(_) => Err("Error saving session".to_string()),
                }
            }
            None => Err("User not found".to_string()),
        }
    }
}

