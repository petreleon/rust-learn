use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::teacher_applications::decide_application::{
    TeacherApplicationDecisionError, TeacherApplicationDecisionUseCase,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::decision_dto::TeacherApplicationDecisionRequest;
use crate::http::teacher_applications::dto::TeacherApplicationResponse;
use crate::http::teacher_applications::support::{
    notify_teacher_application_event, TeacherApplicationNotification,
};

pub(super) async fn decide_application(
    req: HttpRequest,
    requester: AuthUser,
    path: web::Path<i64>,
    use_case: web::Data<Arc<dyn TeacherApplicationDecisionUseCase>>,
    body: web::Json<TeacherApplicationDecisionRequest>,
) -> impl Responder {
    let decision = body.into_inner();
    let decision_reason = decision.decision_reason.clone();
    match use_case
        .decide_application(decision.into_command(requester.user_id(), path.into_inner()))
        .await
    {
        Ok(application) => {
            let event_type = application.status.clone();
            notify_teacher_application_event(
                &req,
                &TeacherApplicationNotification::from(&application),
                &event_type,
                decision_reason.as_deref(),
            )
            .await;
            HttpResponse::Ok().json(TeacherApplicationResponse::from(application))
        }
        Err(error) => decision_error_response(error),
    }
}

fn decision_error_response(error: TeacherApplicationDecisionError) -> HttpResponse {
    match error {
        TeacherApplicationDecisionError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        TeacherApplicationDecisionError::InvalidInput(message) => {
            HttpResponse::BadRequest().body(message)
        }
        TeacherApplicationDecisionError::InvalidTransition(message) => {
            HttpResponse::Conflict().body(message)
        }
        TeacherApplicationDecisionError::NotFound => {
            HttpResponse::NotFound().body("Teacher application not found")
        }
        TeacherApplicationDecisionError::Connection(message)
        | TeacherApplicationDecisionError::Database(message) => {
            log::error!(
                "event=teacher_application_decision_api_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process teacher application")
        }
    }
}
