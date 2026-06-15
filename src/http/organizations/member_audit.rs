use std::sync::Arc;

use actix_web::web;

use crate::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditEventOutput, OrganizationMemberAuditQuery,
    OrganizationMemberAuditUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;

use super::errors::organization_member_audit_error;
use super::member_audit_dto::OrganizationMemberAuditEventResponse;

pub(super) async fn get_member_audit_route(
    requester: AuthUser,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn OrganizationMemberAuditUseCase>>,
) -> Result<web::Json<Vec<OrganizationMemberAuditEventResponse>>, ApiError> {
    let (organization_id, target_user_id) = path.into_inner();

    let query = OrganizationMemberAuditQuery {
        actor_user_id: requester.user_id(),
        organization_id,
        target_user_id,
    };

    use_case
        .list_member_audit(query)
        .await
        .map(audit_event_responses)
        .map(web::Json)
        .map_err(|error| organization_member_audit_error(organization_id, error))
}

fn audit_event_responses(
    events: Vec<OrganizationMemberAuditEventOutput>,
) -> Vec<OrganizationMemberAuditEventResponse> {
    events
        .into_iter()
        .map(OrganizationMemberAuditEventResponse::from)
        .collect()
}
