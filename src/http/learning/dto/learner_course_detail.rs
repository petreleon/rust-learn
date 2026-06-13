use serde::Serialize;

use crate::application::learning::get_learner_course_detail::LearnerCourseDetailOutput;
use crate::application::learning::learner_course_catalog::{
    LearnerCourseCatalogChapterOutput, LearnerCourseCatalogContentOutput,
};
use crate::http::learning::dto::learner_course_catalog::LearnerCourseCatalogItemResponse;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseDetailResponse {
    pub course: LearnerCourseCatalogItemResponse,
    pub chapters: Vec<LearnerCourseCatalogChapterResponse>,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseCatalogChapterResponse {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<LearnerCourseCatalogContentResponse>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LearnerCourseCatalogContentResponse {
    pub id: i32,
    pub order: i32,
    pub content_type: String,
}

impl From<LearnerCourseDetailOutput> for LearnerCourseDetailResponse {
    fn from(output: LearnerCourseDetailOutput) -> Self {
        Self {
            course: output.course.into(),
            chapters: output.chapters.into_iter().map(Into::into).collect(),
            prerequisites: output.prerequisites,
        }
    }
}

impl From<LearnerCourseCatalogChapterOutput> for LearnerCourseCatalogChapterResponse {
    fn from(chapter: LearnerCourseCatalogChapterOutput) -> Self {
        Self {
            id: chapter.id,
            title: chapter.title,
            order: chapter.order,
            contents: chapter.contents.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<LearnerCourseCatalogContentOutput> for LearnerCourseCatalogContentResponse {
    fn from(content: LearnerCourseCatalogContentOutput) -> Self {
        Self {
            id: content.id,
            order: content.order,
            content_type: content.content_type,
        }
    }
}
