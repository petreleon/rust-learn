use diesel_async::AsyncPgConnection;

use crate::application::access_control::authorize_reward::{
    RewardAuthorizationAction, RewardAuthorizationError,
};

pub(in crate::infra::postgres::rewards) async fn can_approve_student_reward_candidate(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ApproveStudentRewardCandidate { course_id },
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_manage_course_reward_rules(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ManageCourseRewardRules { course_id },
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_submit_course_reward_event(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::SubmitCourseRewardEvent { course_id },
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_submit_organization_course_reward_event(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    organization_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::SubmitOrganizationCourseRewardEvent { organization_id },
    )
    .await
}

pub(in crate::infra::postgres::rewards) async fn can_view_course_reward_status(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
) -> Result<bool, RewardAuthorizationError> {
    super::authorize(
        conn,
        actor_user_id,
        RewardAuthorizationAction::ViewCourseRewardStatus { course_id },
    )
    .await
}
