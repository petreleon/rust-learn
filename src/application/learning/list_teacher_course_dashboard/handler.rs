use crate::application::learning::list_teacher_course_dashboard::{
    TeacherCourseDashboardListOutput, TeacherCourseDashboardListQuery,
    TeacherCourseDashboardListStore,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;

pub async fn list_teacher_course_dashboard(
    store: &mut impl TeacherCourseDashboardListStore,
    query: TeacherCourseDashboardListQuery,
) -> Result<TeacherCourseDashboardListOutput, TeacherCourseDashboardError> {
    store.list_teacher_course_dashboard(query).await
}
