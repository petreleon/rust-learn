use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::application::organizations::list_organization_teacher_applications::{
    OrganizationTeacherApplicationAuditSummaryOutput, OrganizationTeacherApplicationCourseOutput,
    OrganizationTeacherApplicationOrganizationOutput,
    OrganizationTeacherApplicationPermissionsOutput, TeacherApplicationDashboardSummaryOutput,
    TeacherApplicationUserSummaryOutput,
};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherApplicationUserSummaryResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationOrganizationResponse {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationCourseResponse {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationAuditSummaryResponse {
    pub event_count: usize,
    pub latest_event_type: Option<String>,
    pub latest_event_at: Option<DateTime<Utc>>,
    pub latest_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherApplicationDashboardSummaryResponse {
    pub approved: i64,
    pub needs_changes: i64,
    pub rejected: i64,
    pub submitted: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationPermissionsResponse {
    pub can_view_applications: bool,
    pub can_nominate_teachers: bool,
}

impl From<TeacherApplicationUserSummaryOutput> for TeacherApplicationUserSummaryResponse {
    fn from(user: TeacherApplicationUserSummaryOutput) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}

impl From<OrganizationTeacherApplicationOrganizationOutput>
    for OrganizationTeacherApplicationOrganizationResponse
{
    fn from(organization: OrganizationTeacherApplicationOrganizationOutput) -> Self {
        Self {
            id: organization.id,
            name: organization.name,
        }
    }
}

impl From<OrganizationTeacherApplicationCourseOutput>
    for OrganizationTeacherApplicationCourseResponse
{
    fn from(course: OrganizationTeacherApplicationCourseOutput) -> Self {
        Self {
            id: course.id,
            title: course.title,
        }
    }
}

impl From<OrganizationTeacherApplicationAuditSummaryOutput>
    for OrganizationTeacherApplicationAuditSummaryResponse
{
    fn from(audit: OrganizationTeacherApplicationAuditSummaryOutput) -> Self {
        Self {
            event_count: audit.event_count,
            latest_event_type: audit.latest_event_type,
            latest_event_at: audit.latest_event_at,
            latest_reason: audit.latest_reason,
        }
    }
}

impl From<TeacherApplicationDashboardSummaryOutput> for TeacherApplicationDashboardSummaryResponse {
    fn from(summary: TeacherApplicationDashboardSummaryOutput) -> Self {
        Self {
            approved: summary.approved,
            needs_changes: summary.needs_changes,
            rejected: summary.rejected,
            submitted: summary.submitted,
            total: summary.total,
        }
    }
}

impl From<OrganizationTeacherApplicationPermissionsOutput>
    for OrganizationTeacherApplicationPermissionsResponse
{
    fn from(permissions: OrganizationTeacherApplicationPermissionsOutput) -> Self {
        Self {
            can_view_applications: permissions.can_view_applications,
            can_nominate_teachers: permissions.can_nominate_teachers,
        }
    }
}
