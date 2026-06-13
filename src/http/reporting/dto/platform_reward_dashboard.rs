use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::reporting::platform_reward_dashboard::{
    PlatformRewardDashboardOutput, RewardCandidateDashboardRowOutput,
    RewardCandidateDashboardSummaryOutput, RewardExecutionFailureRowOutput,
    RewardReconciliationMismatchRowOutput, TeacherApplicationDashboardSummaryOutput,
};

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardDashboardResponse {
    pub teacher_applications: PlatformTeacherApplicationDashboardSummaryResponse,
    pub reward_candidates: RewardCandidateDashboardSummaryResponse,
    pub pending_amount_approval_count: i64,
    pub pending_amount_approvals: Vec<RewardCandidateDashboardRowResponse>,
    pub payout_failure_count: i64,
    pub payout_failures: Vec<RewardExecutionFailureRowResponse>,
    pub reconciliation_mismatch_count: i64,
    pub reconciliation_mismatches: Vec<RewardReconciliationMismatchRowResponse>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct PlatformTeacherApplicationDashboardSummaryResponse {
    pub total: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct RewardCandidateDashboardSummaryResponse {
    pub total: i64,
    pub pending_teacher_approval: i64,
    pub teacher_approved: i64,
    pub teacher_rejected: i64,
    pub amount_approved: i64,
    pub amount_rejected: i64,
    pub token_pending: i64,
    pub token_confirmed: i64,
    pub wallet_credited: i64,
    pub notified: i64,
    pub completed: i64,
    pub needs_reconciliation: i64,
    pub failed: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardCandidateDashboardRowResponse {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_organization_id: Option<i32>,
    pub event_type: String,
    pub status: String,
    pub approved_amount: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardExecutionFailureRowResponse {
    pub reward_execution_job_id: i64,
    pub reward_candidate_id: i64,
    pub status: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RewardReconciliationMismatchRowResponse {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub status: String,
    pub mismatch_type: String,
    pub approved_amount: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<PlatformRewardDashboardOutput> for PlatformRewardDashboardResponse {
    fn from(output: PlatformRewardDashboardOutput) -> Self {
        Self {
            teacher_applications: output.teacher_applications.into(),
            reward_candidates: output.reward_candidates.into(),
            pending_amount_approval_count: output.pending_amount_approval_count,
            pending_amount_approvals: output
                .pending_amount_approvals
                .into_iter()
                .map(Into::into)
                .collect(),
            payout_failure_count: output.payout_failure_count,
            payout_failures: output.payout_failures.into_iter().map(Into::into).collect(),
            reconciliation_mismatch_count: output.reconciliation_mismatch_count,
            reconciliation_mismatches: output
                .reconciliation_mismatches
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

impl From<TeacherApplicationDashboardSummaryOutput>
    for PlatformTeacherApplicationDashboardSummaryResponse
{
    fn from(summary: TeacherApplicationDashboardSummaryOutput) -> Self {
        Self {
            total: summary.total,
            submitted: summary.submitted,
            needs_changes: summary.needs_changes,
            approved: summary.approved,
            rejected: summary.rejected,
        }
    }
}

impl From<RewardCandidateDashboardSummaryOutput> for RewardCandidateDashboardSummaryResponse {
    fn from(summary: RewardCandidateDashboardSummaryOutput) -> Self {
        Self {
            total: summary.total,
            pending_teacher_approval: summary.pending_teacher_approval,
            teacher_approved: summary.teacher_approved,
            teacher_rejected: summary.teacher_rejected,
            amount_approved: summary.amount_approved,
            amount_rejected: summary.amount_rejected,
            token_pending: summary.token_pending,
            token_confirmed: summary.token_confirmed,
            wallet_credited: summary.wallet_credited,
            notified: summary.notified,
            completed: summary.completed,
            needs_reconciliation: summary.needs_reconciliation,
            failed: summary.failed,
        }
    }
}

impl From<RewardCandidateDashboardRowOutput> for RewardCandidateDashboardRowResponse {
    fn from(row: RewardCandidateDashboardRowOutput) -> Self {
        Self {
            reward_candidate_id: row.reward_candidate_id,
            course_id: row.course_id,
            student_user_id: row.student_user_id,
            submitter_user_id: row.submitter_user_id,
            source_organization_id: row.source_organization_id,
            event_type: row.event_type,
            status: row.status,
            approved_amount: row.approved_amount,
            updated_at: row.updated_at,
        }
    }
}

impl From<RewardExecutionFailureRowOutput> for RewardExecutionFailureRowResponse {
    fn from(row: RewardExecutionFailureRowOutput) -> Self {
        Self {
            reward_execution_job_id: row.reward_execution_job_id,
            reward_candidate_id: row.reward_candidate_id,
            status: row.status,
            attempts: row.attempts,
            last_error: row.last_error,
            updated_at: row.updated_at,
        }
    }
}

impl From<RewardReconciliationMismatchRowOutput> for RewardReconciliationMismatchRowResponse {
    fn from(row: RewardReconciliationMismatchRowOutput) -> Self {
        Self {
            reward_candidate_id: row.reward_candidate_id,
            course_id: row.course_id,
            student_user_id: row.student_user_id,
            status: row.status,
            mismatch_type: row.mismatch_type,
            approved_amount: row.approved_amount,
            updated_at: row.updated_at,
        }
    }
}
