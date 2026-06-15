use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::teacher_applications::nominate_application::TeacherApplicationNominationUseCase;
use crate::application::teacher_applications::notify_application_event::TeacherApplicationNotificationUseCase;
use crate::domain::teacher_applications::audit::TeacherApplicationAuditEventType;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::dto::TeacherApplicationResponse;
use crate::http::teacher_applications::errors::nomination_error;
use crate::http::teacher_applications::organization_nomination_dto::OrganizationTeacherNominationRequest;
use crate::http::teacher_applications::support::notify_teacher_application_event;

pub(crate) async fn nominate_application(
    requester: AuthUser,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn TeacherApplicationNominationUseCase>>,
    notifications: Option<web::Data<Arc<dyn TeacherApplicationNotificationUseCase>>>,
    body: web::Json<OrganizationTeacherNominationRequest>,
) -> Result<(web::Json<TeacherApplicationResponse>, StatusCode), ApiError> {
    let organization_id = path.into_inner();
    let command = body
        .into_inner()
        .into_command(requester.user_id(), organization_id);

    let application = use_case
        .nominate_application(command)
        .await
        .map_err(nomination_error)?;
    notify_teacher_application_event(
        notifications.as_ref(),
        &application,
        TeacherApplicationAuditEventType::OrganizationNominated,
        None,
    )
    .await;
    Ok((
        web::Json(TeacherApplicationResponse::from(application)),
        StatusCode::CREATED,
    ))
}
