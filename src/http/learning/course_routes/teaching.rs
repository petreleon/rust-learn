use std::sync::Arc;

use actix_web::web;

use crate::application::learning::get_teacher_course_enrollment_workspace::{
    TeacherCourseEnrollmentWorkspaceQuery, TeacherCourseEnrollmentWorkspaceUseCase,
};
use crate::application::learning::get_teacher_course_students::{
    TeacherCourseStudentsQuery, TeacherCourseStudentsUseCase,
};
use crate::application::learning::get_teacher_course_workspace::{
    TeacherCourseWorkspaceQuery, TeacherCourseWorkspaceUseCase,
};
use crate::application::learning::list_teacher_course_dashboard::{
    TeacherCourseDashboardListQuery, TeacherCourseDashboardListUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::learning::dto::{
    TeacherCourseDashboardResponse, TeacherCourseEnrollmentWorkspaceResponse,
    TeacherCourseStudentsResponse, TeacherCourseWorkspaceResponse,
};

use super::dto::{TeacherCourseDashboardParams, TeacherCourseEnrollmentParams};
use super::errors::teacher_course_dashboard_read_error;

pub(super) async fn list_teacher_course_dashboard(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn TeacherCourseDashboardListUseCase>>,
    query: web::Query<TeacherCourseDashboardParams>,
) -> Result<web::Json<TeacherCourseDashboardResponse>, ApiError> {
    let dashboard_query = TeacherCourseDashboardListQuery::new(
        requester.user_id(),
        query.search.clone(),
        query.lifecycle_status.clone(),
        query.limit,
        query.offset,
    );

    use_case
        .list_teacher_course_dashboard(dashboard_query)
        .await
        .map(TeacherCourseDashboardResponse::from)
        .map(web::Json)
        .map_err(teacher_course_dashboard_read_error)
}

pub(super) async fn get_teacher_course_workspace_route(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn TeacherCourseWorkspaceUseCase>>,
) -> Result<web::Json<TeacherCourseWorkspaceResponse>, ApiError> {
    let query = TeacherCourseWorkspaceQuery::new(requester.user_id(), path.into_inner());
    use_case
        .get_teacher_course_workspace(query)
        .await
        .map(TeacherCourseWorkspaceResponse::from)
        .map(web::Json)
        .map_err(teacher_course_dashboard_read_error)
}

pub(super) async fn get_teacher_course_enrollment_workspace_route(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn TeacherCourseEnrollmentWorkspaceUseCase>>,
    query: web::Query<TeacherCourseEnrollmentParams>,
) -> Result<web::Json<TeacherCourseEnrollmentWorkspaceResponse>, ApiError> {
    let enrollment_query = TeacherCourseEnrollmentWorkspaceQuery::new(
        requester.user_id(),
        path.into_inner(),
        query.status.clone(),
        query.limit,
        query.offset,
    );
    use_case
        .get_teacher_course_enrollment_workspace(enrollment_query)
        .await
        .map(TeacherCourseEnrollmentWorkspaceResponse::from)
        .map(web::Json)
        .map_err(teacher_course_dashboard_read_error)
}

pub(super) async fn get_teacher_course_students_route(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn TeacherCourseStudentsUseCase>>,
) -> Result<web::Json<TeacherCourseStudentsResponse>, ApiError> {
    let query = TeacherCourseStudentsQuery::new(requester.user_id(), path.into_inner());
    use_case
        .get_teacher_course_students(query)
        .await
        .map(TeacherCourseStudentsResponse::from)
        .map(web::Json)
        .map_err(teacher_course_dashboard_read_error)
}
