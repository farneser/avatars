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

#[cfg(test)]
mod tests {
    use super::*;
    fn create_test_otp(id: &str, user_id: i64, created_at: i64, expires_at: i64) -> Otp {
        Otp {
            id: id.to_string(),
            user_id,
            created_at: DateTime::from_timestamp(created_at, 0).unwrap(),
            expires_at: DateTime::from_timestamp(expires_at, 0).unwrap(),
        }
    }

    #[tokio::test]
    async fn test_save() {
        // Given
        let mut repo = InMemoryOtpRepository::new();
        let otp = create_test_otp("1", 123, 1627846261, 1627849861);

        // When
        let result = repo.save(otp.clone()).await;

        // Then
        assert!(result.is_ok());
        assert_eq!(repo.store.get("1"), Some(&(123, 1627846261, 1627849861)));
    }

    #[tokio::test]
    async fn test_find_by_id() {
        // Given
        let mut repo = InMemoryOtpRepository::new();
        let otp = create_test_otp("1", 123, 1627846261, 1627849861);
        repo.save(otp.clone()).await.unwrap();

        // When
        let found = repo.find_by_id("1").await;

        // Then
        assert!(found.is_some());
        let found_otp = found.unwrap();
        assert_eq!(found_otp.id, "1");
        assert_eq!(found_otp.user_id, 123);
        assert_eq!(found_otp.created_at.timestamp(), 1627846261);
        assert_eq!(found_otp.expires_at.timestamp(), 1627849861);
    }

    #[tokio::test]
    async fn test_find_by_id_not_found() {
        // Given
        let repo = InMemoryOtpRepository::new();

        // When
        let found = repo.find_by_id("1").await;

        // Then
        assert!(found.is_none());
    }

    #[tokio::test]
    async fn test_find_by_id_expired() {
        // Given
        let mut repo = InMemoryOtpRepository::new();
        let otp = create_test_otp("1", 123, 1627846261, 1627849861);
        repo.save(otp.clone()).await.unwrap();

        // When
        let found = repo.find_by_id("1").await;

        // Then
        assert!(found.is_some());
        let found_otp = found.unwrap();
        assert_eq!(found_otp.id, "1");
        assert_eq!(found_otp.user_id, 123);
        assert_eq!(found_otp.created_at.timestamp(), 1627846261);
        assert_eq!(found_otp.expires_at.timestamp(), 1627849861);
    }

    #[tokio::test]
    async fn test_delete() {
        // Given
        let mut repo = InMemoryOtpRepository::new();
        let otp = create_test_otp("1", 123, 1627846261, 1627849861);
        repo.save(otp.clone()).await.unwrap();

        // When
        let result = repo.delete("1").await;

        // Then
        assert!(result.is_ok());
        assert!(repo.store.get("1").is_none());
    }
}