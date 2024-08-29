use crate::models::session::Session;
use async_trait::async_trait;

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    /// Load and return the session by its ID
    ///
    /// ### Arguments
    ///
    /// * `id` - The unique identifier for the session to load
    ///
    /// ### Returns
    ///
    /// An `Option<Session>` containing the session if found, or `None` if the session does not exist
    async fn load(&mut self, id: &str) -> Option<Session>;

    /// Save the session and return the session ID
    ///
    /// ### Arguments
    ///
    /// * `session` - The session to save
    ///
    /// ### Returns
    ///
    /// The unique identifier of the saved session
    async fn save(&mut self, session: &Session) -> String;

    /// Destroy the session by its ID
    ///
    /// ### Arguments
    ///
    /// * `id` - The unique identifier for the session to destroy
    ///
    /// ### Returns
    ///
    /// A `Result` indicating whether the session was successfully destroyed
    async fn destroy(&mut self, id: &str) -> Result<bool, String>;

    /// Clean up expired sessions
    ///
    /// ### Returns
    ///
    /// A `Result` indicating whether the cleanup operation was successful
    async fn cleanup(&mut self) -> Result<(), String>;
}

pub struct InMemorySessionRepository {
    sessions: Vec<Session>,
}

impl InMemorySessionRepository {
    pub fn new() -> Self {
        InMemorySessionRepository { sessions: Vec::new() }
    }
}

#[async_trait]
impl SessionRepository for InMemorySessionRepository {
    async fn load(&mut self, id: &str) -> Option<Session> {
        let session = self.sessions.iter().find(|s| s.value == id);

        match session {
            Some(session) => Some(session.clone()),
            None => None,
        }
    }

    async fn save(&mut self, session: &Session) -> String {
        let mut session = session.clone();
        session.value = "1".to_string();

        self.sessions.push(session.clone());

        session.value
    }

    async fn destroy(&mut self, id: &str) -> Result<bool, String> {
        if let Some(index) = self.sessions.iter().position(|s| s.value == id) {
            self.sessions.remove(index);

            Ok(true)
        } else {
            Err("Session not found".to_string())
        }
    }

    async fn cleanup(&mut self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_session(id: &str) -> Session {
        Session::new(id.to_string(), "user123".to_owned(), 300)
    }

    #[tokio::test]
    async fn test_load() {
        // Given
        let mut repo = InMemorySessionRepository::new();
        let session = create_test_session("1");
        repo.save(&session).await;

        // When
        let loaded = repo.load("1").await;

        // Then
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap().value, "1");
    }

    #[tokio::test]
    async fn test_save() {
        // Given
        let mut repo = InMemorySessionRepository::new();
        let session = create_test_session("1");

        // When
        let id = repo.save(&session).await;

        // Then
        assert_eq!(id, "1");
        assert!(repo.load("1").await.is_some());
    }

    #[tokio::test]
    async fn test_destroy() {
        // Given
        let mut repo = InMemorySessionRepository::new();
        let session = create_test_session("1");
        repo.save(&session).await;

        // When
        let result = repo.destroy("1").await;

        // Then
        assert!(result.is_ok());
        assert!(repo.load("1").await.is_none());
    }

    #[tokio::test]
    async fn test_cleanup() {
        // Given
        let mut repo = InMemorySessionRepository::new();

        // When
        let result = repo.cleanup().await;

        // Then
        assert!(result.is_ok());
    }
}