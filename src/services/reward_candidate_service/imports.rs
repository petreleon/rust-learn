use crate::config::constants::permissions::Permissions;
use crate::db::schema::{
    courses, courses_organizations, reward_candidates, reward_fraud_blocks, reward_policies, users,
};
use crate::models::reward_audit_event::{
    NewRewardAuditEvent, REWARD_AUDIT_EVENT_AMOUNT_DECISION,
    REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED, REWARD_AUDIT_EVENT_TEACHER_DECISION,
};
use crate::models::reward_candidate::{
    NewRewardCandidate, RewardCandidate, REWARD_EVENT_ADMINISTRATIVE_ADJUSTMENT,
    REWARD_EVENT_ASSESSMENT_COMPLETION, REWARD_EVENT_COURSE_COMPLETION,
    REWARD_EVENT_MANUAL_COMPLETION, REWARD_SOURCE_COURSE, REWARD_SOURCE_ORGANIZATION,
    REWARD_STATUS_ADJUSTED, REWARD_STATUS_AMOUNT_APPROVED, REWARD_STATUS_AMOUNT_REJECTED,
    REWARD_STATUS_COMPLETED, REWARD_STATUS_FAILED, REWARD_STATUS_NEEDS_RECONCILIATION,
    REWARD_STATUS_NOTIFIED, REWARD_STATUS_PENDING_TEACHER_APPROVAL, REWARD_STATUS_TEACHER_APPROVED,
    REWARD_STATUS_TEACHER_REJECTED, REWARD_STATUS_TOKEN_CONFIRMED, REWARD_STATUS_TOKEN_PENDING,
    REWARD_STATUS_WALLET_CREDITED,
};
use crate::models::reward_fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};
use crate::models::reward_policy::{
    REWARD_POLICY_SCOPE_COURSE, REWARD_POLICY_SCOPE_ORGANIZATION, REWARD_POLICY_SCOPE_PLATFORM,
};
use crate::repositories::course_repository::user_permission_course_request;
use crate::repositories::organization_repository::user_permission_organization_request;
use crate::repositories::platform_repository::user_permission_platform_request;
use crate::repositories::reward_audit_event_repository;
use crate::repositories::reward_candidate_repository::{self, RewardCandidateFilter};
use crate::repositories::reward_execution_job_repository;
use bigdecimal::BigDecimal;
use chrono::Utc;
use diesel::dsl::exists;
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitRewardCandidateRequest {
    pub student_user_id: i32,
    pub event_type: String,
    pub idempotency_key: Option<String>,
    pub evidence: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TeacherRewardCandidateDecisionRequest {
    pub status: String,
    pub decision_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RewardAmountDecisionRequest {
    pub status: String,
    pub approved_amount: Option<BigDecimal>,
    pub decision_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListRewardCandidatesRequest {
    pub status: Option<String>,
    pub student_user_id: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PlatformRewardCandidatesRequest {
    pub status: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardCandidateUserSummary {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardCandidateCourseSummary {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardCandidateItem {
    pub id: i64,
    pub student: PlatformRewardCandidateUserSummary,
    pub course: PlatformRewardCandidateCourseSummary,
    pub event_type: String,
    pub status: String,
    pub teacher_approver: Option<PlatformRewardCandidateUserSummary>,
    pub teacher_decision_reason: Option<String>,
    pub approved_amount: Option<String>,
    pub submitter: PlatformRewardCandidateUserSummary,
    pub source_organization_id: Option<i32>,
    pub source_scope: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardCandidatePermissions {
    pub can_view_candidates: bool,
    pub can_approve_amount: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardCandidatesResponse {
    pub candidates: Vec<PlatformRewardCandidateItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
    pub search: Option<String>,
    pub operator_permissions: PlatformRewardCandidatePermissions,
}

#[derive(Debug, PartialEq, Eq)]
pub enum RewardCandidateError {
    PermissionDenied(String),
    InvalidInput(String),
    InvalidStatus(String),
    NotFound,
    Database(String),
}

impl From<diesel::result::Error> for RewardCandidateError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => RewardCandidateError::NotFound,
            other => RewardCandidateError::Database(other.to_string()),
        }
    }
}

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
