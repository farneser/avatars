use crate::models::otp::Otp;
use crate::models::user::User;
use crate::repositories::DbError;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn find_by_login(&self, login: &str) -> Option<User>;
    async fn save(&mut self, user: User) -> Result<User, DbError>;
}

pub struct InMemoryUserRepository {
    pub users: Vec<User>,
    pub counter: u64,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        InMemoryUserRepository { users: Vec::new(), counter: 1 }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::otp::Otp;
    use crate::models::user::User;

    fn create_test_user(username: &str) -> User {
        User::new(username.to_string())
    }

    fn create_test_otp(user_id: i64, otp_id: &str) -> Otp {
        Otp::new(otp_id.to_string(), user_id, 300).unwrap()
    }

    #[tokio::test]
    async fn test_find_by_login() {
        // Given
        let mut repo = InMemoryUserRepository::new();
        let user = create_test_user("test_user");
        repo.save(user.clone()).await.unwrap();

        // When
        let found_user = repo.find_by_login("test_user").await;

        // Then
        assert!(found_user.is_some());
        assert_eq!(found_user.unwrap().username, "test_user");
    }

    #[tokio::test]
    async fn test_save() {
        // Given
        let mut repo = InMemoryUserRepository::new();
        let user = create_test_user("test_user");

        // When
        let saved_user = repo.save(user.clone()).await.unwrap();

        // Then
        assert_eq!(saved_user.username, "test_user");
        assert!(repo.find_by_login("test_user").await.is_some());
    }
}