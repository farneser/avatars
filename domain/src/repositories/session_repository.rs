use crate::models::session::Session;

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
    async fn load(&self, id: &str) -> Option<Session>;

    /// Save the session and return the session ID
    ///
    /// ### Arguments
    ///
    /// * `session` - The session to save
    ///
    /// ### Returns
    ///
    /// The unique identifier of the saved session
    async fn save(&self, session: &Session) -> String;

    /// Destroy the session by its ID
    ///
    /// ### Arguments
    ///
    /// * `id` - The unique identifier for the session to destroy
    ///
    /// ### Returns
    ///
    /// A `Result` indicating whether the session was successfully destroyed
    async fn destroy(&self, id: &str) -> Result<bool, String>;

    /// Clean up expired sessions
    ///
    /// ### Returns
    ///
    /// A `Result` indicating whether the cleanup operation was successful
    async fn cleanup(&self) -> Result<(), String>;
}
