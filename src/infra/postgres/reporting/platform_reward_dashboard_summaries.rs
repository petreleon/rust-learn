use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_reward_dashboard::{
    record_reward_candidate_status_count, PlatformRewardDashboardError,
    RewardCandidateDashboardSummaryOutput, TeacherApplicationDashboardSummaryOutput,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::teacher_applications::status::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use crate::infra::postgres::schema::{reward_candidates, teacher_applications};

pub(super) async fn teacher_application_dashboard_summary(
    conn: &mut AsyncPgConnection,
) -> Result<TeacherApplicationDashboardSummaryOutput, PlatformRewardDashboardError> {
    let rows = teacher_applications::table
        .group_by(teacher_applications::status)
        .select((teacher_applications::status, diesel::dsl::count_star()))
        .load::<(String, i64)>(conn)
        .await
        .map_err(map_diesel_error)?;

    let mut summary = TeacherApplicationDashboardSummaryOutput::default();
    for (status, count) in rows {
        summary.total += count;
        match status.as_str() {
            TEACHER_APPLICATION_STATUS_SUBMITTED => summary.submitted = count,
            TEACHER_APPLICATION_STATUS_NEEDS_CHANGES => summary.needs_changes = count,
            TEACHER_APPLICATION_STATUS_APPROVED => summary.approved = count,
            TEACHER_APPLICATION_STATUS_REJECTED => summary.rejected = count,
            _ => {}
        }
    }
    Ok(summary)
}

pub(super) async fn reward_candidate_dashboard_summary(
    conn: &mut AsyncPgConnection,
) -> Result<RewardCandidateDashboardSummaryOutput, PlatformRewardDashboardError> {
    let rows = reward_candidates::table
        .group_by(reward_candidates::status)
        .select((reward_candidates::status, diesel::dsl::count_star()))
        .load::<(String, i64)>(conn)
        .await
        .map_err(map_diesel_error)?;

    let mut summary = RewardCandidateDashboardSummaryOutput::default();
    for (status, count) in rows {
        summary.total += count;
        if let Ok(status) = RewardCandidateStatus::parse(&status) {
            record_reward_candidate_status_count(&mut summary, status, count);
        }
    }
    Ok(summary)
}

pub(super) fn map_diesel_error(error: diesel::result::Error) -> PlatformRewardDashboardError {
    PlatformRewardDashboardError::Database(error.to_string())
}
