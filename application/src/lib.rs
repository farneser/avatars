pub mod command;
pub mod shared;
pub mod mediator;

use crate::command::Command;
use crate::mediator::Mediator;
use crate::shared::error::AppStatus;
use domain::repositories::id_provider::IdProvider;
use domain::repositories::otp_repository::OtpRepository;
use domain::repositories::session_repository::SessionRepository;
use domain::repositories::user_repository::UserRepository;
use domain::services::mail_service::MailService;

pub struct AppContainer {
    mediator: Mediator,
}

impl AppContainer {
    pub fn new(
        user_repository: impl UserRepository + Sync + Send + 'static,
        session_repository: impl SessionRepository + Sync + Send + 'static,
        otp_repository: impl OtpRepository + Sync + Send + 'static,
        id_provider: impl IdProvider + Sync + Send + 'static,
        mail_service: impl MailService + Sync + Send + 'static,
    ) -> Self {
        let mediator = build_mediator(
            user_repository,
            session_repository,
            otp_repository,
            id_provider,
            mail_service,
        );

        Self { mediator }
    }

    pub fn new_from_mediator(mediator: Mediator) -> Self {
        Self { mediator }
    }

    pub async fn send_command<REQUEST, RESPONSE>(&self, command: REQUEST) -> Result<RESPONSE, AppStatus>
    where
        REQUEST: Command<RESPONSE> + 'static,
        RESPONSE: 'static,
    {
        self.mediator.send::<REQUEST, RESPONSE>(command).await
    }
}

pub fn build_mediator<UR, SR, OR, IP, MS>(
    user_repository: UR,
    session_repository: SR,
    otp_repository: OR,
    id_provider: IP,
    mail_service: MS,
) -> Mediator
where
    UR: UserRepository + Sync + Send + 'static,
    SR: SessionRepository + Sync + Send + 'static,
    OR: OtpRepository + Sync + Send + 'static,
    IP: IdProvider + Sync + Send + 'static,
    MS: MailService + Sync + Send + 'static,
{
    let login_ch = command::user::login_user::LoginUserCommandHandler::new(
        user_repository,
        session_repository,
        otp_repository,
        id_provider,
        mail_service,
    );

    let mut mediator = Mediator::new();

    mediator.register_handler(login_ch);

    mediator
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::user::login_user::LoginUserCommand;
    use crate::command::CommandHandler;
    use async_trait::async_trait;
    use domain::repositories::id_provider::SimpleIdProvider;
    use domain::repositories::otp_repository::InMemoryOtpRepository;
    use domain::repositories::session_repository::InMemorySessionRepository;
    use domain::repositories::user_repository::InMemoryUserRepository;
    use domain::services::mail_service::InMemoryMailService;

    struct TestCommand;
    struct TestResponse(pub String);
    impl Command<TestResponse> for TestCommand {}

    struct TestCommandHandler;
    #[async_trait]
    impl CommandHandler<TestCommand, TestResponse> for TestCommandHandler {
        async fn handle(&mut self, _command: TestCommand) -> Result<TestResponse, AppStatus> {
            Ok(TestResponse("Command result".to_string()))
        }
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
    }

    #[tokio::test]
    async fn test_send_command_with_handler() {
        // Given
        let user_repository = InMemoryUserRepository::new();
        let session_repository = InMemorySessionRepository::new();
        let otp_repository = InMemoryOtpRepository::new();
        let id_provider = SimpleIdProvider::new();
        let mail_service = InMemoryMailService::new();

        let app_container = AppContainer::new(
            user_repository,
            session_repository,
            otp_repository,
            id_provider,
            mail_service,
        );

        let command = LoginUserCommand::new("user".to_string(), Some("password".to_string()));

        // When
        let response = app_container.send_command(command).await;

        // Then
        match response {
            Err(AppStatus::AuthError(_)) => { assert!(true) }
            _ => { assert!(false); }
        }
    }
}