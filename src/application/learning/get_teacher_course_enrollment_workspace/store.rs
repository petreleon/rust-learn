use futures::future::BoxFuture;

use crate::application::learning::get_teacher_course_enrollment_workspace::{
    TeacherCourseEnrollmentWorkspaceOutput, TeacherCourseEnrollmentWorkspaceQuery,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;

pub trait TeacherCourseEnrollmentWorkspaceStore {
    fn get_teacher_course_enrollment_workspace(
        &mut self,
        query: TeacherCourseEnrollmentWorkspaceQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseEnrollmentWorkspaceOutput, TeacherCourseDashboardError>>;
}
