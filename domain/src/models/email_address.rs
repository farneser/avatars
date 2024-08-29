use chrono::{DateTime, Utc};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Email {
    pub id: i32,
    pub user_id: i32,
    pub value: String,
    pub is_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Email {
    pub fn new(user_id: i32, value: String) -> Self {
        let now = Utc::now();

        Self {
            id: -1,
            user_id,
            value,
            is_verified: false,
            created_at: now,
            updated_at: now,
        }
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}