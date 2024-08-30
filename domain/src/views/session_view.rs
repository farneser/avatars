use crate::models::session::Session;
use crate::views::user_view::UserView;

#[derive(Debug)]
pub struct SessionView {
    pub value: String,
    pub user: UserView,
    pub created_at: String,
    pub expired_at: String,
}

impl SessionView {
    pub fn new(session: Session, user: UserView) -> Self {
        Self {
            value: session.value,
            user,
            created_at: session.created_at.to_string(),
            expired_at: session.expired_at.to_string(),
        }
    }
}
