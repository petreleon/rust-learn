use crate::application::learning::get_teacher_course_students::{
    TeacherCourseStudentsOutput, TeacherCourseStudentsQuery, TeacherCourseStudentsStore,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;

pub async fn get_teacher_course_students(
    store: &mut impl TeacherCourseStudentsStore,
    query: TeacherCourseStudentsQuery,
) -> Result<TeacherCourseStudentsOutput, TeacherCourseDashboardError> {
    store.get_teacher_course_students(query).await
}
