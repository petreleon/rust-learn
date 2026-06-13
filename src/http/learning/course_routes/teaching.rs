use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

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
use crate::http::learning::dto::{
    TeacherCourseDashboardResponse, TeacherCourseEnrollmentWorkspaceResponse,
    TeacherCourseStudentsResponse, TeacherCourseWorkspaceResponse,
};
use crate::utils::request_auth::authenticated_user;

use super::dto::{TeacherCourseDashboardParams, TeacherCourseEnrollmentParams};
use super::support::teacher_course_dashboard_read_error_response;

pub(super) async fn list_teacher_course_dashboard(
    req: HttpRequest,
    use_case: web::Data<Arc<dyn TeacherCourseDashboardListUseCase>>,
    query: web::Query<TeacherCourseDashboardParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    let dashboard_query = TeacherCourseDashboardListQuery::new(
        requester.user_id,
        query.search.clone(),
        query.lifecycle_status.clone(),
        query.limit,
        query.offset,
    );

    match use_case
        .list_teacher_course_dashboard(dashboard_query)
        .await
    {
        Ok(catalog) => HttpResponse::Ok().json(TeacherCourseDashboardResponse::from(catalog)),
        Err(error) => teacher_course_dashboard_read_error_response(error),
    }
}

pub(super) async fn get_teacher_course_workspace_route(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn TeacherCourseWorkspaceUseCase>>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    let query = TeacherCourseWorkspaceQuery::new(requester.user_id, path.into_inner());
    match use_case.get_teacher_course_workspace(query).await {
        Ok(workspace) => HttpResponse::Ok().json(TeacherCourseWorkspaceResponse::from(workspace)),
        Err(error) => teacher_course_dashboard_read_error_response(error),
    }
}

pub(super) async fn get_teacher_course_enrollment_workspace_route(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn TeacherCourseEnrollmentWorkspaceUseCase>>,
    query: web::Query<TeacherCourseEnrollmentParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    let enrollment_query = TeacherCourseEnrollmentWorkspaceQuery::new(
        requester.user_id,
        path.into_inner(),
        query.status.clone(),
        query.limit,
        query.offset,
    );
    match use_case
        .get_teacher_course_enrollment_workspace(enrollment_query)
        .await
    {
        Ok(workspace) => {
            HttpResponse::Ok().json(TeacherCourseEnrollmentWorkspaceResponse::from(workspace))
        }
        Err(error) => teacher_course_dashboard_read_error_response(error),
    }
}

pub(super) async fn get_teacher_course_students_route(
    req: HttpRequest,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn TeacherCourseStudentsUseCase>>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };

    let query = TeacherCourseStudentsQuery::new(requester.user_id, path.into_inner());
    match use_case.get_teacher_course_students(query).await {
        Ok(students) => HttpResponse::Ok().json(TeacherCourseStudentsResponse::from(students)),
        Err(error) => teacher_course_dashboard_read_error_response(error),
    }
}
