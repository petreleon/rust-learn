use chrono::{DateTime, Utc};

use crate::domain::access_control::delegation::DelegatedScopeType;
use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::rewards::token::RewardTokenEventType;
use crate::domain::teacher_applications::scope::TeacherApplicationScope;
use crate::domain::teacher_applications::status::TeacherApplicationStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformDelegatedPermissionExportState {
    Active,
    Expired,
    Revoked,
}

impl PlatformDelegatedPermissionExportState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Expired => "expired",
            Self::Revoked => "revoked",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformTeacherApplicationExportRowOutput {
    pub application_id: i64,
    pub applicant_user_id: i32,
    pub requested_scope: TeacherApplicationScope,
    pub requested_organization_id: Option<i32>,
    pub requested_course_id: Option<i32>,
    pub organization_sponsor_id: Option<i32>,
    pub status: TeacherApplicationStatus,
    pub reviewer_id: Option<i32>,
    pub decision_reason: String,
    pub portfolio_links: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRewardApprovalExportRowOutput {
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_scope: RewardCandidateSourceScope,
    pub source_organization_id: Option<i32>,
    pub event_type: RewardEventType,
    pub status: RewardCandidateStatus,
    pub teacher_approver_user_id: Option<i32>,
    pub teacher_decision_reason: String,
    pub teacher_decided_at: Option<DateTime<Utc>>,
    pub amount_reviewer_user_id: Option<i32>,
    pub approved_amount: String,
    pub amount_decision_reason: String,
    pub amount_decided_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformTokenPayoutExportRowOutput {
    pub reward_payout_record_id: i64,
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub payout_transaction_id: i64,
    pub external_transaction_id: i64,
    pub amount: String,
    pub blockchain_address: String,
    pub chain_id: Option<i64>,
    pub contract_address: String,
    pub transaction_hash: String,
    pub log_index: Option<i64>,
    pub event_type: Option<RewardTokenEventType>,
    pub from_address: String,
    pub to_address: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformWalletCreditExportRowOutput {
    pub reward_wallet_credit_record_id: i64,
    pub reward_candidate_id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub wallet_id: i32,
    pub transaction_id: i64,
    pub internal_transaction_id: i64,
    pub amount: String,
    pub notification_id: Option<i64>,
    pub notified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformDelegatedPermissionExportRowOutput {
    pub delegated_permission_id: i64,
    pub grantor_user_id: i32,
    pub grantee_user_id: i32,
    pub permission: String,
    pub scope_type: DelegatedScopeType,
    pub organization_id: Option<i32>,
    pub course_id: Option<i32>,
    pub state: PlatformDelegatedPermissionExportState,
    pub reason: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub revoked_by_user_id: Option<i32>,
    pub revoke_reason: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
