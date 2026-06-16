use std::sync::Arc;

use actix_web::web;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::application::identity::assign_platform_role::{
    AssignPlatformRoleCommand, PlatformRoleAssignmentUseCase,
};
use crate::application::identity::list_platform_role_assignment_audit::{
    PlatformRoleAssignmentAuditEventOutput, PlatformRoleAssignmentAuditQuery,
    PlatformRoleAssignmentAuditUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;
use crate::http::identity::errors::{
    platform_role_assignment_audit_error, platform_role_assignment_error,
};

#[derive(Deserialize)]
pub(super) struct AssignRoleRequest {
    pub role_name: String,
}

#[derive(Debug, Serialize)]
pub(super) struct RoleAssignmentAuditEventResponse {
    pub id: i64,
    pub target_user_id: i32,
    pub actor_user_id: Option<i32>,
    pub platform_role_id: Option<i32>,
    pub role_name: String,
    pub event_type: String,
    pub created_at: DateTime<Utc>,
}

pub(super) async fn assign_role(
    requester: AuthUserId,
    path: web::Path<i32>,
    body: web::Json<AssignRoleRequest>,
    use_case: web::Data<Arc<dyn PlatformRoleAssignmentUseCase>>,
) -> Result<&'static str, ApiError> {
    let target_user_id = path.into_inner();
    let role_name = body.into_inner().role_name;

    let requester_id = requester.into_inner();

    let command = AssignPlatformRoleCommand {
        requester_user_id: requester_id,
        target_user_id,
        role_name: role_name.clone(),
    };
    use_case
        .assign_platform_role(command)
        .await
        .map(|_| "Role assigned successfully")
        .map_err(|error| {
            platform_role_assignment_error(requester_id, target_user_id, &role_name, error)
        })
}

pub(super) async fn list_audit(
    requester: AuthUserId,
    path: web::Path<i32>,
    use_case: web::Data<Arc<dyn PlatformRoleAssignmentAuditUseCase>>,
) -> Result<web::Json<Vec<RoleAssignmentAuditEventResponse>>, ApiError> {
    let target_user_id = path.into_inner();
    let query = PlatformRoleAssignmentAuditQuery {
        actor_user_id: requester.into_inner(),
        target_user_id,
    };

    use_case
        .list_platform_role_assignment_audit(query)
        .await
        .map(audit_event_responses)
        .map(web::Json)
        .map_err(|error| platform_role_assignment_audit_error(target_user_id, error))
}

fn audit_event_responses(
    events: Vec<PlatformRoleAssignmentAuditEventOutput>,
) -> Vec<RoleAssignmentAuditEventResponse> {
    events
        .into_iter()
        .map(RoleAssignmentAuditEventResponse::from)
        .collect()
}

impl From<PlatformRoleAssignmentAuditEventOutput> for RoleAssignmentAuditEventResponse {
    fn from(event: PlatformRoleAssignmentAuditEventOutput) -> Self {
        Self {
            actor_user_id: event.actor_user_id,
            created_at: event.created_at,
            event_type: event.event_type.as_str().to_string(),
            id: event.id,
            platform_role_id: event.platform_role_id,
            role_name: event.role_name,
            target_user_id: event.target_user_id,
        }
    }
}
