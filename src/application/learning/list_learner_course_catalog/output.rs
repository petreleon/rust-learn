use crate::application::learning::learner_course_catalog::LearnerCourseCatalogItemOutput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseCatalogOutput {
    pub courses: Vec<LearnerCourseCatalogItemOutput>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub lifecycle_status: Option<String>,
    pub enrollment_status: Option<String>,
    pub reward_available: Option<bool>,
}
