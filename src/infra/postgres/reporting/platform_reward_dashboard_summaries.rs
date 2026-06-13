use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::reporting::platform_reward_dashboard::{
    PlatformRewardDashboardError, RewardCandidateDashboardSummaryOutput,
    TeacherApplicationDashboardSummaryOutput,
};
use crate::db::schema::{reward_candidates, teacher_applications};
use crate::domain::rewards::candidate::status::{
    REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_AMOUNT_REJECTED, REWARD_STATUS_COMPLETED,
    REWARD_STATUS_FAILED, REWARD_STATUS_NEEDS_RECONCILIATION, REWARD_STATUS_NOTIFIED,
    REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED, REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::models::teacher_application::{
    TEACHER_APPLICATION_STATUS_APPROVED, TEACHER_APPLICATION_STATUS_NEEDS_CHANGES,
    TEACHER_APPLICATION_STATUS_REJECTED, TEACHER_APPLICATION_STATUS_SUBMITTED,
};

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
        match status.as_str() {
            REWARD_STATUS_PENDING_TEACHER_APPROVAL => summary.pending_teacher_approval = count,
            REWARD_STATUS_TEACHER_APPROVED => summary.teacher_approved = count,
            REWARD_STATUS_TEACHER_REJECTED => summary.teacher_rejected = count,
            REWARD_STATUS_AMOUNT_APPROVED => summary.amount_approved = count,
            REWARD_STATUS_AMOUNT_REJECTED => summary.amount_rejected = count,
            REWARD_STATUS_TOKEN_PENDING => summary.token_pending = count,
            REWARD_STATUS_TOKEN_CONFIRMED => summary.token_confirmed = count,
            REWARD_STATUS_WALLET_CREDITED => summary.wallet_credited = count,
            REWARD_STATUS_NOTIFIED => summary.notified = count,
            REWARD_STATUS_COMPLETED => summary.completed = count,
            REWARD_STATUS_NEEDS_RECONCILIATION => summary.needs_reconciliation = count,
            REWARD_STATUS_FAILED => summary.failed = count,
            _ => {}
        }
    }
    Ok(summary)
}

pub(super) fn map_diesel_error(error: diesel::result::Error) -> PlatformRewardDashboardError {
    PlatformRewardDashboardError::Database(error.to_string())
}
