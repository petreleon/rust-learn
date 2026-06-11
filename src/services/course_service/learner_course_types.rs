#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogItem {
    pub id: i32,
    pub title: String,
    pub lifecycle_status: String,
    pub description: Option<String>,
    pub topics: Vec<String>,
    pub prerequisites: Vec<String>,
    pub organizations: Vec<LearnerCourseCatalogOrganization>,
    pub teachers: Vec<LearnerCourseCatalogTeacher>,
    pub content: LearnerCourseContentSummary,
    pub rewards: LearnerCourseRewardSummary,
    pub enrollment: LearnerCourseEnrollmentSummary,
    pub access: LearnerCourseAccessSummary,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogOrganization {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogTeacher {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseContentSummary {
    pub chapter_count: usize,
    pub content_count: usize,
    pub content_types: Vec<String>,
    pub has_content: bool,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseRewardSummary {
    pub available: bool,
    pub active_policy_count: usize,
    pub event_types: Vec<String>,
    pub token_amounts: Vec<String>,
    pub payment_strategies: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseEnrollmentSummary {
    pub state: String,
    pub request_id: Option<i64>,
    pub can_request_join: bool,
    pub reason: Option<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseAccessSummary {
    pub can_view_course: bool,
    pub can_view_content: bool,
    pub can_view_rewards: bool,
    pub can_request_join: bool,
}

enum TeacherCourseCandidateScope {
    All,
    CourseIds(Vec<i32>),
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogChapter {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<LearnerCourseCatalogContent>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogContent {
    pub id: i32,
    pub order: i32,
    pub content_type: String,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseLearningChapter {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<LearnerCourseLearningContent>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseLearningContent {
    pub id: i32,
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
    pub display_state: String,
    pub processing_status: Option<String>,
    pub processing_error: Option<String>,
}
