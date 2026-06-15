mod course;
mod fraud_block;
mod platform;

use diesel_async::AsyncPgConnection;

use crate::application::access_control::authorize_reward::{
    authorize_reward_action, RewardAuthorizationAction, RewardAuthorizationError,
};
use crate::infra::postgres::access_control::access_decision_store::PostgresAccessDecisionStore;

pub(super) use course::{
    can_approve_student_reward_candidate, can_manage_course_reward_rules,
    can_submit_course_reward_event, can_submit_organization_course_reward_event,
    can_view_course_reward_status,
};
pub(super) use fraud_block::{
    can_manage_organization_reward_fraud_block, can_manage_reward_fraud_block,
    can_manage_teacher_reward_fraud_block, can_view_reward_fraud_blocks,
};
pub(super) use platform::{
    can_approve_reward_amount, can_execute_reward_payout, can_manage_reward_policy,
    can_record_reward_compensation, can_view_reward_audit,
};

async fn authorize(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    action: RewardAuthorizationAction,
) -> Result<bool, RewardAuthorizationError> {
    let mut store = PostgresAccessDecisionStore::new(conn);
    authorize_reward_action(&mut store, actor_user_id, action).await
}
