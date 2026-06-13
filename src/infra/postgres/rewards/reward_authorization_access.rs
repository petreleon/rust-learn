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

pub(super) async fn can_approve_student_reward_candidate(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ApproveStudentRewardCandidate { course_id },
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

pub(super) async fn can_manage_course_reward_rules(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ManageCourseRewardRules { course_id },
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

pub(super) async fn can_submit_course_reward_event(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::SubmitCourseRewardEvent { course_id },
    )
    .await
}

pub(super) async fn can_submit_organization_course_reward_event(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::SubmitOrganizationCourseRewardEvent { organization_id },
    )
    .await
}

pub(super) async fn can_view_course_reward_status(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ViewCourseRewardStatus { course_id },
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
