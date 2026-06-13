use actix_web::web;

use crate::config::constants::permissions::Permissions;
use crate::http::access_control::handlers;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;

pub fn roles_scope() -> actix_web::Scope {
    web::scope("/roles")
        .service(
            web::resource("").route(web::get().to(handlers::list_platform_roles).wrap(
                PlatformPermissionMiddleware::require(
                    Permissions::VIEW_ROLE_ASSIGNMENTS.to_string(),
                ),
            )),
        )
        .service(web::resource("/organization").route(
            web::get().to(handlers::list_organization_roles).wrap(
                PlatformPermissionMiddleware::require(
                    Permissions::VIEW_ROLE_ASSIGNMENTS.to_string(),
                ),
            ),
        ))
        .service(
            web::resource("/course").route(web::get().to(handlers::list_course_roles).wrap(
                PlatformPermissionMiddleware::require(
                    Permissions::VIEW_ROLE_ASSIGNMENTS.to_string(),
                ),
            )),
        )
}
