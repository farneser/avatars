use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub register_date: DateTime<Utc>,
    pub last_update_date: DateTime<Utc>,
}

impl User {
    pub fn new(user_id: i64, username: String) -> Self {
        let now = Utc::now();

        User { user_id, username, register_date: now, last_update_date: now }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::user::User;

    #[tokio::test]
    pub async fn test_user_model() {
        let user = User::new(1, String::from("example"));

        assert_eq!(user.user_id, 1);
        assert_eq!(user.username, "example");
        assert_eq!(user.register_date, user.last_update_date);
    }
}