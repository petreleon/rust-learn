use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::teacher_applications::get_my_application::TeacherApplicationSelfUseCase;
use crate::application::teacher_applications::list_applications::TeacherApplicationListUseCase;
use crate::application::teacher_applications::notify_application_event::TeacherApplicationNotificationUseCase;
use crate::application::teacher_applications::submit_application::TeacherApplicationSubmitUseCase;
use crate::domain::teacher_applications::audit::TeacherApplicationAuditEventType;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::dto::{
    teacher_application_responses, ListTeacherApplicationsParams, SubmitTeacherApplicationRequest,
    TeacherApplicationResponse, TeacherApplicationSelfResponse,
};
use crate::http::teacher_applications::errors::{
    list_applications_error, self_application_error, submit_application_error,
};
use crate::http::teacher_applications::support::notify_teacher_application_event;

pub(super) async fn submit_application(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn TeacherApplicationSubmitUseCase>>,
    notifications: Option<web::Data<Arc<dyn TeacherApplicationNotificationUseCase>>>,
    body: web::Json<SubmitTeacherApplicationRequest>,
) -> Result<(web::Json<TeacherApplicationResponse>, StatusCode), ApiError> {
    let command = body.into_inner().into_command(requester.user_id());
    let application = use_case
        .submit_application(command)
        .await
        .map_err(submit_application_error)?;
    notify_teacher_application_event(
        notifications.as_ref(),
        &application,
        TeacherApplicationAuditEventType::Submitted,
        None,
    )
    .await;
    Ok((
        web::Json(TeacherApplicationResponse::from(application)),
        StatusCode::CREATED,
    ))
}

pub(super) async fn list_applications(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn TeacherApplicationListUseCase>>,
    query: web::Query<ListTeacherApplicationsParams>,
) -> Result<web::Json<Vec<TeacherApplicationResponse>>, ApiError> {
    let query = query.into_inner().into_query(requester.user_id());
    use_case
        .list_applications(query)
        .await
        .map(teacher_application_responses)
        .map(web::Json)
        .map_err(list_applications_error)
}

pub(super) async fn get_my_application(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn TeacherApplicationSelfUseCase>>,
) -> Result<web::Json<TeacherApplicationSelfResponse>, ApiError> {
    use_case
        .get_my_application(requester.user_id())
        .await
        .map(TeacherApplicationSelfResponse::from)
        .map(web::Json)
        .map_err(self_application_error)
}
