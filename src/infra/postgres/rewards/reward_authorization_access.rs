use diesel_async::AsyncPgConnection;

use crate::application::access_control::authorize_reward::{
    authorize_reward_action, RewardAuthorizationAction, RewardAuthorizationError,
};
use crate::infra::postgres::access_control::reward_authorization_store::PostgresRewardAuthorizationStore;

pub(super) async fn can_approve_reward_amount(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ApproveRewardAmount,
    )
    .await
}

pub(super) async fn can_execute_reward_payout(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ExecuteRewardPayout,
    )
    .await
}

pub(super) async fn can_manage_reward_policy(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ManageRewardPolicy,
    )
    .await
}

pub(super) async fn can_record_reward_compensation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::RecordRewardCompensation,
    )
    .await
}

pub(super) async fn can_view_reward_audit(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ViewRewardAudit,
    )
    .await
}

async fn authorize(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    action: RewardAuthorizationAction,
) -> Result<bool, RewardAuthorizationError> {
    let mut store = PostgresRewardAuthorizationStore::new(conn);
    authorize_reward_action(&mut store, actor_user_id, action).await
}
