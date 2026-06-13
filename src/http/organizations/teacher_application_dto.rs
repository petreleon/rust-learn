use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationItemOutput, OrganizationTeacherApplicationListOutput,
};
use crate::http::organizations::teacher_application_nested_dto::{
    OrganizationTeacherApplicationAuditSummaryResponse,
    OrganizationTeacherApplicationCourseResponse,
    OrganizationTeacherApplicationOrganizationResponse,
    OrganizationTeacherApplicationPermissionsResponse, TeacherApplicationDashboardSummaryResponse,
    TeacherApplicationUserSummaryResponse,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationsResponse {
    pub organization: OrganizationTeacherApplicationOrganizationResponse,
    pub applications: Vec<OrganizationTeacherApplicationItemResponse>,
    pub summary: TeacherApplicationDashboardSummaryResponse,
    pub operator_permissions: OrganizationTeacherApplicationPermissionsResponse,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationItemResponse {
    pub id: i64,
    pub applicant: TeacherApplicationUserSummaryResponse,
    pub requested_scope: String,
    pub requested_organization: Option<OrganizationTeacherApplicationOrganizationResponse>,
    pub requested_course: Option<OrganizationTeacherApplicationCourseResponse>,
    pub sponsored_by_this_organization: bool,
    pub requested_for_this_organization: bool,
    pub experience_summary: String,
    pub portfolio_links: Vec<String>,
    pub status: String,
    pub reviewer: Option<TeacherApplicationUserSummaryResponse>,
    pub decision_reason: Option<String>,
    pub audit: OrganizationTeacherApplicationAuditSummaryResponse,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
}

impl From<OrganizationTeacherApplicationListOutput> for OrganizationTeacherApplicationsResponse {
    fn from(output: OrganizationTeacherApplicationListOutput) -> Self {
        Self {
            organization: output.organization.into(),
            applications: output.applications.into_iter().map(Into::into).collect(),
            summary: output.summary.into(),
            operator_permissions: output.operator_permissions.into(),
            total: output.total,
            limit: output.limit,
            offset: output.offset,
            status: output.status,
            search: output.search,
        }
    }
}

impl From<OrganizationTeacherApplicationItemOutput> for OrganizationTeacherApplicationItemResponse {
    fn from(application: OrganizationTeacherApplicationItemOutput) -> Self {
        Self {
            id: application.id,
            applicant: application.applicant.into(),
            requested_scope: application.requested_scope,
            requested_organization: application.requested_organization.map(Into::into),
            requested_course: application.requested_course.map(Into::into),
            sponsored_by_this_organization: application.sponsored_by_this_organization,
            requested_for_this_organization: application.requested_for_this_organization,
            experience_summary: application.experience_summary,
            portfolio_links: application.portfolio_links,
            status: application.status,
            reviewer: application.reviewer.map(Into::into),
            decision_reason: application.decision_reason,
            audit: application.audit.into(),
            created_at: application.created_at,
            updated_at: application.updated_at,
            decided_at: application.decided_at,
        }
    }
}
