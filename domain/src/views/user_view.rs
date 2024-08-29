use crate::models::user::User;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct UserView {
    pub id: i64,
    pub username: String,
    pub register_complete: bool,
    pub register_date: DateTime<Utc>,
}

impl UserView {
    pub fn new(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            register_complete: user.register_complete,
            register_date: user.register_date,
        }
    }
}
