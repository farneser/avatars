use crate::models::otp::Otp;
use crate::models::user::User;
use crate::repositories::DbError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_by_login(&self, login: &str) -> Option<User>;
    async fn save(&mut self, user: User) -> Result<User, DbError>;
    async fn find_otp_by_username(&self, username: &str, otp: &str) -> Option<Otp>;
}

pub struct InMemoryUserRepository {
    pub users: Vec<User>,
    pub otp: Vec<Otp>,
    pub counter: u64,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository { users: Vec::new(), otp: Vec::new(), counter: 1 }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn find_by_login(&self, login: &str) -> Option<User> {
        self.users.iter().find(|u| u.username == login).cloned()
    }

    async fn save(&mut self, user: User) -> Result<User, DbError> {
        let mut user = user;

        user.id = self.counter as i64;

        self.counter += 1;
        self.users.push(user.clone());

        Ok(user)
    }

    async fn find_otp_by_username(&self, username: &str, otp: &str) -> Option<Otp> {
        let user = self.users.iter().find(|u| u.username == username);

        match user {
            Some(user) => self.otp.iter().find(|o| o.user_id == user.id && o.id == otp).cloned(),
            None => None,
        }
    }
}