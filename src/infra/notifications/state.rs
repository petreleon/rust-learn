use crate::db::DbPool;

#[derive(Clone)]
pub struct NotificationsState {
    pub(super) pool: DbPool,
}

impl NotificationsState {
    /// Create a new NotificationsState from an existing DB pool.
    pub fn new(pool: DbPool) -> Self {
        NotificationsState { pool }
    }
}

impl From<DbPool> for NotificationsState {
    fn from(pool: DbPool) -> Self {
        NotificationsState::new(pool)
    }
}
