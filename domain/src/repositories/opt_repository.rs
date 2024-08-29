use crate::models::otp::Otp;
use crate::repositories::DbError;
use async_trait::async_trait;
use chrono::DateTime;
use std::collections::HashMap;

#[async_trait]
pub trait OtpRepository {
    async fn save<'a>(&'a mut self, otp: Otp) -> Result<Otp, DbError>;
    async fn find_by_id<'a>(&'a self, id: &'a str) -> Option<Otp>;
    async fn delete<'a>(&'a mut self, id: &'a str) -> Result<(), DbError>;
}

pub struct InMemoryOtpRepository {
    store: HashMap<String, (i64, i64, i64)>,
}

impl InMemoryOtpRepository {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }
}

#[async_trait]
impl OtpRepository for InMemoryOtpRepository {
    async fn save<'a>(&'a mut self, otp: Otp) -> Result<Otp, DbError> {
        self.store.insert(otp.id.clone(), (otp.user_id.clone(), otp.created_at.timestamp(), otp.expires_at.timestamp()));

        Ok(otp)
    }

    async fn find_by_id<'a>(&'a self, id: &'a str) -> Option<Otp> {
        let otp = self.store.get(id);

        match otp {
            Some((user_id, created_at, expires_at)) => {
                let created = match DateTime::from_timestamp(*created_at, 0) {
                    None => {
                        return None;
                    }
                    Some(created) => created,
                };

                let expired = match DateTime::from_timestamp(*expires_at, 0) {
                    None => {
                        return None;
                    }
                    Some(expired) => expired,
                };

                if created > expired {
                    return None;
                }

                Some(Otp {
                    id: id.to_string(),
                    user_id: *user_id,
                    created_at: created,
                    expires_at: expired,
                })
            }
            None => None,
        }
    }

    async fn delete<'a>(&'a mut self, id: &'a str) -> Result<(), DbError> {
        self.store.remove(id);

        Ok(())
    }
}
