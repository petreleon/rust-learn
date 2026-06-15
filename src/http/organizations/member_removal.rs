use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::organizations::remove_organization_member::{
    OrganizationMemberRemovalCommand, OrganizationMemberRemovalUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;

use super::errors::organization_member_removal_error;

pub(super) async fn remove_organization_member_route(
    actor: AuthUserId,
    path: web::Path<(i32, i32)>,
    use_case: web::Data<Arc<dyn OrganizationMemberRemovalUseCase>>,
) -> Result<(&'static str, StatusCode), ApiError> {
    let (organization_id, target_user_id) = path.into_inner();
    let actor_user_id = actor.into_inner();

    let command = OrganizationMemberRemovalCommand {
        actor_user_id,
        organization_id,
        target_user_id,
    };

    use_case
        .remove_organization_member(command)
        .await
        .map(|_| ("Member removed", StatusCode::OK))
        .map_err(|error| organization_member_removal_error(organization_id, target_user_id, error))
}
