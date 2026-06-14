use crate::models::course::Course;
use serde::Serialize;

use super::learner_course_types::{
    LearnerCourseCatalogChapter, LearnerCourseCatalogItem, LearnerCourseLearningChapter,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseDiscoveryQuery {
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize)]
pub struct CourseDiscoveryResponse {
    pub courses: Vec<Course>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub organization_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseCatalogQuery {
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub lifecycle_status: Option<String>,
    pub enrollment_status: Option<String>,
    pub reward_available: Option<bool>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseCatalogResponse {
    pub courses: Vec<LearnerCourseCatalogItem>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub organization_id: Option<i32>,
    pub lifecycle_status: Option<String>,
    pub enrollment_status: Option<String>,
    pub reward_available: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseDetailResponse {
    pub course: LearnerCourseCatalogItem,
    pub chapters: Vec<LearnerCourseCatalogChapter>,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct LearnerCourseLearningResponse {
    pub course: LearnerCourseCatalogItem,
    pub chapters: Vec<LearnerCourseLearningChapter>,
    pub active_content_id: Option<i32>,
    pub progress_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseDashboardQuery {
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrganizationCourseListQuery {
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
    pub reward_available: Option<bool>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseEnrollmentQuery {
    pub status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}
