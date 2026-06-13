#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseListOutput {
    pub organization: OrganizationCourseSummaryOutput,
    pub courses: Vec<OrganizationCourseListItemOutput>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub reward_available: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseSummaryOutput {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseListItemOutput {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub teachers: Vec<OrganizationCourseTeacherOutput>,
    pub content: OrganizationCourseContentSummaryOutput,
    pub rewards: OrganizationCourseRewardSummaryOutput,
    pub roster: OrganizationCourseRosterSummaryOutput,
    pub reward_queue: OrganizationCourseRewardQueueSummaryOutput,
    pub permissions: OrganizationCoursePermissionSummaryOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseTeacherOutput {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseContentSummaryOutput {
    pub chapter_count: usize,
    pub content_count: usize,
    pub content_types: Vec<String>,
    pub has_content: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseRewardSummaryOutput {
    pub available: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
    pub token_amounts: Vec<String>,
    pub payment_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseRosterSummaryOutput {
    pub enrolled_student_count: i64,
    pub pending_join_request_count: i64,
    pub waitlisted_join_request_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseRewardQueueSummaryOutput {
    pub pending_teacher_count: i64,
    pub teacher_approved_count: i64,
    pub failed_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCoursePermissionSummaryOutput {
    pub can_view_courses: bool,
    pub can_create_courses: bool,
    pub can_manage_course_settings: bool,
    pub can_manage_enrollments: bool,
    pub can_submit_reward_events: bool,
    pub can_view_reward_reports: bool,
    pub can_manage_reward_budget: bool,
}
