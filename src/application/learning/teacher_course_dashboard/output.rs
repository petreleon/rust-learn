#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseDashboardItemOutput {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Option<String>,
    pub prerequisites: Option<String>,
    pub organizations: Vec<TeacherCourseDashboardOrganizationOutput>,
    pub content: TeacherCourseContentSummaryOutput,
    pub rewards: TeacherCourseRewardSummaryOutput,
    pub roster: TeacherCourseRosterSummaryOutput,
    pub reward_queue: TeacherCourseRewardQueueSummaryOutput,
    pub permissions: TeacherCoursePermissionSummaryOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseDashboardOrganizationOutput {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseContentSummaryOutput {
    pub chapter_count: usize,
    pub content_count: usize,
    pub content_types: Vec<String>,
    pub has_content: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseRewardSummaryOutput {
    pub available: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
    pub token_amounts: Vec<String>,
    pub payment_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseRosterSummaryOutput {
    pub enrolled_student_count: i64,
    pub pending_join_request_count: i64,
    pub waitlisted_join_request_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseRewardQueueSummaryOutput {
    pub pending_teacher_count: i64,
    pub teacher_approved_count: i64,
    pub failed_count: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCoursePermissionSummaryOutput {
    pub can_manage_settings: bool,
    pub can_manage_content: bool,
    pub can_manage_enrollments: bool,
    pub can_view_reward_candidates: bool,
    pub can_approve_reward_candidates: bool,
    pub can_manage_reward_rules: bool,
}

impl TeacherCoursePermissionSummaryOutput {
    pub fn has_teacher_access(&self) -> bool {
        self.can_manage_settings
            || self.can_manage_content
            || self.can_manage_enrollments
            || self.can_view_reward_candidates
            || self.can_approve_reward_candidates
            || self.can_manage_reward_rules
    }
}
