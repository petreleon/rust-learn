use std::sync::Arc;

use actix_web::web;
use serde::Deserialize;

use crate::application::identity::assign_platform_role::{
    AssignPlatformRoleCommand, PlatformRoleAssignmentUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;
use crate::http::identity::errors::platform_role_assignment_error;

#[derive(Deserialize)]
pub(super) struct AssignRoleRequest {
    pub role_name: String,
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
