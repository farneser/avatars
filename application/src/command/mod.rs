use crate::shared::error::AppError;
use async_trait::async_trait;
use std::error::Error;

pub mod user;

pub trait Command<REQUEST> {}

#[async_trait]
pub trait CommandHandler<REQUEST, RESPONSE>
where
    REQUEST: Command<RESPONSE>,
{
    async fn handle(&self, command: REQUEST) -> Result<RESPONSE, AppError>;
}