use crate::command::{Command, CommandHandler};
use crate::shared::error::AppStatus;
use crate::shared::error::AppStatus::{AuthError, BadRequest};
use async_trait::async_trait;
use domain::repositories::id_provider::IdProvider;
use domain::repositories::otp_repository::OtpRepository;
use domain::repositories::session_repository::SessionRepository;
use domain::repositories::user_repository::UserRepository;
use domain::services::mail_service::MailService;
use domain::services::user_service::UserService;
use log::info;
use domain::views::user_view::UserView;

#[derive(Debug, Clone)]
pub struct LoginUserCommand {
    login: String,
    otp: Option<String>,
}

impl LoginUserCommand {
    pub fn new(username: String, otp: Option<String>) -> Self {
        Self { login: username, otp }
    }
}

impl Command<UserView> for LoginUserCommand {}

pub struct LoginUserCommandHandler<UR, SR, OR, IP, MS>
where
    UR: UserRepository + Sync + Send,
    SR: SessionRepository + Sync + Send,
    OR: OtpRepository + Sync + Send,
    IP: IdProvider + Sync + Send,
    MS: MailService + Sync + Send,
{
    user_service: UserService<UR, SR, OR, IP>,
    mail_service: MS,
}

impl<UR, SR, OR, IP, MS> LoginUserCommandHandler<UR, SR, OR, IP, MS>
where
    UR: UserRepository + Sync + Send,
    SR: SessionRepository + Sync + Send,
    OR: OtpRepository + Sync + Send,
    IP: IdProvider + Sync + Send,
    MS: MailService + Sync + Send,
{
    pub fn new(user_repository: UR, session_repository: SR, otp_repository: OR, otp_id_provider: IP, mail_service: MS) -> Self {
        let user_service = UserService::new(user_repository, session_repository, otp_repository, otp_id_provider);

        Self {
            user_service,
            mail_service,
        }
    }

    async fn process_user(&mut self, command: LoginUserCommand) -> Result<UserView, AppStatus> {
        let user_result = self.user_service.find_by_login(&command.login).await;

        if let Ok(user) = user_result {
            return Ok(user);
        }

        match self.user_service.create(command.login).await {
            Ok(user) => Ok(user),
            Err(err) => {
                info!("{}", err);
                Err(AppStatus::InternalError(err))
            }
        }
    }
}

#[async_trait]
impl<UR, SR, OR, IP, MS> CommandHandler<LoginUserCommand, UserView> for LoginUserCommandHandler<UR, SR, OR, IP, MS>
where
    UR: UserRepository + Sync + Send,
    SR: SessionRepository + Sync + Send,
    OR: OtpRepository + Sync + Send,
    IP: IdProvider + Sync + Send,
    MS: MailService + Sync + Send,
{
    async fn handle(&mut self, command: LoginUserCommand) -> Result<UserView, AppStatus> {
        if command.login.is_empty() {
            return Err(BadRequest("Username is required".to_string()));
        }

        let user_view = match self.process_user(command.clone()).await {
            Ok(u) => u,
            Err(err) => return Err(err),
        };

        if command.otp.is_none() {
            let otp_view = match self.user_service.save_otp(user_view.id).await {
                Ok(otp) => otp,
                Err(err) => {
                    return Err(AppStatus::InternalError(format!("Failed to save OTP: {}", err)));
                }
            };

            if let Err(err) = self.mail_service.send_otp(&user_view.username, otp_view).await {
                return Err(AppStatus::InternalError(format!("Failed to send OTP: {}", err)));
            }

            return Err(AppStatus::Ok("OTP sent to email".to_string()));
        }

        let otp_code = command.otp.unwrap();

        match self.user_service.validate_otp(&user_view.username, &otp_code).await {
            Ok(u) => Ok(u),
            Err((err)) => Err(AuthError(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::repositories::id_provider::SimpleIdProvider;
    use domain::repositories::OTP_LENGTH;
    use domain::repositories::otp_repository::InMemoryOtpRepository;
    use domain::repositories::session_repository::InMemorySessionRepository;
    use domain::repositories::user_repository::InMemoryUserRepository;
    use domain::services::mail_service::InMemoryMailService;

    #[tokio::test]
    async fn test_handle_with_empty_username() {
        // Given
        let ur = InMemoryUserRepository::new();
        let sr = InMemorySessionRepository::new();
        let or = InMemoryOtpRepository::new();
        let ip = SimpleIdProvider::new();
        let ms = InMemoryMailService::new();

        let mut handler = LoginUserCommandHandler::new(ur, sr, or, ip, ms);
        let command = LoginUserCommand::new("".to_string(), None);

        // When
        let result = handler.handle(command).await;

        // Then
        assert!(matches!(result, Err(BadRequest(_))));
    }

    #[tokio::test]
    async fn test_handle_with_valid_username_and_no_otp() {
        // Given
        let ur = InMemoryUserRepository::new();
        let sr = InMemorySessionRepository::new();
        let or = InMemoryOtpRepository::new();
        let ip = SimpleIdProvider::new();
        let ms = InMemoryMailService::new();

        let mut handler = LoginUserCommandHandler::new(ur, sr, or, ip, ms);
        let command = LoginUserCommand::new("test_user".to_string(), None);

        // When
        let result = handler.handle(command).await;

        // Then
        assert!(matches!(result, Err(AppStatus::Ok(_))));
    }

    #[derive(Debug, Clone)]
    struct TestIdProvider {}

    impl TestIdProvider {
        fn new() -> Self {
            Self {}
        }
    }

    #[async_trait]
    impl IdProvider for TestIdProvider {
        fn get_id(&self, length: usize) -> String {
            self.get_numeric_id(length)
        }

        fn get_numeric_id(&self, length: usize) -> String {
            self.get_from_alphabet(vec![], length)
        }

        fn get_from_alphabet(&self, _: Vec<&str>, length: usize) -> String {
            (0..length).map(|i| i.to_string()).collect()
        }
    }

    #[tokio::test]
    async fn test_handle_with_valid_username_and_otp() {
        // Given
        let ur = InMemoryUserRepository::new();
        let sr = InMemorySessionRepository::new();
        let or = InMemoryOtpRepository::new();
        let ip = TestIdProvider::new();
        let ms = InMemoryMailService::new();

        let mut handler = LoginUserCommandHandler::new(ur, sr, or, ip.clone(), ms);
        let start_command = LoginUserCommand::new("test_user".to_string(), None);
        let command = LoginUserCommand::new("test_user".to_string(), Some(ip.get_numeric_id(OTP_LENGTH)));

        // When
        let _ = handler.handle(start_command).await;
        let result = handler.handle(command).await;

        // Then
        assert!(matches!(result, Ok(_)));
    }
}