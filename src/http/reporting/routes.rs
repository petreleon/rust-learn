use actix_web::web;

use crate::config::constants::permissions::Permissions;
use crate::http::reporting::handlers::{
    organization_summary, platform_fraud_dashboard, platform_summary,
};
use crate::middlewares::organization_permission_middleware::OrganizationPermissionMiddleware;
use crate::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::models::param_type::ParamType;

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

pub fn platform_fraud_dashboard_resource() -> actix_web::Resource {
    web::resource("/platform/fraud-dashboard").route(
        web::get()
            .to(platform_fraud_dashboard::get_platform_fraud_dashboard)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::VIEW_REWARD_AUDIT.to_string(),
            )),
    )
}

pub fn platform_fraud_dashboard_csv_resource() -> actix_web::Resource {
    web::resource("/platform/fraud-dashboard.csv").route(
        web::get()
            .to(platform_fraud_dashboard::export_platform_fraud_dashboard)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::EXPORT_DATA.to_string(),
            )),
    )
}

pub fn organization_summary_resource() -> actix_web::Resource {
    web::resource("/organizations/{id}/summary").route(
        web::get()
            .to(organization_summary::get_organization_summary)
            .wrap(OrganizationPermissionMiddleware::require(
                Permissions::VIEW_REPORT.to_string(),
                ParamType::Path,
                "id".to_string(),
            )),
    )
}

pub fn organization_summary_csv_resource() -> actix_web::Resource {
    web::resource("/organizations/{id}/summary.csv").route(
        web::get()
            .to(organization_summary::export_organization_summary)
            .wrap(OrganizationPermissionMiddleware::require(
                Permissions::GENERATE_REPORT.to_string(),
                ParamType::Path,
                "id".to_string(),
            )),
    )
}
