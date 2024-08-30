use crate::command::{Command, CommandHandler};
use crate::shared::error::AppStatus;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// The Mediator struct manages command handlers associated with command types.
pub struct Mediator {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Mediator {
    pub fn new() -> Self {
        Self { handlers: HashMap::new() }
    }

    /// Registers a handler for a specific command type
    ///
    /// # Arguments
    ///
    /// * `handler` - The handler to be registered.
    pub fn register_handler<REQUEST, RESPONSE: 'static, H>(&mut self, handler: H)
    where
        REQUEST: Command<RESPONSE> + 'static,
        H: CommandHandler<REQUEST, RESPONSE> + Send + Sync + 'static,
    {
        let handler: Arc<Mutex<dyn CommandHandler<REQUEST, RESPONSE> + Send + Sync>> = Arc::new(Mutex::new(handler));

        self.handlers.insert(TypeId::of::<REQUEST>(), Box::new(handler));
    }

    /// Sends a command to the appropriate handler and awaits the result.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to be sent.
    ///
    /// # Returns
    ///
    /// The result of the command handler's handle method.
    pub async fn send<REQUEST, RESPONSE>(&self, command: REQUEST) -> Result<RESPONSE, AppStatus>
    where
        REQUEST: Command<RESPONSE> + 'static,
        RESPONSE: 'static,
    {
        // Retrieve the handler associated with the command's type.
        let handler = self.handlers.get(&TypeId::of::<REQUEST>())
            .ok_or(AppStatus::InternalError("Handler not found".to_string()))?;

        // Downcast the handler to the specific type we expect (Arc<Mutex<dyn CommandHandler>>).
        let handler = match handler
            .downcast_ref::<Arc<Mutex<dyn CommandHandler<REQUEST, RESPONSE> + Send + Sync>>>()
            .ok_or(AppStatus::InternalError("Handler type mismatch".to_string()))
        {
            Ok(handler) => handler,
            Err(err) => return Err(err),
        };

        // Lock the mutex to access the handler and then call its handle method
        let mut handler = handler.lock().await;

        handler.handle(command).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct TestCommand;
    struct TestResponse(pub String);
    impl Command<TestResponse> for TestCommand {}
    struct TestCommandHandler;

    #[async_trait]
    impl CommandHandler<TestCommand, TestResponse> for TestCommandHandler {
        async fn handle(&mut self, _command: TestCommand) -> Result<TestResponse, AppStatus> {
            Ok(TestResponse("Test executed".to_string()))
        }
    }

    #[tokio::test]
    async fn test_register_and_send_command() {
        // Given
        let mut mediator = Mediator::new();
        let handler = TestCommandHandler;

        mediator.register_handler(handler);

        // When
        let command = TestCommand;
        let response = mediator.send(command).await;

        // Then
        assert!(response.is_ok());
        assert_eq!(response.unwrap().0, "Test executed");
    }

    #[tokio::test]
    async fn test_send_command_without_handler() {
        // Given
        let mediator = Mediator::new();

        // When
        let command = TestCommand;
        let response = mediator.send(command).await;

        // Then
        assert!(response.is_err());
        match response {
            Err(AppStatus::InternalError(msg)) => assert_eq!(msg, "Handler not found".to_string()),
            _ => panic!("Expected InternalError with message 'Handler not found'"),
        }
    }
}
