use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::db;
use crate::http::teacher_applications::support::{
    notify_teacher_application_event, service_error_response,
};
use crate::services::teacher_application_service::{
    self, ListTeacherApplicationsRequest, PlatformTeacherApplicationsRequest,
    SubmitTeacherApplicationRequest, TeacherApplicationDecisionRequest,
};
use crate::utils::request_auth::authenticated_user;

pub(super) async fn submit_application(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    body: web::Json<SubmitTeacherApplicationRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::submit_application(
        &mut conn,
        requester.user_id,
        body.into_inner(),
    )
    .await
    {
        Ok(application) => {
            notify_teacher_application_event(&req, &mut conn, &application, "submitted", None)
                .await;
            HttpResponse::Created().json(application)
        }
        Err(error) => service_error_response(error),
    }
}

pub(super) async fn list_applications(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<ListTeacherApplicationsRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::list_applications(
        &mut conn,
        requester.user_id,
        query.into_inner(),
    )
    .await
    {
        Ok(applications) => HttpResponse::Ok().json(applications),
        Err(error) => service_error_response(error),
    }
}

pub(super) async fn list_platform_review_applications(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
    query: web::Query<PlatformTeacherApplicationsRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::list_platform_applications(
        &mut conn,
        requester.user_id,
        query.into_inner(),
    )
    .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => service_error_response(error),
    }
}

pub(super) async fn get_my_application(
    req: HttpRequest,
    pool: web::Data<db::DbPool>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };

    match teacher_application_service::get_my_application(&mut conn, requester.user_id).await {
        Ok(snapshot) => HttpResponse::Ok().json(snapshot),
        Err(error) => service_error_response(error),
    }
}

pub(super) async fn decide_application(
    req: HttpRequest,
    path: web::Path<i64>,
    pool: web::Data<db::DbPool>,
    body: web::Json<TeacherApplicationDecisionRequest>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let mut conn = match pool.get().await {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to get DB connection"),
    };
    let decision = body.into_inner();
    let decision_reason = decision.decision_reason.clone();

    match teacher_application_service::decide_application(
        &mut conn,
        requester.user_id,
        path.into_inner(),
        decision,
    )
    .await
    {
        Ok(application) => {
            let event_type = application.status.clone();
            notify_teacher_application_event(
                &req,
                &mut conn,
                &application,
                &event_type,
                decision_reason.as_deref(),
            )
            .await;
            HttpResponse::Ok().json(application)
        }
        Err(error) => service_error_response(error),
    }
}
