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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};
    use tokio::time::{sleep, Duration as TokioDuration};

    #[tokio::test]
    async fn test_session_initialization() {
        // Given
        let user_id = String::from("user123");
        let value = String::from("session_token");
        let lifetime_seconds = 3600;

        // When
        let session = Session::new(value.clone(), user_id.clone(), lifetime_seconds);

        // Then
        assert_eq!(session.user_id, user_id);
        assert_eq!(session.value, value);
        assert_eq!(session.id, 0);

        let now = Utc::now();

        assert!(session.created_at <= now);
        assert!(session.expired_at > now);
        assert_eq!(
            session.expired_at,
            session.created_at + Duration::seconds(lifetime_seconds as i64)
        );
    }

    #[tokio::test]
    async fn test_session_lifetime() {
        // Given
        let user_id = String::from("user123");
        let value = String::from("session_token");
        let lifetime_seconds = 3600;

        // When
        let session = Session::new(value.clone(), user_id.clone(), lifetime_seconds);

        // Then
        let expected_expiry = session.created_at + Duration::seconds(lifetime_seconds as i64);

        assert_eq!(session.expired_at, expected_expiry);
    }

    #[tokio::test]
    async fn test_session_expiration() {
        // Given
        let user_id = String::from("user123");
        let value = String::from("session_token");
        let lifetime_seconds = 1; // 1 second

        // When
        let session = Session::new(value.clone(), user_id.clone(), lifetime_seconds);

        sleep(TokioDuration::from_secs(3)).await;

        // Then
        let now = Utc::now();
        assert!(session.expired_at <= now, "Session should be expired");
    }
}
