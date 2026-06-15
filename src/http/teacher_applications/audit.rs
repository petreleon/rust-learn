use std::sync::Arc;

use actix_web::web;

use crate::application::teacher_applications::list_application_audit::{
    TeacherApplicationAuditQuery, TeacherApplicationAuditUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;
use crate::http::teacher_applications::dto::TeacherApplicationAuditEventResponse;
use crate::http::teacher_applications::errors::audit_error;

pub(super) async fn list_audit_events(
    requester: AuthUser,
    path: web::Path<i64>,
    use_case: web::Data<Arc<dyn TeacherApplicationAuditUseCase>>,
) -> Result<web::Json<Vec<TeacherApplicationAuditEventResponse>>, ApiError> {
    let query = TeacherApplicationAuditQuery {
        actor_user_id: requester.user_id(),
        application_id: path.into_inner(),
    };
    use_case
        .list_application_audit(query)
        .await
        .map(audit_event_responses)
        .map(web::Json)
        .map_err(audit_error)
}

fn audit_event_responses(
    events: Vec<crate::application::teacher_applications::TeacherApplicationAuditEventOutput>,
) -> Vec<TeacherApplicationAuditEventResponse> {
    events
        .into_iter()
        .map(TeacherApplicationAuditEventResponse::from)
        .collect()
}
