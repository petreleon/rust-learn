use futures::future::BoxFuture;

use crate::application::identity::current_session::{CurrentSessionError, CurrentSessionOutput};
use crate::application::identity::list_users::ListUsersQuery;
use crate::application::identity::user_profile::{UserProfileError, UserProfileOutput};

pub trait CurrentSessionStore {
    fn load_current_session(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<CurrentSessionOutput, CurrentSessionError>>;
}

pub trait UserProfileStore {
    fn list_users(
        &mut self,
        query: ListUsersQuery,
    ) -> BoxFuture<'_, Result<Vec<UserProfileOutput>, UserProfileError>>;

    fn find_user(
        &mut self,
        user_id: i32,
    ) -> BoxFuture<'_, Result<UserProfileOutput, UserProfileError>>;
}
