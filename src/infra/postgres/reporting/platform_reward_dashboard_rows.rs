use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_reward_dashboard::{
    reward_candidate_dashboard_row, reward_execution_failure_row, PlatformRewardDashboardError,
    RewardCandidateDashboardRowFact, RewardCandidateDashboardRowOutput,
    RewardExecutionFailureRowFact, RewardExecutionFailureRowOutput,
};
use crate::db::schema::{reward_candidates, reward_execution_jobs};
use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::domain::rewards::candidate::status::{
    RewardCandidateStatus, REWARD_STATUS_TEACHER_APPROVED,
};
use crate::domain::rewards::execution::RewardExecutionJobStatus;
use crate::infra::postgres::models::reward_candidate::RewardCandidate;
use crate::infra::postgres::models::reward_execution_job::RewardExecutionJob;
use crate::infra::postgres::reporting::platform_reward_dashboard_summaries::map_diesel_error;

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
        .map(reward_candidate_fact)
        .map(|fact| fact.map(reward_candidate_dashboard_row))
        .collect::<Result<Vec<_>, _>>()?;
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
        .map(reward_execution_failure_fact)
        .map(|fact| fact.map(reward_execution_failure_row))
        .collect::<Result<Vec<_>, _>>()?;
    let count = reward_execution_jobs::table
        .filter(reward_execution_jobs::status.eq(failed))
        .count()
        .get_result(conn)
        .await
        .map_err(map_diesel_error)?;
    Ok((rows, count))
}

fn reward_candidate_fact(
    candidate: RewardCandidate,
) -> Result<RewardCandidateDashboardRowFact, PlatformRewardDashboardError> {
    let event_type = RewardEventType::parse(&candidate.event_type)
        .map_err(|error| PlatformRewardDashboardError::Database(error.to_string()))?;
    let status = RewardCandidateStatus::parse(&candidate.status)
        .map_err(|error| PlatformRewardDashboardError::Database(error.to_string()))?;

    Ok(RewardCandidateDashboardRowFact {
        reward_candidate_id: candidate.id,
        course_id: candidate.course_id,
        student_user_id: candidate.student_user_id,
        submitter_user_id: candidate.submitter_user_id,
        source_organization_id: candidate.source_organization_id,
        event_type,
        status,
        approved_amount: candidate.approved_amount,
        updated_at: candidate.updated_at,
    })
}

fn reward_execution_failure_fact(
    job: RewardExecutionJob,
) -> Result<RewardExecutionFailureRowFact, PlatformRewardDashboardError> {
    let status = RewardExecutionJobStatus::parse(&job.status)
        .map_err(|error| PlatformRewardDashboardError::Database(error.to_string()))?;

    Ok(RewardExecutionFailureRowFact {
        reward_execution_job_id: job.id,
        reward_candidate_id: job.reward_candidate_id,
        status,
        attempts: job.attempts,
        last_error: job.last_error,
        updated_at: job.updated_at,
    })
}
