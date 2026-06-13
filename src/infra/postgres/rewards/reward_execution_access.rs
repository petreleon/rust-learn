use diesel_async::AsyncPgConnection;

use crate::application::access_control::authorize_reward::{
    authorize_reward_action, RewardAuthorizationAction, RewardAuthorizationError,
};
use crate::infra::postgres::access_control::reward_authorization_store::PostgresRewardAuthorizationStore;

pub(super) async fn can_execute_reward_payout(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    let mut store = PostgresRewardAuthorizationStore::new(conn);
    authorize_reward_action(
        &mut store,
        actor_user_id,
        RewardAuthorizationAction::ExecuteRewardPayout,
    )
    .await
}
