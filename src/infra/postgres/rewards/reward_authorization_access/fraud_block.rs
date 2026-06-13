use diesel_async::AsyncPgConnection;

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationAction, RewardAuthorizationError,
};

pub(in crate::infra::postgres::rewards) async fn can_manage_organization_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ManageOrganizationRewardFraudBlock,
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_manage_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ManageRewardFraudBlock,
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_manage_teacher_reward_fraud_block(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ManageTeacherRewardFraudBlock,
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_view_reward_fraud_blocks(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ViewRewardFraudBlocks,
    )
    .await
}
