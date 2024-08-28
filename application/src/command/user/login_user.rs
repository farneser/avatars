use crate::command::{Command, CommandHandler};
use crate::shared::error::AppError;
use crate::shared::error::AppError::{AuthError, BadRequest};
use domain::models::otp::OTP_LENGTH;
use domain::models::user::User;
use domain::repositories::id_provider::IdProvider;
use domain::repositories::session_repository::SessionRepository;
use domain::repositories::user_repository::UserRepository;
use domain::services::mail_service::MailService;
use domain::services::user_service::UserService;
use log::info;
use std::sync::Arc;

pub struct LoginUserCommand {
    login: String,
    otp: Option<String>,
}

impl LoginUserCommand {
    pub fn new(username: String, otp: Option<String>) -> Self {
        Self { login: username, otp }
    }
}

impl Command<User> for LoginUserCommand {}

pub struct LoginUserCommandHandler<UR, SR, I, MS>
where
    UR: UserRepository,
    SR: SessionRepository,
    I: IdProvider,
    MS: MailService,
{
    user_service: UserService<UR, SR, I>,
    mail_service: Arc<MS>,
}

impl<UR, SR, I, MS> LoginUserCommandHandler<UR, SR, I, MS>
where
    UR: UserRepository,
    SR: SessionRepository,
    I: IdProvider,
    MS: MailService,
{
    pub fn new(user_repository: UR, session_repository: SR, otp_id_provider: I, mail_service: Arc<MS>) -> Self {
        let service = UserService::new(user_repository, session_repository, otp_id_provider);

        Self {
            user_service: service,
            mail_service,
        }
    }
}

impl<UR, SR, I, MS> CommandHandler<LoginUserCommand, User> for LoginUserCommandHandler<UR, SR, I, MS>
where
    UR: UserRepository,
    SR: SessionRepository,
    I: IdProvider,
    MS: MailService,
{
    async fn handle(&self, command: LoginUserCommand) -> Result<User, AppError> {
        if command.login.is_empty() {
            return Err(BadRequest("Username is required".to_string()));
        }

        let user = match self.user_service.find_by_login(&command.login).await {
            Ok(user) => user,
            Err(err) => {
                info!("{}", err);
                return match self.user_service.save(User::new(command.login)).await {
                    Ok(user) => Ok(user),
                    Err(msg) => Err(AppError::InternalError(msg)),
                };
            }
        };

        if command.otp.is_none() {
            let otp = self.user_service.generate_otp(&user).await;

            if let Err(err) = self.mail_service.send_otp(&user, &otp).await {
                return Err(AppError::InternalError(format!("Failed to send OTP: {}", err)));
            }

            return Err(BadRequest("OTP sent to email".to_string()));
        }

        let otp = command.otp.unwrap();
        if otp.len() != OTP_LENGTH {
            return Err(BadRequest("OTP must be 8 characters".to_string()));
        }

        if !self.user_service.validate_otp(&user.username, &otp).await {
            return Err(AuthError("Invalid OTP".to_string()));
        }

        Ok(user)
    }
}
