use futures::future::BoxFuture;

use crate::application::learning::list_teacher_course_dashboard::{
    TeacherCourseDashboardListOutput, TeacherCourseDashboardListQuery,
};
use crate::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;

pub trait TeacherCourseDashboardListStore {
    fn list_teacher_course_dashboard(
        &mut self,
        query: TeacherCourseDashboardListQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseDashboardListOutput, TeacherCourseDashboardError>>;
}
