use actix_web::web;

use crate::domain::access_control::permissions::Permissions;
use crate::http::identity::{platform_role_assignment, user_handlers, user_list};
use crate::http::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;

pub(super) fn user_scope() -> actix_web::Scope {
    web::scope("/user")
        .service(
            web::resource("").route(web::get().to(user_list::list_users).wrap(
                PlatformPermissionMiddleware::require(Permissions::VIEW_USER.to_string()),
            )),
        )
        .service(web::resource("/{id}").route(web::get().to(user_handlers::get_user)))
        .service(web::resource("/{id}/role").route(
            web::post().to(platform_role_assignment::assign_role).wrap(
                PlatformPermissionMiddleware::require(
                    Permissions::ASSIGN_ROLES_TO_USER.to_string(),
                ),
            ),
        ))
}
