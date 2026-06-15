use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::notifications::delivery::{
    NotificationDeliveryUseCase, RoleAssignmentNotification, RoleAssignmentScope,
};
use crate::application::organizations::assign_organization_member_role::{
    OrganizationMemberRoleAssignmentCommand, OrganizationMemberRoleAssignmentUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;

use super::dto::AssignRoleRequest;
use super::errors::organization_member_role_assignment_error;

pub(super) async fn assign_role(
    actor: AuthUserId,
    path: web::Path<(i32, i32)>,
    body: web::Json<AssignRoleRequest>,
    use_case: web::Data<Arc<dyn OrganizationMemberRoleAssignmentUseCase>>,
    notifications: Option<web::Data<Arc<dyn NotificationDeliveryUseCase>>>,
) -> Result<(&'static str, StatusCode), ApiError> {
    let (organization_id, target_user_id) = path.into_inner();
    let actor_user_id = actor.into_inner();

    let command = OrganizationMemberRoleAssignmentCommand {
        actor_user_id,
        organization_id,
        target_user_id,
        role_name: body.role_name.clone(),
    };

    let output = use_case
        .assign_organization_member_role(command)
        .await
        .map_err(|error| {
            organization_member_role_assignment_error(
                error,
                organization_id,
                actor_user_id,
                target_user_id,
                &body.role_name,
            )
        })?;
    if let Some(notifications) = notifications {
        if let Err(err) = notifications
            .send_role_assignment(RoleAssignmentNotification {
                target_user_id: output.target_user_id,
                scope: RoleAssignmentScope::Organization,
                scope_id: output.organization_id,
                role_name: output.role_name.clone(),
            })
            .await
        {
            log::warn!(
                "event=notification_send_failed kind=role_assignment scope=organization organization_id={} target_user_id={} error={}",
                output.organization_id,
                output.target_user_id,
                err.message()
            );
        }
    }

    Ok(("Role assigned successfully", StatusCode::OK))
}
