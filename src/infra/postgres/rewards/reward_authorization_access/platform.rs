use diesel_async::AsyncPgConnection;

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationAction, RewardAuthorizationError,
};

pub(in crate::infra::postgres::rewards) async fn can_approve_reward_amount(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ApproveRewardAmount,
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_execute_reward_payout(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ExecuteRewardPayout,
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_manage_reward_policy(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ManageRewardPolicy,
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_record_reward_compensation(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::RecordRewardCompensation,
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_view_reward_audit(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ViewRewardAudit,
    )
    .await
}
