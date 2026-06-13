use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardItemOutput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeacherCourseDashboardListOutput {
    pub courses: Vec<TeacherCourseDashboardItemOutput>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub search: Option<String>,
    pub lifecycle_status: Option<String>,
}
