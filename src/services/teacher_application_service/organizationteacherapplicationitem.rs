#[derive(Debug, Clone, Serialize)]
pub struct OrganizationTeacherApplicationItem {
    pub id: i64,
    pub applicant: TeacherApplicationUserSummary,
    pub requested_scope: String,
    pub requested_organization: Option<TeacherApplicationOrganizationSummary>,
    pub requested_course: Option<TeacherApplicationCourseSummary>,
    pub sponsored_by_this_organization: bool,
    pub requested_for_this_organization: bool,
    pub experience_summary: String,
    pub portfolio_links: Vec<String>,
    pub status: String,
    pub reviewer: Option<TeacherApplicationUserSummary>,
    pub decision_reason: Option<String>,
    pub audit: TeacherApplicationAuditSummary,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub decided_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeacherApplicationUserSummary {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeacherApplicationOrganizationSummary {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeacherApplicationCourseSummary {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TeacherApplicationAuditSummary {
    pub event_count: usize,
    pub latest_event_type: Option<String>,
    pub latest_event_at: Option<chrono::DateTime<chrono::Utc>>,
    pub latest_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct TeacherApplicationDashboardSummary {
    pub approved: i64,
    pub needs_changes: i64,
    pub rejected: i64,
    pub submitted: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationTeacherApplicationPermissions {
    pub can_view_applications: bool,
    pub can_nominate_teachers: bool,
}
