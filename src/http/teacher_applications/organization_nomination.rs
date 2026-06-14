use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::teacher_applications::nominate_application::{
    TeacherApplicationNominationError, TeacherApplicationNominationUseCase,
};
use crate::application::teacher_applications::notify_application_event::TeacherApplicationNotificationUseCase;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::dto::TeacherApplicationResponse;
use crate::http::teacher_applications::organization_nomination_dto::OrganizationTeacherNominationRequest;
use crate::http::teacher_applications::support::notify_teacher_application_event;

pub(crate) async fn nominate_application(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn TeacherApplicationNominationUseCase>>,
    notifications: Option<web::Data<Arc<dyn TeacherApplicationNotificationUseCase>>>,
    body: web::Json<OrganizationTeacherNominationRequest>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let command = body
        .into_inner()
        .into_command(requester.user_id(), organization_id);

    match use_case.nominate_application(command).await {
        Ok(application) => {
            notify_teacher_application_event(
                notifications.as_ref(),
                &application,
                "organization_nominated",
                None,
            )
            .await;
            HttpResponse::Created().json(TeacherApplicationResponse::from(application))
        }
        Err(error) => nomination_error_response(error),
    }
}

fn nomination_error_response(error: TeacherApplicationNominationError) -> HttpResponse {
    match error {
        TeacherApplicationNominationError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        TeacherApplicationNominationError::InvalidInput(message) => {
            HttpResponse::BadRequest().body(message)
        }
        TeacherApplicationNominationError::InvalidTransition(message) => {
            HttpResponse::Conflict().body(message)
        }
        TeacherApplicationNominationError::NotFound => {
            HttpResponse::NotFound().body("Teacher application not found")
        }
        TeacherApplicationNominationError::Connection(message)
        | TeacherApplicationNominationError::Database(message) => {
            log::error!(
                "event=teacher_application_nomination_api_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process teacher application")
        }
    }
}
