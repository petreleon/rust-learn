use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessDecisionStore, AccessScope,
};
use crate::application::identity::get_user_profile::GetUserProfileCommand;
use crate::application::identity::ports::UserProfileStore;
use crate::application::identity::user_profile::{UserProfileError, UserProfileOutput};

const VIEW_USER: &str = "VIEW_USER";

pub async fn get_user_profile(
    store: &mut (impl AccessDecisionStore<Error = UserProfileError> + UserProfileStore),
    command: GetUserProfileCommand,
) -> Result<UserProfileOutput, UserProfileError> {
    if command.requester_user_id != command.target_user_id
        && !store
            .can(
                AccessActor::user(command.requester_user_id),
                AccessAction::permission(VIEW_USER),
                AccessScope::platform(),
            )
            .await?
    {
        return Err(UserProfileError::Forbidden);
    }

    store.find_user(command.target_user_id).await
}

#[cfg(test)]
mod tests;
