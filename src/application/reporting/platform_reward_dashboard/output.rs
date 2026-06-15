use chrono::{DateTime, Utc};

use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::rewards::execution::RewardExecutionJobStatus;

use super::RewardReconciliationMismatchType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRewardDashboardOutput {
    pub teacher_applications: TeacherApplicationDashboardSummaryOutput,
    pub reward_candidates: RewardCandidateDashboardSummaryOutput,
    pub pending_amount_approval_count: i64,
    pub pending_amount_approvals: Vec<RewardCandidateDashboardRowOutput>,
    pub payout_failure_count: i64,
    pub payout_failures: Vec<RewardExecutionFailureRowOutput>,
    pub reconciliation_mismatch_count: i64,
    pub reconciliation_mismatches: Vec<RewardReconciliationMismatchRowOutput>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationDashboardSummaryOutput {
    pub total: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RewardCandidateDashboardSummaryOutput {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardCandidateDashboardRowOutput {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_organization_id: Option<i32>,
    pub event_type: RewardEventType,
    pub status: RewardCandidateStatus,
    pub approved_amount: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardExecutionFailureRowOutput {
    pub reward_execution_job_id: i64,
    pub reward_candidate_id: i64,
    pub status: RewardExecutionJobStatus,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardReconciliationMismatchRowOutput {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub status: RewardCandidateStatus,
    pub mismatch_type: RewardReconciliationMismatchType,
    pub approved_amount: Option<String>,
    pub updated_at: DateTime<Utc>,
}
