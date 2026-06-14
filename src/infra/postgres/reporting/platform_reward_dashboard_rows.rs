use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_reward_dashboard::{
    PlatformRewardDashboardError, RewardCandidateDashboardRowOutput,
    RewardExecutionFailureRowOutput,
};
use crate::db::schema::{reward_candidates, reward_execution_jobs};
use crate::domain::rewards::candidate::status::REWARD_STATUS_TEACHER_APPROVED;
use crate::domain::rewards::execution::RewardExecutionJobStatus;
use crate::infra::postgres::reporting::platform_reward_dashboard_summaries::map_diesel_error;
use crate::models::reward_candidate::RewardCandidate;
use crate::models::reward_execution_job::RewardExecutionJob;

pub(super) async fn pending_amount_approvals(
    conn: &mut AsyncPgConnection,
) -> Result<(Vec<RewardCandidateDashboardRowOutput>, i64), PlatformRewardDashboardError> {
    let rows = reward_candidates::table
        .filter(reward_candidates::status.eq(REWARD_STATUS_TEACHER_APPROVED))
        .order(reward_candidates::updated_at.desc())
        .limit(50)
        .load::<RewardCandidate>(conn)
        .await
        .map_err(map_diesel_error)?
        .into_iter()
        .map(map_candidate)
        .collect::<Vec<_>>();
    let count = reward_candidates::table
        .filter(reward_candidates::status.eq(REWARD_STATUS_TEACHER_APPROVED))
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)?;
    Ok((rows, count))
}

pub(super) async fn payout_failures(
    conn: &mut AsyncPgConnection,
) -> Result<(Vec<RewardExecutionFailureRowOutput>, i64), PlatformRewardDashboardError> {
    let failed = RewardExecutionJobStatus::Failed.as_str();
    let rows = reward_execution_jobs::table
        .filter(reward_execution_jobs::status.eq(failed))
        .order(reward_execution_jobs::updated_at.desc())
        .limit(50)
        .load::<RewardExecutionJob>(conn)
        .await
        .map_err(map_diesel_error)?
        .into_iter()
        .map(map_execution_failure)
        .collect::<Vec<_>>();
    let count = reward_execution_jobs::table
        .filter(reward_execution_jobs::status.eq(failed))
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)?;
    Ok((rows, count))
}

fn map_candidate(candidate: RewardCandidate) -> RewardCandidateDashboardRowOutput {
    RewardCandidateDashboardRowOutput {
        reward_candidate_id: candidate.id,
        course_id: candidate.course_id,
        student_user_id: candidate.student_user_id,
        submitter_user_id: candidate.submitter_user_id,
        source_organization_id: candidate.source_organization_id,
        event_type: candidate.event_type,
        status: candidate.status,
        approved_amount: candidate.approved_amount.as_ref().map(ToString::to_string),
        updated_at: candidate.updated_at,
    }
}

fn map_execution_failure(job: RewardExecutionJob) -> RewardExecutionFailureRowOutput {
    RewardExecutionFailureRowOutput {
        reward_execution_job_id: job.id,
        reward_candidate_id: job.reward_candidate_id,
        status: job.status,
        attempts: job.attempts,
        last_error: job.last_error,
        updated_at: job.updated_at,
    }
}
