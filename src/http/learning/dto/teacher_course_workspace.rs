use serde::Serialize;

use crate::application::learning::get_teacher_course_workspace::{
    TeacherCoursePublicationSummaryOutput, TeacherCourseWorkspaceChapterOutput,
    TeacherCourseWorkspaceContentOutput, TeacherCourseWorkspaceOutput,
};

use super::teacher_course_dashboard::TeacherCourseDashboardItemResponse;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseWorkspaceResponse {
    pub course: TeacherCourseDashboardItemResponse,
    pub teacher_roles: Vec<String>,
    pub publication: TeacherCoursePublicationSummaryResponse,
    pub chapters: Vec<TeacherCourseWorkspaceChapterResponse>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCoursePublicationSummaryResponse {
    pub course_lifecycle_status: String,
    pub content_publication_status_supported: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseWorkspaceChapterResponse {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<TeacherCourseWorkspaceContentResponse>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TeacherCourseWorkspaceContentResponse {
    pub id: i32,
    pub order: i32,
    pub content_type: String,
    pub data: Option<String>,
    pub data_present: bool,
    pub publication_status: String,
    pub display_state: String,
    pub processing_status: Option<String>,
    pub processing_error: Option<String>,
}

impl From<TeacherCourseWorkspaceOutput> for TeacherCourseWorkspaceResponse {
    fn from(output: TeacherCourseWorkspaceOutput) -> Self {
        Self {
            course: output.course.into(),
            teacher_roles: output.teacher_roles,
            publication: output.publication.into(),
            chapters: output.chapters.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<TeacherCoursePublicationSummaryOutput> for TeacherCoursePublicationSummaryResponse {
    fn from(publication: TeacherCoursePublicationSummaryOutput) -> Self {
        Self {
            course_lifecycle_status: publication.course_lifecycle_status,
            content_publication_status_supported: publication.content_publication_status_supported,
        }
    }
}

impl From<TeacherCourseWorkspaceChapterOutput> for TeacherCourseWorkspaceChapterResponse {
    fn from(chapter: TeacherCourseWorkspaceChapterOutput) -> Self {
        Self {
            id: chapter.id,
            title: chapter.title,
            order: chapter.order,
            contents: chapter.contents.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<TeacherCourseWorkspaceContentOutput> for TeacherCourseWorkspaceContentResponse {
    fn from(content: TeacherCourseWorkspaceContentOutput) -> Self {
        Self {
            id: content.id,
            order: content.order,
            content_type: content.content_type,
            data: content.data,
            data_present: content.data_present,
            publication_status: content.publication_status,
            display_state: content.display_state,
            processing_status: content.processing_status,
            processing_error: content.processing_error,
        }
    }
}
