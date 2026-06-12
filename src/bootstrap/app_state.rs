use crate::db::DbPool;
use crate::utils::notifications::NotificationsState;
use crate::utils::s3_utils::S3State;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub s3: S3State,
    pub notifications: NotificationsState,
}
