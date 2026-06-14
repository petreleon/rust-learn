use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationPlatformReviewOutput {
    pub applications: Vec<TeacherApplicationPlatformReviewItemOutput>,
    pub summary: TeacherApplicationPlatformReviewSummaryOutput,
    pub operator_permissions: TeacherApplicationPlatformReviewPermissionsOutput,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationPlatformReviewDataset {
    pub applications: Vec<TeacherApplicationPlatformReviewItemOutput>,
    pub summary: TeacherApplicationPlatformReviewSummaryOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationPlatformReviewItemOutput {
    pub id: i64,
    pub applicant: TeacherApplicationPlatformReviewUserOutput,
    pub requested_scope: String,
    pub requested_organization: Option<TeacherApplicationPlatformReviewOrganizationOutput>,
    pub requested_course: Option<TeacherApplicationPlatformReviewCourseOutput>,
    pub sponsor_organization: Option<TeacherApplicationPlatformReviewOrganizationOutput>,
    pub experience_summary: String,
    pub portfolio_links: Vec<String>,
    pub status: String,
    pub reviewer: Option<TeacherApplicationPlatformReviewUserOutput>,
    pub decision_reason: Option<String>,
    pub audit: TeacherApplicationPlatformReviewAuditSummaryOutput,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationPlatformReviewUserOutput {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationPlatformReviewOrganizationOutput {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherApplicationPlatformReviewCourseOutput {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationPlatformReviewAuditSummaryOutput {
    pub event_count: usize,
    pub latest_event_type: Option<String>,
    pub latest_event_at: Option<DateTime<Utc>>,
    pub latest_reason: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationPlatformReviewSummaryOutput {
    pub approved: i64,
    pub needs_changes: i64,
    pub rejected: i64,
    pub submitted: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TeacherApplicationPlatformReviewPermissionsOutput {
    pub can_view_applications: bool,
    pub can_approve_applications: bool,
    pub can_reject_applications: bool,
    pub can_request_changes: bool,
}
