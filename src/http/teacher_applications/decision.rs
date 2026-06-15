use std::sync::Arc;

use actix_web::web;

use crate::application::teacher_applications::decide_application::TeacherApplicationDecisionUseCase;
use crate::application::teacher_applications::notify_application_event::TeacherApplicationNotificationUseCase;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::decision_dto::TeacherApplicationDecisionRequest;
use crate::http::teacher_applications::dto::TeacherApplicationResponse;
use crate::http::teacher_applications::errors::decision_error;
use crate::http::teacher_applications::support::notify_teacher_application_event;

pub(super) async fn decide_application(
    requester: AuthUser,
    path: web::Path<i64>,
    use_case: web::Data<Arc<dyn TeacherApplicationDecisionUseCase>>,
    notifications: Option<web::Data<Arc<dyn TeacherApplicationNotificationUseCase>>>,
    body: web::Json<TeacherApplicationDecisionRequest>,
) -> Result<web::Json<TeacherApplicationResponse>, ApiError> {
    let decision = body.into_inner();
    let decision_reason = decision.decision_reason.clone();
    let application = use_case
        .decide_application(decision.into_command(requester.user_id(), path.into_inner()))
        .await
        .map_err(decision_error)?;
    let event_type = application.status.clone();
    notify_teacher_application_event(
        notifications.as_ref(),
        &application,
        &event_type,
        decision_reason.as_deref(),
    )
    .await;
    Ok(web::Json(TeacherApplicationResponse::from(application)))
}
