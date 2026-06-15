use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::rewards::list_platform_candidates::{
    PlatformRewardCandidateCourseSummary, PlatformRewardCandidateItem,
    PlatformRewardCandidatePermissions, PlatformRewardCandidateUserSummary,
    PlatformRewardCandidatesQuery, PlatformRewardCandidatesResponse,
};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PlatformRewardCandidatesRequest {
    pub status: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardCandidatesResponseBody {
    pub candidates: Vec<PlatformRewardCandidateItemResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
    pub search: Option<String>,
    pub operator_permissions: PlatformRewardCandidatePermissionsResponse,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardCandidateUserSummaryResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardCandidateCourseSummaryResponse {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardCandidateItemResponse {
    pub id: i64,
    pub student: PlatformRewardCandidateUserSummaryResponse,
    pub course: PlatformRewardCandidateCourseSummaryResponse,
    pub event_type: String,
    pub status: String,
    pub teacher_approver: Option<PlatformRewardCandidateUserSummaryResponse>,
    pub teacher_decision_reason: Option<String>,
    pub approved_amount: Option<String>,
    pub submitter: PlatformRewardCandidateUserSummaryResponse,
    pub source_organization_id: Option<i32>,
    pub source_scope: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlatformRewardCandidatePermissionsResponse {
    pub can_view_candidates: bool,
    pub can_approve_amount: bool,
}

impl From<PlatformRewardCandidatesRequest> for PlatformRewardCandidatesQuery {
    fn from(request: PlatformRewardCandidatesRequest) -> Self {
        Self {
            status: request.status,
            search: request.search,
            limit: request.limit,
            offset: request.offset,
        }
    }
}

impl From<PlatformRewardCandidatesResponse> for PlatformRewardCandidatesResponseBody {
    fn from(output: PlatformRewardCandidatesResponse) -> Self {
        Self {
            candidates: output
                .candidates
                .into_iter()
                .map(PlatformRewardCandidateItemResponse::from)
                .collect(),
            total: output.total,
            limit: output.limit,
            offset: output.offset,
            status: output.status,
            search: output.search,
            operator_permissions: PlatformRewardCandidatePermissionsResponse::from(
                output.operator_permissions,
            ),
        }
    }
}

impl From<PlatformRewardCandidateItem> for PlatformRewardCandidateItemResponse {
    fn from(candidate: PlatformRewardCandidateItem) -> Self {
        Self {
            id: candidate.id,
            student: PlatformRewardCandidateUserSummaryResponse::from(candidate.student),
            course: PlatformRewardCandidateCourseSummaryResponse::from(candidate.course),
            event_type: candidate.event_type.as_str().to_string(),
            status: candidate.status.as_str().to_string(),
            teacher_approver: candidate
                .teacher_approver
                .map(PlatformRewardCandidateUserSummaryResponse::from),
            teacher_decision_reason: candidate.teacher_decision_reason,
            approved_amount: candidate.approved_amount,
            submitter: PlatformRewardCandidateUserSummaryResponse::from(candidate.submitter),
            source_organization_id: candidate.source_organization_id,
            source_scope: candidate.source_scope.as_str().to_string(),
            created_at: candidate.created_at,
            updated_at: candidate.updated_at,
        }
    }
}

impl From<PlatformRewardCandidateUserSummary> for PlatformRewardCandidateUserSummaryResponse {
    fn from(user: PlatformRewardCandidateUserSummary) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}

impl From<PlatformRewardCandidateCourseSummary> for PlatformRewardCandidateCourseSummaryResponse {
    fn from(course: PlatformRewardCandidateCourseSummary) -> Self {
        Self {
            id: course.id,
            title: course.title,
        }
    }
}

impl From<PlatformRewardCandidatePermissions> for PlatformRewardCandidatePermissionsResponse {
    fn from(permissions: PlatformRewardCandidatePermissions) -> Self {
        Self {
            can_view_candidates: permissions.can_view_candidates,
            can_approve_amount: permissions.can_approve_amount,
        }
    }
}
