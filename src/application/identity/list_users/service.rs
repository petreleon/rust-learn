use futures::future::BoxFuture;

use crate::application::identity::list_users::ListUsersQuery;
use crate::application::identity::user_profile::{UserProfileError, UserProfileOutput};

pub trait UserListUseCase: Send + Sync {
    fn list_users(
        &self,
        query: ListUsersQuery,
    ) -> BoxFuture<'_, Result<Vec<UserProfileOutput>, UserProfileError>>;
}
