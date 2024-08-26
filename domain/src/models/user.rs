use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub login_attempts: i8,
    pub register_complete: bool,
    pub register_date: DateTime<Utc>,
    pub last_update_date: DateTime<Utc>,
}

impl User {
    pub fn new(username: String) -> Self {
        let now = Utc::now();

        User {
            id: -1,
            username,
            register_complete: false,
            login_attempts: 0,
            register_date: now,
            last_update_date: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::user::User;

    #[tokio::test]
    pub async fn test_user_model() {
        let user = User::new(String::from("example"));

        assert_eq!(user.id, -1);
        assert_eq!(user.username, "example");
        assert_eq!(user.register_date, user.last_update_date);
        assert_eq!(user.register_complete, false);
        assert_eq!(user.login_attempts, 0);
    }
}