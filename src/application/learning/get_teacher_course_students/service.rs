use futures::future::BoxFuture;

use crate::application::learning::get_teacher_course_students::{
    TeacherCourseStudentsOutput, TeacherCourseStudentsQuery,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;

pub trait TeacherCourseStudentsUseCase: Send + Sync {
    fn get_teacher_course_students(
        &self,
        query: TeacherCourseStudentsQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseStudentsOutput, TeacherCourseDashboardError>>;
}
