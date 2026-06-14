use serde::Deserialize;

#[derive(Deserialize)]
pub(super) struct CourseDiscoveryParams {
    pub(super) search: Option<String>,
    pub(super) organization_id: Option<i32>,
    pub(super) limit: Option<i64>,
    pub(super) offset: Option<i64>,
}

#[derive(Deserialize)]
pub(super) struct LearnerCourseCatalogParams {
    pub(super) search: Option<String>,
    pub(super) organization_id: Option<i32>,
    pub(super) lifecycle_status: Option<String>,
    pub(super) enrollment_status: Option<String>,
    pub(super) reward_available: Option<bool>,
    pub(super) limit: Option<i64>,
    pub(super) offset: Option<i64>,
}

#[derive(Deserialize)]
pub(super) struct TeacherCourseDashboardParams {
    pub(super) search: Option<String>,
    pub(super) lifecycle_status: Option<String>,
    pub(super) limit: Option<i64>,
    pub(super) offset: Option<i64>,
}

#[derive(Deserialize)]
pub(super) struct TeacherCourseEnrollmentParams {
    pub(super) status: Option<String>,
    pub(super) limit: Option<i64>,
    pub(super) offset: Option<i64>,
}
