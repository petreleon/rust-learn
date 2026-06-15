use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::organizations::remove_organization_member::{
    OrganizationMemberRemovalCommand, OrganizationMemberRemovalError,
    OrganizationMemberRemovalUseCase,
};
use crate::http::extractors::auth_user::AuthUserId;

pub(super) async fn remove_organization_member_route(
    actor: AuthUserId,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn OrganizationMemberRemovalUseCase>>,
) -> impl Responder {
    let (organization_id, target_user_id) = path.into_inner();
    let actor_user_id = actor.into_inner();

    let command = OrganizationMemberRemovalCommand {
        actor_user_id,
        organization_id,
        target_user_id,
    };

    match use_case.remove_organization_member(command).await {
        Ok(_) => HttpResponse::Ok().body("Member removed"),
        Err(OrganizationMemberRemovalError::PermissionDenied) => HttpResponse::Forbidden()
            .body("User does not have permission to remove organization members"),
        Err(OrganizationMemberRemovalError::NotFound) => {
            HttpResponse::NotFound().body("User not found in organization")
        }
        Err(OrganizationMemberRemovalError::Connection(error)) => {
            log::error!(
                "event=organization_member_remove_connection_failed organization_id={} target_user_id={} error={}",
                organization_id, target_user_id, error
            );
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        Err(OrganizationMemberRemovalError::Database(error)) => {
            log::error!(
                "event=organization_member_remove_failed organization_id={} target_user_id={} error={}",
                organization_id, target_user_id, error
            );
            HttpResponse::InternalServerError().body("Failed to remove member")
        }
    }
}
