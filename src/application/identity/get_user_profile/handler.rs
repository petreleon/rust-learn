use crate::application::identity::ports::UserProfileStore;
use crate::application::identity::user_profile::{UserProfileError, UserProfileOutput};

pub async fn get_user_profile(
    store: &mut impl UserProfileStore,
    user_id: i32,
) -> Result<UserProfileOutput, UserProfileError> {
    store.find_user(user_id).await
}
