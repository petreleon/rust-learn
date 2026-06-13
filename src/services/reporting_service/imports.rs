use crate::db::schema::{
    delegated_permissions, external_transactions, internal_transactions, reward_candidates,
    reward_execution_jobs, reward_payout_records, reward_wallet_credit_records,
    teacher_applications, wallets,
};
use crate::domain::rewards::execution::RewardExecutionJobStatus;
use crate::models::delegated_permission::DelegatedPermission;
use crate::models::reward_candidate::{
    RewardCandidate, REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_AMOUNT_REJECTED,
    REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED, REWARD_STATUS_NEEDS_RECONCILIATION,
    REWARD_STATUS_NOTIFIED, REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED, REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::models::reward_execution_job::RewardExecutionJob;
use crate::models::reward_payout_record::RewardPayoutRecord;
use crate::models::reward_wallet_credit_record::RewardWalletCreditRecord;
use crate::models::teacher_application::{
    TeacherApplication, TEACHER_APPLICATION_STATUS_APPROVED,
    TEACHER_APPLICATION_STATUS_NEEDS_CHANGES, TEACHER_APPLICATION_STATUS_REJECTED,
    TEACHER_APPLICATION_STATUS_SUBMITTED,
};
use crate::models::wallet::Wallet;
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct PlatformReportSummary {
    pub total_users: i64,
    pub total_organizations: i64,
    pub total_courses: i64,
    pub total_wallets: i64,
    pub total_notifications: i64,
}

#[derive(Debug, Serialize)]
pub struct PlatformRewardDashboard {
    pub teacher_applications: TeacherApplicationDashboardSummary,
    pub reward_candidates: RewardCandidateDashboardSummary,
    pub pending_amount_approval_count: i64,
    pub pending_amount_approvals: Vec<RewardCandidateDashboardRow>,
    pub payout_failure_count: i64,
    pub payout_failures: Vec<RewardExecutionFailureRow>,
    pub reconciliation_mismatch_count: i64,
    pub reconciliation_mismatches: Vec<RewardReconciliationMismatchRow>,
}

#[derive(Debug, Default, Serialize)]
pub struct TeacherApplicationDashboardSummary {
    pub total: i64,
    pub submitted: i64,
    pub needs_changes: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[derive(Debug, Default, Serialize)]
pub struct RewardCandidateDashboardSummary {
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
