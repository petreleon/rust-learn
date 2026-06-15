use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::teacher_applications::list_platform_review::{
    TeacherApplicationPlatformReviewAuditSummaryOutput,
    TeacherApplicationPlatformReviewCourseOutput, TeacherApplicationPlatformReviewItemOutput,
    TeacherApplicationPlatformReviewOrganizationOutput,
    TeacherApplicationPlatformReviewPermissionsOutput,
    TeacherApplicationPlatformReviewSummaryOutput, TeacherApplicationPlatformReviewUserOutput,
};

#[derive(Debug, Serialize)]
pub(super) struct PlatformTeacherApplicationItemResponse {
    pub(super) id: i64,
    pub(super) applicant: TeacherApplicationUserSummaryResponse,
    pub(super) requested_scope: String,
    pub(super) requested_organization: Option<TeacherApplicationOrganizationSummaryResponse>,
    pub(super) requested_course: Option<TeacherApplicationCourseSummaryResponse>,
    pub(super) sponsor_organization: Option<TeacherApplicationOrganizationSummaryResponse>,
    pub(super) experience_summary: String,
    pub(super) portfolio_links: Vec<String>,
    pub(super) status: String,
    pub(super) reviewer: Option<TeacherApplicationUserSummaryResponse>,
    pub(super) decision_reason: Option<String>,
    pub(super) audit: TeacherApplicationAuditSummaryResponse,
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
    pub(super) decided_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub(super) struct TeacherApplicationUserSummaryResponse {
    pub(super) id: i32,
    pub(super) name: String,
    pub(super) email: String,
}

#[derive(Debug, Serialize)]
pub(super) struct TeacherApplicationOrganizationSummaryResponse {
    pub(super) id: i32,
    pub(super) name: String,
}

#[derive(Debug, Serialize)]
pub(super) struct TeacherApplicationCourseSummaryResponse {
    pub(super) id: i32,
    pub(super) title: String,
}

#[derive(Debug, Serialize)]
pub(super) struct TeacherApplicationAuditSummaryResponse {
    pub(super) event_count: usize,
    pub(super) latest_event_type: Option<String>,
    pub(super) latest_event_at: Option<DateTime<Utc>>,
    pub(super) latest_reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub(super) struct TeacherApplicationDashboardSummaryResponse {
    pub(super) approved: i64,
    pub(super) needs_changes: i64,
    pub(super) rejected: i64,
    pub(super) submitted: i64,
    pub(super) total: i64,
}

#[derive(Debug, Serialize)]
pub(super) struct PlatformTeacherApplicationPermissionsResponse {
    pub(super) can_view_applications: bool,
    pub(super) can_approve_applications: bool,
    pub(super) can_reject_applications: bool,
    pub(super) can_request_changes: bool,
}

impl From<TeacherApplicationPlatformReviewItemOutput> for PlatformTeacherApplicationItemResponse {
    fn from(application: TeacherApplicationPlatformReviewItemOutput) -> Self {
        Self {
            applicant: application.applicant.into(),
            audit: application.audit.into(),
            created_at: application.created_at,
            decided_at: application.decided_at,
            decision_reason: application.decision_reason,
            experience_summary: application.experience_summary,
            id: application.id,
            portfolio_links: application.portfolio_links,
            requested_course: application.requested_course.map(Into::into),
            requested_organization: application.requested_organization.map(Into::into),
            requested_scope: application.requested_scope,
            reviewer: application.reviewer.map(Into::into),
            sponsor_organization: application.sponsor_organization.map(Into::into),
            status: application.status,
            updated_at: application.updated_at,
        }
    }
}

impl From<TeacherApplicationPlatformReviewUserOutput> for TeacherApplicationUserSummaryResponse {
    fn from(user: TeacherApplicationPlatformReviewUserOutput) -> Self {
        Self {
            email: user.email,
            id: user.id,
            name: user.name,
        }
    }
}

impl From<TeacherApplicationPlatformReviewOrganizationOutput>
    for TeacherApplicationOrganizationSummaryResponse
{
    fn from(organization: TeacherApplicationPlatformReviewOrganizationOutput) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
        }
    }
}

impl From<TeacherApplicationPlatformReviewCourseOutput>
    for TeacherApplicationCourseSummaryResponse
{
    fn from(course: TeacherApplicationPlatformReviewCourseOutput) -> Self {
        Self {
            id: course.id,
            title: course.title,
        }
    }
}

impl From<TeacherApplicationPlatformReviewAuditSummaryOutput>
    for TeacherApplicationAuditSummaryResponse
{
    fn from(audit: TeacherApplicationPlatformReviewAuditSummaryOutput) -> Self {
        Self {
            event_count: audit.event_count,
            latest_event_at: audit.latest_event_at,
            latest_event_type: audit
                .latest_event_type
                .map(|event_type| event_type.as_str().to_string()),
            latest_reason: audit.latest_reason,
        }
    }
}

impl From<TeacherApplicationPlatformReviewSummaryOutput>
    for TeacherApplicationDashboardSummaryResponse
{
    fn from(summary: TeacherApplicationPlatformReviewSummaryOutput) -> Self {
        Self {
            approved: summary.approved,
            needs_changes: summary.needs_changes,
            rejected: summary.rejected,
            submitted: summary.submitted,
            total: summary.total,
        }
    }
}

impl From<TeacherApplicationPlatformReviewPermissionsOutput>
    for PlatformTeacherApplicationPermissionsResponse
{
    fn from(permissions: TeacherApplicationPlatformReviewPermissionsOutput) -> Self {
        Self {
            can_approve_applications: permissions.can_approve_applications,
            can_reject_applications: permissions.can_reject_applications,
            can_request_changes: permissions.can_request_changes,
            can_view_applications: permissions.can_view_applications,
        }
    }
}
