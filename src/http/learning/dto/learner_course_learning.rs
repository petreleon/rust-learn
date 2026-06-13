use serde::Serialize;

use crate::application::learning::get_learner_course_learning::{
    LearnerCourseLearningChapterOutput, LearnerCourseLearningContentOutput,
    LearnerCourseLearningOutput,
};
use crate::http::learning::dto::learner_course_catalog::LearnerCourseCatalogItemResponse;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseLearningResponse {
    pub course: LearnerCourseCatalogItemResponse,
    pub chapters: Vec<LearnerCourseLearningChapterResponse>,
    pub active_content_id: Option<i32>,
    pub progress_supported: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseLearningChapterResponse {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<LearnerCourseLearningContentResponse>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseLearningContentResponse {
    pub id: i32,
    pub chapter_id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
    pub display_state: String,
    pub processing_status: Option<String>,
    pub processing_error: Option<String>,
}

impl From<LearnerCourseLearningOutput> for LearnerCourseLearningResponse {
    fn from(output: LearnerCourseLearningOutput) -> Self {
        Self {
            course: output.course.into(),
            chapters: output.chapters.into_iter().map(Into::into).collect(),
            active_content_id: output.active_content_id,
            progress_supported: output.progress_supported,
        }
    }
}

impl From<LearnerCourseLearningChapterOutput> for LearnerCourseLearningChapterResponse {
    fn from(chapter: LearnerCourseLearningChapterOutput) -> Self {
        Self {
            id: chapter.id,
            title: chapter.title,
            order: chapter.order,
            contents: chapter.contents.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<LearnerCourseLearningContentOutput> for LearnerCourseLearningContentResponse {
    fn from(content: LearnerCourseLearningContentOutput) -> Self {
        Self {
            id: content.id,
            chapter_id: content.chapter_id,
            order: content.order,
            content_type: content.content_type,
            data: content.data,
            display_state: content.display_state,
            processing_status: content.processing_status,
            processing_error: content.processing_error,
        }
    }
}
