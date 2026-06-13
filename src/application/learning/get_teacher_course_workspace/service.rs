use futures::future::BoxFuture;

use crate::application::learning::get_teacher_course_workspace::{
    TeacherCourseWorkspaceOutput, TeacherCourseWorkspaceQuery,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;

pub trait TeacherCourseWorkspaceUseCase: Send + Sync {
    fn get_teacher_course_workspace(
        &self,
        query: TeacherCourseWorkspaceQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseWorkspaceOutput, TeacherCourseDashboardError>>;
}
