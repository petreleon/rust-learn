#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseCatalogItemOutput {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Vec<String>,
    pub prerequisites: Vec<String>,
    pub organizations: Vec<LearnerCourseCatalogOrganizationOutput>,
    pub teachers: Vec<LearnerCourseCatalogTeacherOutput>,
    pub content: LearnerCourseContentSummaryOutput,
    pub rewards: LearnerCourseRewardSummaryOutput,
    pub enrollment: LearnerCourseEnrollmentSummaryOutput,
    pub access: LearnerCourseAccessSummaryOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseCatalogOrganizationOutput {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseCatalogTeacherOutput {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseContentSummaryOutput {
    pub chapter_count: usize,
    pub content_count: usize,
    pub content_types: Vec<String>,
    pub has_content: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseRewardSummaryOutput {
    pub available: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
    pub token_amounts: Vec<String>,
    pub payment_strategies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseEnrollmentSummaryOutput {
    pub state: String,
    pub request_id: Option<i64>,
    pub can_request_join: bool,
    pub reason: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseAccessSummaryOutput {
    pub can_view_course: bool,
    pub can_view_content: bool,
    pub can_view_rewards: bool,
    pub can_request_join: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseCatalogChapterOutput {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<LearnerCourseCatalogContentOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseCatalogContentOutput {
    pub id: i32,
    pub order: i32,
    pub content_type: String,
}
