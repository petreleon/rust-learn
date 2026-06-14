use crate::application::identity::list_users::ListUsersQuery;
use crate::application::identity::ports::UserProfileStore;
use crate::application::identity::user_profile::{UserProfileError, UserProfileOutput};

pub async fn list_users(
    store: &mut impl UserProfileStore,
    query: ListUsersQuery,
) -> Result<Vec<UserProfileOutput>, UserProfileError> {
    store.list_users(query).await
}
