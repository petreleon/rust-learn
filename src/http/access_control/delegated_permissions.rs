use std::sync::Arc;

use actix_web::{web, HttpResponse, Responder};

use crate::application::access_control::manage_delegated_permissions::{
    DelegatedPermissionError, DelegatedPermissionUseCase,
};
use crate::http::access_control::dto::{
    DelegatedPermissionResponse, GrantDelegatedPermissionRequest, ListDelegatedPermissionsParams,
    RevokeDelegatedPermissionRequest,
};
use crate::http::extractors::auth_user::AuthUser;

async fn grant_delegated_permission(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn DelegatedPermissionUseCase>>,
    body: web::Json<GrantDelegatedPermissionRequest>,
) -> impl Responder {
    let command = body.into_inner().into_command(requester.user_id());
    match use_case.grant_delegated_permission(command).await {
        Ok(delegation) => {
            HttpResponse::Created().json(DelegatedPermissionResponse::from(delegation))
        }
        Err(error) => delegated_permission_error_response(error),
    }
}

async fn list_delegated_permissions(
    requester: AuthUser,
    use_case: web::Data<Arc<dyn DelegatedPermissionUseCase>>,
    query: web::Query<ListDelegatedPermissionsParams>,
) -> impl Responder {
    let query = query.into_inner().into_query(requester.user_id());
    match use_case.list_delegated_permissions(query).await {
        Ok(delegations) => HttpResponse::Ok().json(delegated_permission_responses(delegations)),
        Err(error) => delegated_permission_error_response(error),
    }
}

async fn revoke_delegated_permission(
    requester: AuthUser,
    path: web::Path<i64>,
    use_case: web::Data<Arc<dyn DelegatedPermissionUseCase>>,
    body: web::Json<RevokeDelegatedPermissionRequest>,
) -> impl Responder {
    let command = body
        .into_inner()
        .into_command(requester.user_id(), path.into_inner());
    match use_case.revoke_delegated_permission(command).await {
        Ok(delegation) => HttpResponse::Ok().json(DelegatedPermissionResponse::from(delegation)),
        Err(error) => delegated_permission_error_response(error),
    }
}

fn delegated_permission_error_response(error: DelegatedPermissionError) -> HttpResponse {
    match error {
        DelegatedPermissionError::PermissionDenied(_) => {
            HttpResponse::Forbidden().body("User does not have delegated-permission access")
        }
        DelegatedPermissionError::InvalidInput(message) => HttpResponse::BadRequest().body(message),
        DelegatedPermissionError::NotFound => {
            HttpResponse::NotFound().body("Delegated permission not found")
        }
        DelegatedPermissionError::Connection(message)
        | DelegatedPermissionError::Database(message) => {
            log::error!("event=delegated_permission_api_failed error={}", message);
            HttpResponse::InternalServerError().body("Failed to process delegated permission")
        }
    }
}

fn delegated_permission_responses(
    delegations: Vec<
        crate::application::access_control::manage_delegated_permissions::DelegatedPermissionOutput,
    >,
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
