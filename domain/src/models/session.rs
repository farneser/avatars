use chrono::{DateTime, Utc};

/// Represents a session with an ID, user ID, value, and creation time.
#[derive(Debug, Clone)]
pub struct Session {
    /// Unique identifier for the session
    pub id: i64,
    /// Identifier of the user associated with the session
    pub user_id: String,
    /// Unique identifier for the session
    pub value: String,
    /// Timestamp when the session was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the session was expired
    pub expired_at: DateTime<Utc>,
}

impl Session {
    /// Create a new session with the given ID and user ID
    ///
    /// ### Arguments
    ///
    /// * `id` - The unique identifier for the session
    /// * `user_id` - The identifier of the user associated with the session
    /// * `lifetime_seconds` - The number of seconds the session is valid
    ///
    /// ### Returns
    ///
    /// A new `Session` instance
    pub fn new(value: String, user_id: String, lifetime_seconds: usize) -> Self {
        let created_at = Utc::now();
        let expired_at = created_at + chrono::Duration::seconds(lifetime_seconds as i64);

        Self {
            id: 0,
            value,
            user_id,
            created_at,
            expired_at,
        }
    }
}
