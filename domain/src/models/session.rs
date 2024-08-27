use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a session with an ID, user ID, value, and creation time.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Session {
    /// Unique identifier for the session
    pub id: String,
    /// Identifier of the user associated with the session
    pub user_id: String,
    /// Timestamp when the session was created
    pub created_at: DateTime<Utc>,
}

impl Session {
    /// Create a new session with the given ID and user ID
    ///
    /// ### Arguments
    ///
    /// * `id` - The unique identifier for the session
    /// * `user_id` - The identifier of the user associated with the session
    ///
    /// ### Returns
    ///
    /// A new `Session` instance
    pub fn new(id: String, user_id: String) -> Self {
        Self {
            id,
            user_id,
            created_at: Utc::now(),
        }
    }
}
