use actix_web::web;

pub fn reports_scope() -> actix_web::Scope {
    web::scope("/reports")
        .service(crate::http::reporting::platform_summary_resource())
        .service(crate::http::reporting::platform_summary_csv_resource())
        .service(crate::http::reporting::platform_reward_dashboard_resource())
        .service(crate::http::reporting::platform_reward_dashboard_csv_resource())
        .service(crate::http::reporting::platform_fraud_dashboard_resource())
        .service(crate::http::reporting::platform_fraud_dashboard_csv_resource())
        .service(crate::http::reporting::platform_wallet_reconciliation_resource())
        .service(crate::http::reporting::platform_teacher_applications_csv_resource())
        .service(crate::http::reporting::platform_reward_approvals_csv_resource())
        .service(crate::http::reporting::platform_token_payouts_csv_resource())
        .service(crate::http::reporting::platform_wallet_credits_csv_resource())
        .service(crate::http::reporting::platform_delegated_permissions_csv_resource())
        .service(crate::http::reporting::organization_summary_resource())
        .service(crate::http::reporting::organization_summary_csv_resource())
        .service(crate::http::reporting::organization_reward_dashboard_resource())
        .service(crate::http::reporting::organization_reward_dashboard_csv_resource())
}
