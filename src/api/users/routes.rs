use super::{assign_role, get_user, list_users};
use crate::config::constants::permissions::Permissions;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use actix_web::web;

pub fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(
            web::resource("").route(web::get().to(list_users).wrap(
                PlatformPermissionMiddleware::require(Permissions::VIEW_USER.to_string()),
            )),
        )
        .service(web::resource("/{id}").route(web::get().to(get_user)))
        .service(
            web::resource("/{id}/role").route(web::post().to(assign_role).wrap(
                PlatformPermissionMiddleware::require(
                    Permissions::ASSIGN_ROLES_TO_USER.to_string(),
                ),
            )),
        )
}
