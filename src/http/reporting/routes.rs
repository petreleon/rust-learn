use actix_web::web;

use crate::config::constants::permissions::Permissions;
use crate::http::middlewares::organization_permission_middleware::OrganizationPermissionMiddleware;
use crate::http::middlewares::platform_permission_middleware::PlatformPermissionMiddleware;
use crate::http::reporting::handlers::{
    organization_reward_dashboard, organization_summary, platform_csv_exports,
    platform_fraud_dashboard, platform_reward_dashboard, platform_summary,
    platform_wallet_reconciliation,
};
use crate::http::request_params::ParamType;

pub(super) fn platform_summary_resource() -> actix_web::Resource {
    web::resource("/platform/summary").route(
        web::get().to(platform_summary::get_platform_summary).wrap(
            PlatformPermissionMiddleware::require(Permissions::VIEW_REPORT.to_string()),
        ),
    )
}

pub(super) fn platform_summary_csv_resource() -> actix_web::Resource {
    web::resource("/platform/summary.csv").route(
        web::get()
            .to(platform_summary::export_platform_summary)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::EXPORT_DATA.to_string(),
            )),
    )
}

pub(super) fn platform_reward_dashboard_resource() -> actix_web::Resource {
    web::resource("/platform/reward-dashboard").route(
        web::get()
            .to(platform_reward_dashboard::get_platform_reward_dashboard)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::VIEW_REWARD_AUDIT.to_string(),
            )),
    )
}

pub(super) fn platform_reward_dashboard_csv_resource() -> actix_web::Resource {
    web::resource("/platform/reward-dashboard.csv").route(
        web::get()
            .to(platform_reward_dashboard::export_platform_reward_dashboard)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::EXPORT_DATA.to_string(),
            )),
    )
}

pub(super) fn platform_fraud_dashboard_resource() -> actix_web::Resource {
    web::resource("/platform/fraud-dashboard").route(
        web::get()
            .to(platform_fraud_dashboard::get_platform_fraud_dashboard)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::VIEW_REWARD_AUDIT.to_string(),
            )),
    )
}

pub(super) fn platform_fraud_dashboard_csv_resource() -> actix_web::Resource {
    web::resource("/platform/fraud-dashboard.csv").route(
        web::get()
            .to(platform_fraud_dashboard::export_platform_fraud_dashboard)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::EXPORT_DATA.to_string(),
            )),
    )
}

pub(super) fn platform_wallet_reconciliation_resource() -> actix_web::Resource {
    web::resource("/platform/wallet-reconciliation").route(
        web::get()
            .to(platform_wallet_reconciliation::get_platform_wallet_reconciliation)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::MANAGE_WALLETS.to_string(),
            )),
    )
}

pub(super) fn platform_teacher_applications_csv_resource() -> actix_web::Resource {
    web::resource("/platform/teacher-applications.csv").route(
        web::get()
            .to(platform_csv_exports::export_platform_teacher_applications)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::EXPORT_DATA.to_string(),
            )),
    )
}

pub(super) fn platform_reward_approvals_csv_resource() -> actix_web::Resource {
    web::resource("/platform/reward-approvals.csv").route(
        web::get()
            .to(platform_csv_exports::export_platform_reward_approvals)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::EXPORT_DATA.to_string(),
            )),
    )
}

pub(super) fn platform_token_payouts_csv_resource() -> actix_web::Resource {
    web::resource("/platform/token-payouts.csv").route(
        web::get()
            .to(platform_csv_exports::export_platform_token_payouts)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::EXPORT_DATA.to_string(),
            )),
    )
}

pub(super) fn platform_wallet_credits_csv_resource() -> actix_web::Resource {
    web::resource("/platform/wallet-credits.csv").route(
        web::get()
            .to(platform_csv_exports::export_platform_wallet_credits)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::EXPORT_DATA.to_string(),
            )),
    )
}

pub(super) fn platform_delegated_permissions_csv_resource() -> actix_web::Resource {
    web::resource("/platform/delegated-permissions.csv").route(
        web::get()
            .to(platform_csv_exports::export_platform_delegated_permissions)
            .wrap(PlatformPermissionMiddleware::require(
                Permissions::EXPORT_DATA.to_string(),
            )),
    )
}

pub(super) fn organization_summary_resource() -> actix_web::Resource {
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

pub(super) fn organization_summary_csv_resource() -> actix_web::Resource {
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

pub(super) fn organization_reward_dashboard_resource() -> actix_web::Resource {
    web::resource("/organizations/{id}/reward-dashboard").route(
        web::get()
            .to(organization_reward_dashboard::get_organization_reward_dashboard)
            .wrap(OrganizationPermissionMiddleware::require(
                Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
                ParamType::Path,
                "id".to_string(),
            )),
    )
}

pub(super) fn organization_reward_dashboard_csv_resource() -> actix_web::Resource {
    web::resource("/organizations/{id}/reward-dashboard.csv").route(
        web::get()
            .to(organization_reward_dashboard::export_organization_reward_dashboard)
            .wrap(OrganizationPermissionMiddleware::require(
                Permissions::VIEW_ORG_REWARD_REPORTS.to_string(),
                ParamType::Path,
                "id".to_string(),
            )),
    )
}
