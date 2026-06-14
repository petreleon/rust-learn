use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::application::organizations::assign_organization_member_role::{
    OrganizationMemberRoleAssignmentCommand, OrganizationMemberRoleAssignmentError,
    OrganizationMemberRoleAssignmentUseCase,
};
use crate::http::extractors::request_auth::authenticated_user_id;
use crate::infra::notifications::NotificationsState;

use super::dto::AssignRoleRequest;

pub(super) async fn assign_role(
    req: HttpRequest,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssignRoleRequest>,
    use_case: web::Data<Arc<dyn OrganizationMemberRoleAssignmentUseCase>>,
) -> impl Responder {
    let (organization_id, target_user_id) = path.into_inner();
    let actor_user_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    let command = OrganizationMemberRoleAssignmentCommand {
        actor_user_id,
        organization_id,
        target_user_id,
        role_name: body.role_name.clone(),
    };

    match use_case.assign_organization_member_role(command).await {
        Ok(output) => {
            if let Some(notifications) = req.app_data::<web::Data<NotificationsState>>() {
                if let Err(err) = notifications
                    .send_role_assignment_notification(
                        output.target_user_id,
                        "organization",
                        Some(output.organization_id),
                        &output.role_name,
                    )
                    .await
                {
                    log::warn!(
                        "event=notification_send_failed kind=role_assignment scope=organization organization_id={} target_user_id={} error={:?}",
                        output.organization_id,
                        output.target_user_id,
                        err
                    );
                }
            }

            HttpResponse::Ok().body("Role assigned successfully")
        }
        Err(error) => organization_member_role_assignment_error_response(
            error,
            organization_id,
            actor_user_id,
            target_user_id,
            &body.role_name,
        ),
    }
}

fn organization_member_role_assignment_error_response(
    error: OrganizationMemberRoleAssignmentError,
    organization_id: i32,
    actor_user_id: i32,
    target_user_id: i32,
    role_name: &str,
) -> HttpResponse {
    match error {
        OrganizationMemberRoleAssignmentError::PermissionDenied => HttpResponse::Forbidden()
            .body("User does not have the required permission within the organization"),
        OrganizationMemberRoleAssignmentError::HierarchyViolation => HttpResponse::Forbidden()
            .body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank."),
        OrganizationMemberRoleAssignmentError::NotFound => {
            HttpResponse::BadRequest().body("Role or User not found")
        }
        OrganizationMemberRoleAssignmentError::Connection(message) => {
            log::error!(
                "event=organization_role_assign_connection_failed organization_id={} requester_user_id={} target_user_id={} role={} error={}",
                organization_id,
                actor_user_id,
                target_user_id,
                role_name,
                message
            );
            HttpResponse::InternalServerError().body("Failed to get DB connection")
        }
        OrganizationMemberRoleAssignmentError::Database(message) => {
            log::error!(
                "event=organization_role_assign_failed organization_id={} requester_user_id={} target_user_id={} role={} error={}",
                organization_id,
                actor_user_id,
                target_user_id,
                role_name,
                message
            );
            HttpResponse::InternalServerError().body("Failed to assign role")
        }
    }
}
