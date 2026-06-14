use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;

use crate::application::identity::assign_platform_role::{
    AssignPlatformRoleCommand, AssignPlatformRoleError, PlatformRoleAssignmentUseCase,
};
use crate::http::extractors::request_auth::authenticated_user_id;

#[derive(Deserialize)]
pub(super) struct AssignRoleRequest {
    pub role_name: String,
}

pub(super) async fn assign_role(
    req: HttpRequest,
    path: web::Path<i32>,
    body: web::Json<AssignRoleRequest>,
    use_case: web::Data<Arc<dyn PlatformRoleAssignmentUseCase>>,
) -> impl Responder {
    let target_user_id = path.into_inner();
    let role_name = body.into_inner().role_name;

    let requester_id = match authenticated_user_id(&req) {
        Ok(user_id) => user_id,
        Err(response) => return response,
    };

    let command = AssignPlatformRoleCommand {
        requester_user_id: requester_id,
        target_user_id,
        role_name: role_name.clone(),
    };
    match use_case.assign_platform_role(command).await {
        Ok(_) => HttpResponse::Ok().body("Role assigned successfully"),
        Err(AssignPlatformRoleError::HierarchyViolation) => HttpResponse::Forbidden().body("Hierarchy check failed: Cannot assign role higher than or equal to your own, or modify user with higher/equal rank."),
        Err(AssignPlatformRoleError::RoleNotFound) => {
            HttpResponse::BadRequest().body(format!("Role '{}' not found", role_name))
        }
        Err(error @ (AssignPlatformRoleError::Connection(_) | AssignPlatformRoleError::Database(_))) => {
            log::error!(
                "event=platform_role_assign_failed requester_user_id={} target_user_id={} role={} error={}",
                requester_id,
                target_user_id,
                role_name,
                role_assignment_error_log(&error)
            );
            HttpResponse::InternalServerError().body("Failed to assign role")
        }
    }
}

fn role_assignment_error_log(error: &AssignPlatformRoleError) -> String {
    match error {
        AssignPlatformRoleError::Connection(message)
        | AssignPlatformRoleError::Database(message) => message.clone(),
        AssignPlatformRoleError::HierarchyViolation => "hierarchy_violation".to_string(),
        AssignPlatformRoleError::RoleNotFound => "role_not_found".to_string(),
    }
}
