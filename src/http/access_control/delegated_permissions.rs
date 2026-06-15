use std::sync::Arc;

use actix_web::{http::StatusCode, web};

use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionOutput, DelegatedPermissionUseCase,
};
use crate::http::access_control::dto::{
    DelegatedPermissionResponse, GrantDelegatedPermissionRequest, ListDelegatedPermissionsParams,
    RevokeDelegatedPermissionRequest,
};
use crate::http::access_control::errors::delegated_permission_error;
use crate::http::errors::ApiError;
use crate::http::extractors::auth_user::AuthUser;

async fn grant_delegated_permission(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn DelegatedPermissionUseCase>>,
    body: web::Json<GrantDelegatedPermissionRequest>,
) -> Result<(web::Json<DelegatedPermissionResponse>, StatusCode), ApiError> {
    let command = body.into_inner().into_command(requester.user_id());
    use_case
        .grant_delegated_permission(command)
        .await
        .map(DelegatedPermissionResponse::from)
        .map(web::Json)
        .map(|body| (body, StatusCode::CREATED))
        .map_err(delegated_permission_error)
}

async fn list_delegated_permissions(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn DelegatedPermissionUseCase>>,
    query: web::Query<ListDelegatedPermissionsParams>,
) -> Result<web::Json<Vec<DelegatedPermissionResponse>>, ApiError> {
    let query = query.into_inner().into_query(requester.user_id());
    use_case
        .list_delegated_permissions(query)
        .await
        .map(delegated_permission_responses)
        .map(web::Json)
        .map_err(delegated_permission_error)
}

async fn revoke_delegated_permission(
    requester: AuthUser,
    path: web::Path<i64>,
    use_case: web::Data<Arc<dyn DelegatedPermissionUseCase>>,
    body: web::Json<RevokeDelegatedPermissionRequest>,
) -> Result<web::Json<DelegatedPermissionResponse>, ApiError> {
    let command = body
        .into_inner()
        .into_command(requester.user_id(), path.into_inner());
    use_case
        .revoke_delegated_permission(command)
        .await
        .map(DelegatedPermissionResponse::from)
        .map(web::Json)
        .map_err(delegated_permission_error)
}

fn delegated_permission_responses(
    delegations: Vec<DelegatedPermissionOutput>,
) -> Vec<DelegatedPermissionResponse> {
    delegations
        .into_iter()
        .map(DelegatedPermissionResponse::from)
        .collect()
}

pub(super) fn delegated_permission_scope() -> actix_web::Scope {
    web::scope("/delegated-permissions")
        .service(
            web::resource("")
                .route(web::post().to(grant_delegated_permission))
                .route(web::get().to(list_delegated_permissions)),
        )
        .service(web::resource("/{id}/revoke").route(web::put().to(revoke_delegated_permission)))
}
