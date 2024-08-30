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
        let mediator = how_to_name_this_shit(
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

pub fn how_to_name_this_shit<UR, SR, OR, IP, MS>(
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
