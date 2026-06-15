use chrono::{DateTime, Utc};

use crate::domain::rewards::candidate::event_type::RewardEventType;
use crate::domain::rewards::candidate::source::RewardCandidateSourceScope;
use crate::domain::rewards::candidate::status::RewardCandidateStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRewardCandidateUserSummary {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRewardCandidateCourseSummary {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRewardCandidateRecord {
    pub id: i64,
    pub course_id: i32,
    pub student_user_id: i32,
    pub submitter_user_id: i32,
    pub source_scope: RewardCandidateSourceScope,
    pub source_organization_id: Option<i32>,
    pub event_type: RewardEventType,
    pub status: RewardCandidateStatus,
    pub teacher_approver_user_id: Option<i32>,
    pub teacher_decision_reason: Option<String>,
    pub approved_amount: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRewardCandidateItem {
    pub id: i64,
    pub student: PlatformRewardCandidateUserSummary,
    pub course: PlatformRewardCandidateCourseSummary,
    pub event_type: RewardEventType,
    pub status: RewardCandidateStatus,
    pub teacher_approver: Option<PlatformRewardCandidateUserSummary>,
    pub teacher_decision_reason: Option<String>,
    pub approved_amount: Option<String>,
    pub submitter: PlatformRewardCandidateUserSummary,
    pub source_organization_id: Option<i32>,
    pub source_scope: RewardCandidateSourceScope,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRewardCandidatePermissions {
    pub can_view_candidates: bool,
    pub can_approve_amount: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformRewardCandidatesOutput {
    pub candidates: Vec<PlatformRewardCandidateItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
    pub search: Option<String>,
    pub operator_permissions: PlatformRewardCandidatePermissions,
}
