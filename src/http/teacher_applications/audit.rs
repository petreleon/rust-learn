use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::teacher_applications::list_application_audit::{
    TeacherApplicationAuditError, TeacherApplicationAuditQuery, TeacherApplicationAuditUseCase,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::dto::TeacherApplicationAuditEventResponse;

pub(super) async fn list_audit_events(
    requester: AuthUser,
    path: web::Path<i64>,
    use_case: web::Data<Arc<dyn TeacherApplicationAuditUseCase>>,
) -> impl Responder {
    let query = TeacherApplicationAuditQuery {
        actor_user_id: requester.user_id(),
        application_id: path.into_inner(),
    };
    match use_case.list_application_audit(query).await {
        Ok(events) => HttpResponse::Ok().json(audit_event_responses(events)),
        Err(error) => audit_error_response(error),
    }
}

fn audit_error_response(error: TeacherApplicationAuditError) -> HttpResponse {
    match error {
        TeacherApplicationAuditError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        TeacherApplicationAuditError::Connection(message)
        | TeacherApplicationAuditError::Database(message) => {
            log::error!(
                "event=teacher_application_audit_api_failed reason=database error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process teacher application")
        }
    }
}

fn audit_event_responses(
    events: Vec<crate::application::teacher_applications::TeacherApplicationAuditEventOutput>,
) -> Vec<TeacherApplicationAuditEventResponse> {
    events
        .into_iter()
        .map(TeacherApplicationAuditEventResponse::from)
        .collect()
}
