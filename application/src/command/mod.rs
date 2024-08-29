use crate::shared::error::AppStatus;
use async_trait::async_trait;

pub mod user;

pub trait Command<REQUEST> {}

#[async_trait]
pub trait CommandHandler<REQUEST, RESPONSE>
where
    REQUEST: Command<RESPONSE>,
{
    async fn handle(&mut self, command: REQUEST) -> Result<RESPONSE, AppStatus>;
}