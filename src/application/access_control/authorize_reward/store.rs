use futures::future::BoxFuture;

use crate::application::access_control::authorize_reward::RewardAuthorizationError;
use crate::application::access_control::check_permission::{
    AccessAction, AccessActor, AccessScope,
};

pub trait RewardAuthorizationStore {
    fn can(
        &mut self,
        actor: AccessActor,
        action: AccessAction,
        scope: AccessScope,
    ) -> BoxFuture<'_, Result<bool, RewardAuthorizationError>>;
}
