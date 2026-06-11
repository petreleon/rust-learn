#[derive(Debug, Serialize)]
pub struct TeacherCourseDashboardResponse {
    pub courses: Vec<TeacherCourseDashboardItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationCourseListResponse {
    pub organization: LearnerCourseCatalogOrganization,
    pub courses: Vec<OrganizationCourseListItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub reward_available: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct OrganizationCourseListItem {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub teachers: Vec<LearnerCourseCatalogTeacher>,
    pub content: LearnerCourseContentSummary,
    pub rewards: LearnerCourseRewardSummary,
    pub roster: TeacherCourseRosterSummary,
    pub reward_queue: TeacherCourseRewardQueueSummary,
    pub permissions: OrganizationCoursePermissionSummary,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseDashboardItem {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub organizations: Vec<LearnerCourseCatalogOrganization>,
    pub content: LearnerCourseContentSummary,
    pub rewards: LearnerCourseRewardSummary,
    pub roster: TeacherCourseRosterSummary,
    pub reward_queue: TeacherCourseRewardQueueSummary,
    pub permissions: TeacherCoursePermissionSummary,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseWorkspaceResponse {
    pub course: TeacherCourseDashboardItem,
    pub teacher_roles: Vec<String>,
    pub publication: TeacherCoursePublicationSummary,
    pub chapters: Vec<TeacherCourseWorkspaceChapter>,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseEnrollmentWorkspaceResponse {
    pub course: TeacherCourseDashboardItem,
    pub teacher_roles: Vec<String>,
    pub join_requests: TeacherCourseJoinRequestPage,
    pub roster: TeacherCourseRosterPage,
    pub progress_supported: bool,
    pub reward_eligibility_supported: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseStudentsResponse {
    pub course: TeacherCourseDashboardItem,
    pub teacher_roles: Vec<String>,
    pub students: Vec<TeacherCourseStudentProgressItem>,
    pub total: i64,
    pub progress_supported: bool,
    pub reward_evidence_supported: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseStudentProgressItem {
    pub user: TeacherEnrollmentUserSummary,
    pub roles: Vec<String>,
    pub access_state: String,
    pub latest_join_request_status: Option<String>,
    pub progress: TeacherStudentProgressSummary,
    pub rewards: TeacherStudentRewardProgressSummary,
}

#[derive(Debug, Serialize)]
pub struct TeacherStudentProgressSummary {
    pub supported: bool,
    pub completed_content_count: Option<i64>,
    pub total_content_count: usize,
    pub completion_percentage: Option<f64>,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub note: String,
}

#[derive(Debug, Serialize)]
pub struct TeacherStudentRewardProgressSummary {
    pub reward_candidate_count: i64,
    pub pending_teacher_count: i64,
    pub teacher_approved_count: i64,
    pub teacher_rejected_count: i64,
    pub completed_count: i64,
    pub failed_count: i64,
    pub latest_candidate: Option<TeacherStudentRewardCandidateSummary>,
}

#[derive(Debug, Serialize)]
pub struct TeacherStudentRewardCandidateSummary {
    pub id: i64,
    pub event_type: String,
    pub status: String,
    pub evidence: Value,
    pub teacher_decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
