use crate::application::learning::learner_course_catalog::LearnerCourseCatalogItemOutput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseLearningOutput {
    pub course: LearnerCourseCatalogItemOutput,
    pub chapters: Vec<LearnerCourseLearningChapterOutput>,
    pub active_content_id: Option<i32>,
    pub progress_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseLearningChapterOutput {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<LearnerCourseLearningContentOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnerCourseLearningContentOutput {
    pub id: i32,
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
    pub display_state: String,
    pub processing_status: Option<String>,
    pub processing_error: Option<String>,
}
