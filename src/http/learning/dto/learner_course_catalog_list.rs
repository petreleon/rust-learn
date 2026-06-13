use serde::Serialize;

use crate::application::learning::list_learner_course_catalog::LearnerCourseCatalogOutput;
use crate::http::learning::dto::learner_course_catalog::LearnerCourseCatalogItemResponse;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseCatalogResponse {
    pub courses: Vec<LearnerCourseCatalogItemResponse>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub lifecycle_status: Option<String>,
    pub enrollment_status: Option<String>,
    pub reward_available: Option<bool>,
}

impl From<LearnerCourseCatalogOutput> for LearnerCourseCatalogResponse {
    fn from(output: LearnerCourseCatalogOutput) -> Self {
        Self {
            courses: output.courses.into_iter().map(Into::into).collect(),
            total: output.total,
            limit: output.limit,
            offset: output.offset,
            search: output.search,
            organization_id: output.organization_id,
            lifecycle_status: output.lifecycle_status,
            enrollment_status: output.enrollment_status,
            reward_available: output.reward_available,
        }
    }
}
