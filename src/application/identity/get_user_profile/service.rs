use futures::future::BoxFuture;

use crate::application::identity::get_user_profile::GetUserProfileCommand;
use crate::application::identity::user_profile::{UserProfileError, UserProfileOutput};

pub trait UserProfileReadUseCase: Send + Sync {
    fn get_user_profile(
        &self,
        command: GetUserProfileCommand,
    ) -> BoxFuture<'_, Result<UserProfileOutput, UserProfileError>>;
}
