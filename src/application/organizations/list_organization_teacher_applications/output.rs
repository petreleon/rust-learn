use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationListOutput {
    pub organization: OrganizationTeacherApplicationOrganizationOutput,
    pub applications: Vec<OrganizationTeacherApplicationItemOutput>,
    pub summary: TeacherApplicationDashboardSummaryOutput,
    pub operator_permissions: OrganizationTeacherApplicationPermissionsOutput,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationDataset {
    pub applications: Vec<OrganizationTeacherApplicationItemOutput>,
    pub summary: TeacherApplicationDashboardSummaryOutput,
    pub operator_permissions: OrganizationTeacherApplicationPermissionsOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationItemOutput {
    pub id: i64,
    pub applicant: TeacherApplicationUserSummaryOutput,
    pub requested_scope: String,
    pub requested_organization: Option<OrganizationTeacherApplicationOrganizationOutput>,
    pub requested_course: Option<OrganizationTeacherApplicationCourseOutput>,
    pub sponsored_by_this_organization: bool,
    pub requested_for_this_organization: bool,
    pub experience_summary: String,
    pub portfolio_links: Vec<String>,
    pub status: String,
    pub reviewer: Option<TeacherApplicationUserSummaryOutput>,
    pub decision_reason: Option<String>,
    pub audit: OrganizationTeacherApplicationAuditSummaryOutput,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationUserSummaryOutput {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationOrganizationOutput {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationCourseOutput {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationAuditSummaryOutput {
    pub event_count: usize,
    pub latest_event_type: Option<String>,
    pub latest_event_at: Option<DateTime<Utc>>,
    pub latest_reason: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationDashboardSummaryOutput {
    pub approved: i64,
    pub needs_changes: i64,
    pub rejected: i64,
    pub submitted: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OrganizationTeacherApplicationPermissionsOutput {
    pub can_view_applications: bool,
    pub can_nominate_teachers: bool,
}

impl OrganizationTeacherApplicationPermissionsOutput {
    pub fn with(mut self, can_view_applications: bool, can_nominate_teachers: bool) -> Self {
        self.can_view_applications = can_view_applications;
        self.can_nominate_teachers = can_nominate_teachers;
        self
    }
}
