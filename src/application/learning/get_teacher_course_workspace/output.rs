use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardItemOutput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseWorkspaceOutput {
    pub course: TeacherCourseDashboardItemOutput,
    pub teacher_roles: Vec<String>,
    pub publication: TeacherCoursePublicationSummaryOutput,
    pub chapters: Vec<TeacherCourseWorkspaceChapterOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCoursePublicationSummaryOutput {
    pub course_lifecycle_status: String,
    pub content_publication_status_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseWorkspaceChapterOutput {
    pub id: i32,
    pub title: String,
    pub order: i32,
    pub contents: Vec<TeacherCourseWorkspaceContentOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseWorkspaceContentOutput {
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
