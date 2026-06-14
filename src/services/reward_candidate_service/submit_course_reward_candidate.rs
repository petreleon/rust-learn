use crate::domain::rewards::candidate::source::REWARD_SOURCE_COURSE;
use crate::models::reward_candidate::RewardCandidate;
use diesel_async::AsyncPgConnection;

use super::create_reward_candidate::create_reward_candidate;
use super::ensure_active_reward_policy::ensure_course_submission_permission;
use super::ensure_no_active_reward_fraud_block::ensure_course_exists;
use super::support::{RewardCandidateError, SubmitRewardCandidateRequest};

pub async fn submit_course_reward_candidate(
    conn: &mut AsyncPgConnection,
    actor_user_id: i32,
    course_id: i32,
    request: SubmitRewardCandidateRequest,
) -> Result<RewardCandidate, RewardCandidateError> {
    ensure_course_exists(conn, course_id).await?;
    ensure_course_submission_permission(conn, actor_user_id, course_id).await?;
    create_reward_candidate(
        conn,
        actor_user_id,
        course_id,
        None,
        REWARD_SOURCE_COURSE,
        request,
    )
    .await
}
