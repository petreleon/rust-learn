use futures::future::BoxFuture;

use crate::application::learning::get_teacher_course_enrollment_workspace::{
    TeacherCourseEnrollmentWorkspaceOutput, TeacherCourseEnrollmentWorkspaceQuery,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;

pub trait TeacherCourseEnrollmentWorkspaceUseCase: Send + Sync {
    fn get_teacher_course_enrollment_workspace(
        &self,
        query: TeacherCourseEnrollmentWorkspaceQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseEnrollmentWorkspaceOutput, TeacherCourseDashboardError>>;
}
