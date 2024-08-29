use crate::command::{Command, CommandHandler};
use crate::shared::error::AppStatus;
use crate::shared::error::AppStatus::{AuthError, BadRequest};
use async_trait::async_trait;
use domain::models::otp::{Otp, OTP_LENGTH};
use domain::models::user::User;
use domain::repositories::id_provider::IdProvider;
use domain::repositories::otp_repository::OtpRepository;
use domain::repositories::session_repository::SessionRepository;
use domain::repositories::user_repository::UserRepository;
use domain::repositories::DbError;
use domain::services::mail_service::MailService;
use domain::services::user_service::UserService;
use log::info;
use std::sync::Arc;

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

impl Command<User> for LoginUserCommand {}

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

    async fn process_user(&mut self, command: LoginUserCommand) -> Result<User, AppStatus> {
        let user_result = self.user_service.find_by_login(&command.login).await;

        if let Ok(user) = user_result {
            return Ok(user);
        }

        let new_user = User::new(command.login);

        match self.user_service.save(new_user).await {
            Ok(user) => Ok(user),
            Err(err) => {
                info!("{}", err);
                Err(AppStatus::InternalError(err))
            }
        }
    }
}

#[async_trait]
impl<UR, SR, OR, IP, MS> CommandHandler<LoginUserCommand, User> for LoginUserCommandHandler<UR, SR, OR, IP, MS>
where
    UR: UserRepository + Sync + Send,
    SR: SessionRepository + Sync + Send,
    OR: OtpRepository + Sync + Send,
    IP: IdProvider + Sync + Send,
    MS: MailService + Sync + Send,
{
    async fn handle(&mut self, command: LoginUserCommand) -> Result<User, AppStatus> {
        if command.login.is_empty() {
            return Err(BadRequest("Username is required".to_string()));
        }

        let user = match self.process_user(command.clone()).await {
            Ok(u) => u,
            Err(err) => return Err(err),
        };

        if command.otp.is_none() {
            let otp = match self.user_service.save_otp(user.id).await {
                Ok(otp) => otp,
                Err(err) => {
                    return Err(AppStatus::InternalError(format!("Failed to save OTP: {}", err)));
                }
            };

            // FIXME Change otp to be a DTO

            let temp_otp_var = Otp::new(otp, user.id, 300);

            if let Err(err) = self.mail_service.send_otp(&user.username, temp_otp_var.unwrap()).await {
                return Err(AppStatus::InternalError(format!("Failed to send OTP: {}", err)));
            }

            return Err(AppStatus::Ok("OTP sent to email".to_string()));
        }

        let otp = command.otp.unwrap();

        match self.user_service.validate_otp(&user.username, &otp).await {
            Ok(u) => Ok(u),
            Err((err)) => Err(AuthError(err)),
        }
    }
}
