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
        let session = self.sessions.iter().find(|s| s.value == id);

        match session {
            Some(_) => Ok(true),
            None => Err("Session not found".to_string()),
        }
    }

    async fn cleanup(&mut self) -> Result<(), String> {
        Ok(())
    }
}