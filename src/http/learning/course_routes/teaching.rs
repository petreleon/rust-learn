use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::services::course_service::{
    discover_teacher_course_dashboard, get_teacher_course_enrollment_workspace,
    get_teacher_course_students, get_teacher_course_workspace, TeacherCourseDashboardQuery,
    TeacherCourseEnrollmentQuery,
};
use crate::utils::request_auth::authenticated_user;

use super::dto::{TeacherCourseDashboardParams, TeacherCourseEnrollmentParams};
use super::support::teacher_course_dashboard_error_response;

pub(super) async fn list_teacher_course_dashboard(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<TeacherCourseDashboardParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let dashboard_query = TeacherCourseDashboardQuery::new(
        query.search.clone(),
        query.lifecycle_status.clone(),
        query.limit,
        query.offset,
    );

    match discover_teacher_course_dashboard(&mut conn, requester.user_id, dashboard_query).await {
        Ok(catalog) => HttpResponse::Ok().json(catalog),
        Err(error) => teacher_course_dashboard_error_response(error),
    }
}

pub(super) async fn get_teacher_course_workspace_route(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match get_teacher_course_workspace(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(workspace) => HttpResponse::Ok().json(workspace),
        Err(error) => teacher_course_dashboard_error_response(error),
    }
}

pub(super) async fn get_teacher_course_enrollment_workspace_route(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
    query: web::Query<TeacherCourseEnrollmentParams>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    let enrollment_query =
        TeacherCourseEnrollmentQuery::new(query.status.clone(), query.limit, query.offset);
    match get_teacher_course_enrollment_workspace(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        enrollment_query,
    )
    .await
    {
        Ok(workspace) => HttpResponse::Ok().json(workspace),
        Err(error) => teacher_course_dashboard_error_response(error),
    }
}

pub(super) async fn get_teacher_course_students_route(
    req: HttpRequest,
    path: web::Path<i32>,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(c) => c,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match get_teacher_course_students(&mut conn, requester.user_id, path.into_inner()).await {
        Ok(students) => HttpResponse::Ok().json(students),
        Err(error) => teacher_course_dashboard_error_response(error),
    }
}
