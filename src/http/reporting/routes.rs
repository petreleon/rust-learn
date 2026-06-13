use actix_web::web;

use crate::config::constants::permissions::Permissions;
use crate::http::reporting::handlers::platform_summary;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;

pub fn platform_summary_resource() -> actix_web::Resource {
    web::resource("/platform/summary").route(
        web::get().to(platform_summary::get_platform_summary).wrap(
            PlatformPermissionMiddleware::require(Permissions::VIEW_REPORT.to_string()),
        ),
    )
}

pub fn platform_summary_csv_resource() -> actix_web::Resource {
    web::resource("/platform/summary.csv").route(
        web::get()
            .to(platform_summary::export_platform_summary)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::EXPORT_DATA.to_string(),
            )),
    )
}
