use crate::application::learning::get_teacher_course_enrollment_workspace::{
    TeacherCourseEnrollmentWorkspaceOutput, TeacherCourseEnrollmentWorkspaceQuery,
    TeacherCourseEnrollmentWorkspaceStore,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;

pub async fn get_teacher_course_enrollment_workspace(
    store: &mut impl TeacherCourseEnrollmentWorkspaceStore,
    query: TeacherCourseEnrollmentWorkspaceQuery,
) -> Result<TeacherCourseEnrollmentWorkspaceOutput, TeacherCourseDashboardError> {
    store.get_teacher_course_enrollment_workspace(query).await
}
