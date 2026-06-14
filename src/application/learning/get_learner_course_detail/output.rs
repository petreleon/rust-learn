use crate::application::learning::learner_course_catalog::{
    LearnerCourseCatalogChapterOutput, LearnerCourseCatalogItemOutput,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseDetailOutput {
    pub course: LearnerCourseCatalogItemOutput,
    pub chapters: Vec<LearnerCourseCatalogChapterOutput>,
    pub prerequisites: Vec<String>,
}
