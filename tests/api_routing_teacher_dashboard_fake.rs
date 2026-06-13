use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::learning::list_teacher_course_dashboard::{
    TeacherCourseDashboardListOutput, TeacherCourseDashboardListQuery,
    TeacherCourseDashboardListUseCase,
};
use rust_learn::application::learning::teacher_course_dashboard::TeacherCourseDashboardError;

struct RouteOnlyTeacherDashboardUseCase;

pub fn teacher_dashboard_data() -> web::Data<Arc<dyn TeacherCourseDashboardListUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyTeacherDashboardUseCase) as Arc<dyn TeacherCourseDashboardListUseCase>
    )
}

impl TeacherCourseDashboardListUseCase for RouteOnlyTeacherDashboardUseCase {
    fn list_teacher_course_dashboard(
        &self,
        query: TeacherCourseDashboardListQuery,
    ) -> BoxFuture<'_, Result<TeacherCourseDashboardListOutput, TeacherCourseDashboardError>> {
        ready(Ok(TeacherCourseDashboardListOutput {
            courses: Vec::new(),
            total: 0,
            limit: query.limit,
            offset: query.offset,
            search: query.search,
            lifecycle_status: query.lifecycle_status,
        }))
        .boxed()
    }
}
