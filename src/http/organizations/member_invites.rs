use std::sync::Arc;

use actix_web::web;

use crate::application::organizations::invite_organization_member::{
    OrganizationMemberInviteCommand, OrganizationMemberInviteUseCase,
};
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUserId;

use super::dto::AddMemberRequest;
use super::errors::organization_member_invite_error;

pub(super) async fn add_member_by_email_route(
    actor: AuthUserId,
    path: web::Path<i32>,
    body: web::Json<AddMemberRequest>,
    use_case: web::Data<Arc<dyn OrganizationMemberInviteUseCase>>,
) -> Result<web::Json<serde_json::Value>, ApiError> {
    let organization_id = path.into_inner();
    let actor_user_id = actor.into_inner();

    let command = OrganizationMemberInviteCommand {
        actor_user_id,
        organization_id,
        email: body.email.clone(),
        role_name: body.role_name.clone(),
    };

    use_case
        .invite_organization_member(command)
        .await
        .map(|output| {
            web::Json(serde_json::json!({
            "user_id": output.user_id,
            "name": output.name,
            "email": output.email,
            "role": output.role,
            }))
        })
        .map_err(|error| organization_member_invite_error(organization_id, &body.email, error))
}
