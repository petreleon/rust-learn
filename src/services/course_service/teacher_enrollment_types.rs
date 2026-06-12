#[derive(Debug, Serialize)]
pub struct TeacherCourseJoinRequestPage {
    pub requests: Vec<TeacherCourseJoinRequestItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseJoinRequestItem {
    pub id: i64,
    pub status: String,
    pub requester: TeacherEnrollmentUserSummary,
    pub reviewer: Option<TeacherEnrollmentUserSummary>,
    pub decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
    pub can_decide: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseRosterPage {
    pub learners: Vec<TeacherCourseRosterLearner>,
    pub total: i64,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseRosterLearner {
    pub user: TeacherEnrollmentUserSummary,
    pub roles: Vec<String>,
    pub latest_join_request_status: Option<String>,
    pub access_state: String,
    pub can_remove: bool,
    pub progress_supported: bool,
    pub reward_eligibility_supported: bool,
    pub reward_eligibility: TeacherStudentRewardEligibilitySummary,
}

#[derive(Debug, Serialize)]
pub struct TeacherEnrollmentUserSummary {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub kyc_verified: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherCoursePublicationSummary {
    pub course_lifecycle_status: String,
    pub content_publication_status_supported: bool,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseWorkspaceChapter {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<TeacherCourseWorkspaceContent>,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseWorkspaceContent {
    pub id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
    pub data_present: bool,
    pub publication_status: String,
    pub display_state: String,
    pub processing_status: Option<String>,
    pub processing_error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseRosterSummary {
    pub enrolled_student_count: i64,
    pub pending_join_request_count: i64,
    pub waitlisted_join_request_count: i64,
}

#[derive(Debug, Serialize)]
pub struct TeacherCourseRewardQueueSummary {
    pub pending_teacher_count: i64,
    pub teacher_approved_count: i64,
    pub failed_count: i64,
}

#[derive(Debug, Serialize)]
pub struct TeacherCoursePermissionSummary {
    pub can_manage_settings: bool,
    pub can_manage_content: bool,
    pub can_manage_enrollments: bool,
    pub can_view_reward_candidates: bool,
    pub can_approve_reward_candidates: bool,
    pub can_manage_reward_rules: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrganizationCoursePermissionSummary {
    pub can_view_courses: bool,
    pub can_create_courses: bool,
    pub can_manage_course_settings: bool,
    pub can_manage_enrollments: bool,
    pub can_submit_reward_events: bool,
    pub can_view_reward_reports: bool,
    pub can_manage_reward_budget: bool,
}
