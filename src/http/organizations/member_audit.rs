use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::organizations::list_organization_member_audit::{
    OrganizationMemberAuditError, OrganizationMemberAuditEventOutput, OrganizationMemberAuditQuery,
    OrganizationMemberAuditUseCase,
};
use crate::utils::request_auth::authenticated_user;

use super::member_audit_dto::OrganizationMemberAuditEventResponse;

pub(super) async fn get_member_audit_route(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn OrganizationMemberAuditUseCase>>,
) -> impl Responder {
    let requester = match authenticated_user(&req) {
        Ok(user) => user,
        Err(response) => return response,
    };
    let (organization_id, target_user_id) = path.into_inner();

    let query = OrganizationMemberAuditQuery {
        actor_user_id: requester.user_id,
        organization_id,
        target_user_id,
    };

    match use_case.list_member_audit(query).await {
        Ok(events) => HttpResponse::Ok().json(audit_event_responses(events)),
        Err(error) => organization_member_audit_error_response(error, organization_id),
    }
}

fn organization_member_audit_error_response(
    error: OrganizationMemberAuditError,
    organization_id: i32,
) -> HttpResponse {
    match error {
        OrganizationMemberAuditError::PermissionDenied(_) => HttpResponse::Forbidden()
            .body("User does not have permission to view organization member audit"),
        OrganizationMemberAuditError::Connection(error) => {
            log::error!(
                "event=member_audit_connection_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        OrganizationMemberAuditError::Database(error) => {
            log::error!(
                "event=member_audit_fetch_failed organization_id={} error={}",
                organization_id,
                error
            );
            HttpResponse::InternalServerError().body("Failed to load audit events")
        }
    }
}

fn audit_event_responses(
    events: Vec<OrganizationMemberAuditEventOutput>,
) -> Vec<OrganizationMemberAuditEventResponse> {
    events
        .into_iter()
        .map(OrganizationMemberAuditEventResponse::from)
        .collect()
}
