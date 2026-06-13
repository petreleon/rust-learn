use crate::application::learning::get_teacher_course_workspace::{
    TeacherCourseWorkspaceOutput, TeacherCourseWorkspaceQuery, TeacherCourseWorkspaceStore,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;

pub async fn get_teacher_course_workspace(
    store: &mut impl TeacherCourseWorkspaceStore,
    query: TeacherCourseWorkspaceQuery,
) -> Result<TeacherCourseWorkspaceOutput, TeacherCourseDashboardError> {
    store.get_teacher_course_workspace(query).await
}
