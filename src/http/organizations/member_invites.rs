use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::organizations::invite_organization_member::{
    OrganizationMemberInviteCommand, OrganizationMemberInviteError, OrganizationMemberInviteUseCase,
};
use crate::http::extractors::auth_user::AuthUserId;

use super::dto::AddMemberRequest;

pub(super) async fn add_member_by_email_route(
    actor: AuthUserId,
    path: web::Path<i32>,
    body: web::Json<AddMemberRequest>,
    use_case: web::Data<Arc<dyn OrganizationMemberInviteUseCase>>,
) -> impl Responder {
    let organization_id = path.into_inner();
    let actor_user_id = actor.into_inner();

    let command = OrganizationMemberInviteCommand {
        actor_user_id,
        organization_id,
        email: body.email.clone(),
        role_name: body.role_name.clone(),
    };

    match use_case.invite_organization_member(command).await {
        Ok(output) => HttpResponse::Ok().json(serde_json::json!({
            "user_id": output.user_id,
            "name": output.name,
            "email": output.email,
            "role": output.role,
        })),
        Err(OrganizationMemberInviteError::PermissionDenied) => HttpResponse::Forbidden()
            .body("User does not have the required permission within the organization"),
        Err(OrganizationMemberInviteError::UserNotFound) => {
            HttpResponse::NotFound().body("User not found by email")
        }
        Err(OrganizationMemberInviteError::HierarchyDenied) => {
            HttpResponse::Forbidden().body("Hierarchy check failed")
        }
        Err(OrganizationMemberInviteError::Connection(error)) => {
            log::error!(
                "event=org_member_add_connection_failed org_id={} email={} error={}",
                organization_id,
                body.email,
                error
            );
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(OrganizationMemberInviteError::RoleOrUserNotFound) => {
            log::error!(
                "event=org_member_add_role_lookup_failed org_id={} email={}",
                organization_id,
                body.email
            );
            HttpResponse::InternalServerError().body("Failed to add member")
        }
        Err(OrganizationMemberInviteError::Database(error)) => {
            log::error!(
                "event=org_member_add_failed org_id={} email={} error={}",
                organization_id,
                body.email,
                error
            );
            HttpResponse::InternalServerError().body("Failed to add member")
        }
    }
}
