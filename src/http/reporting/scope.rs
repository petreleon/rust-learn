use actix_web::{web, Scope};

use crate::http::reporting::routes;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(reports_scope());
}

fn reports_scope() -> Scope {
    web::scope("/reports")
        .service(routes::platform_summary_resource())
        .service(routes::platform_summary_csv_resource())
        .service(routes::platform_reward_dashboard_resource())
        .service(routes::platform_reward_dashboard_csv_resource())
        .service(routes::platform_fraud_dashboard_resource())
        .service(routes::platform_fraud_dashboard_csv_resource())
        .service(routes::platform_wallet_reconciliation_resource())
        .service(routes::platform_teacher_applications_csv_resource())
        .service(routes::platform_reward_approvals_csv_resource())
        .service(routes::platform_token_payouts_csv_resource())
        .service(routes::platform_wallet_credits_csv_resource())
        .service(routes::platform_delegated_permissions_csv_resource())
        .service(routes::organization_summary_resource())
        .service(routes::organization_summary_csv_resource())
        .service(routes::organization_reward_dashboard_resource())
        .service(routes::organization_reward_dashboard_csv_resource())
}
