use crate::models::user::User;
use crate::repositories::DbError;

pub trait UserRepository {
    fn find_by_email(&self, email: &str) -> Option<User>;
    fn save(&self, user: User) -> Result<User, DbError>;
}