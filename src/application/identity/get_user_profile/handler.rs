use crate::application::identity::get_user_profile::GetUserProfileCommand;
use crate::application::identity::ports::{UserProfileAccessStore, UserProfileStore};
use crate::application::identity::user_profile::{UserProfileError, UserProfileOutput};

pub async fn get_user_profile(
    store: &mut (impl UserProfileAccessStore + UserProfileStore),
    command: GetUserProfileCommand,
) -> Result<UserProfileOutput, UserProfileError> {
    if command.requester_user_id != command.target_user_id
        && !store.can_view_any_user(command.requester_user_id).await?
    {
        return Err(UserProfileError::Forbidden);
    }

    store.find_user(command.target_user_id).await
}

#[cfg(test)]
mod tests;
