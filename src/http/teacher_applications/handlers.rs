use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::teacher_applications::get_my_application::{
    TeacherApplicationSelfError, TeacherApplicationSelfUseCase,
};
use crate::application::teacher_applications::list_applications::{
    TeacherApplicationListError, TeacherApplicationListUseCase,
};
use crate::db;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::dto::{
    teacher_application_responses, ListTeacherApplicationsParams, TeacherApplicationSelfResponse,
};
use crate::http::teacher_applications::support::{
    notify_teacher_application_event, service_error_response,
};
use crate::services::teacher_application_service::{
    self, PlatformTeacherApplicationsRequest, SubmitTeacherApplicationRequest,
    TeacherApplicationDecisionRequest,
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
    requester: AuthUser,
    use_case: web::Data<Arc<dyn TeacherApplicationListUseCase>>,
    query: web::Query<ListTeacherApplicationsParams>,
) -> impl Responder {
    let query = query.into_inner().into_query(requester.user_id());
    match use_case.list_applications(query).await {
        Ok(applications) => HttpResponse::Ok().json(teacher_application_responses(applications)),
        Err(error) => list_applications_error_response(error),
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
    requester: AuthUser,
    use_case: web::Data<Arc<dyn TeacherApplicationSelfUseCase>>,
) -> impl Responder {
    match use_case.get_my_application(requester.user_id()).await {
        Ok(snapshot) => HttpResponse::Ok().json(TeacherApplicationSelfResponse::from(snapshot)),
        Err(error) => self_application_error_response(error),
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

fn self_application_error_response(error: TeacherApplicationSelfError) -> HttpResponse {
    match error {
        TeacherApplicationSelfError::Connection(message)
        | TeacherApplicationSelfError::Database(message) => {
            log::error!(
                "event=teacher_application_self_api_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process teacher application")
        }
    }
}

fn list_applications_error_response(error: TeacherApplicationListError) -> HttpResponse {
    match error {
        TeacherApplicationListError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        TeacherApplicationListError::InvalidInput(message) => {
            HttpResponse::BadRequest().body(message)
        }
        TeacherApplicationListError::Connection(message)
        | TeacherApplicationListError::Database(message) => {
            log::error!(
                "event=teacher_application_list_api_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process teacher application")
        }
    }
}
