use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::teacher_applications::get_my_application::{
    TeacherApplicationSelfError, TeacherApplicationSelfUseCase,
};
use crate::application::teacher_applications::list_applications::{
    TeacherApplicationListError, TeacherApplicationListUseCase,
};
use crate::application::teacher_applications::notify_application_event::TeacherApplicationNotificationUseCase;
use crate::application::teacher_applications::submit_application::{
    TeacherApplicationSubmitError, TeacherApplicationSubmitUseCase,
};
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::dto::{
    teacher_application_responses, ListTeacherApplicationsParams, SubmitTeacherApplicationRequest,
    TeacherApplicationResponse, TeacherApplicationSelfResponse,
};
use crate::http::teacher_applications::support::notify_teacher_application_event;

pub(super) async fn submit_application(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn TeacherApplicationSubmitUseCase>>,
    notifications: Option<web::Data<Arc<dyn TeacherApplicationNotificationUseCase>>>,
    body: web::Json<SubmitTeacherApplicationRequest>,
) -> impl Responder {
    let command = body.into_inner().into_command(requester.user_id());
    match use_case.submit_application(command).await {
        Ok(application) => {
            notify_teacher_application_event(
                notifications.as_ref(),
                &application,
                "submitted",
                None,
            )
            .await;
            HttpResponse::Created().json(TeacherApplicationResponse::from(application))
        }
        Err(error) => submit_application_error_response(error),
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

pub(super) async fn get_my_application(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn TeacherApplicationSelfUseCase>>,
) -> impl Responder {
    match use_case.get_my_application(requester.user_id()).await {
        Ok(snapshot) => HttpResponse::Ok().json(TeacherApplicationSelfResponse::from(snapshot)),
        Err(error) => self_application_error_response(error),
    }
}

fn submit_application_error_response(error: TeacherApplicationSubmitError) -> HttpResponse {
    match error {
        TeacherApplicationSubmitError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have the required permission")
        }
        TeacherApplicationSubmitError::InvalidInput(message) => {
            HttpResponse::BadRequest().body(message)
        }
        TeacherApplicationSubmitError::InvalidTransition(message) => {
            HttpResponse::Conflict().body(message)
        }
        TeacherApplicationSubmitError::Connection(message)
        | TeacherApplicationSubmitError::Database(message) => {
            log::error!(
                "event=teacher_application_submit_api_failed error={}",
                message
            );
            HttpResponse::InternalServerError().body("Failed to process teacher application")
        }
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
