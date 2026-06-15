use std::sync::Arc;

use actix_web::web;

use crate::application::reporting::organization_reward_dashboard::OrganizationRewardDashboardUseCase;
use crate::application::reporting::organization_summary::OrganizationSummaryUseCase;
use crate::application::reporting::platform_csv_exports::PlatformCsvExportsUseCase;
use crate::application::reporting::platform_fraud_dashboard::PlatformFraudDashboardUseCase;
use crate::application::reporting::platform_reward_dashboard::PlatformRewardDashboardUseCase;
use crate::application::reporting::platform_summary::PlatformSummaryUseCase;
use crate::application::reporting::platform_wallet_reconciliation::PlatformWalletReconciliationUseCase;
use crate::infra::postgres::reporting::organization_reward_dashboard_use_case::PostgresOrganizationRewardDashboardUseCase;
use crate::infra::postgres::reporting::organization_summary_use_case::PostgresOrganizationSummaryUseCase;
use crate::infra::postgres::reporting::platform_csv_export_use_case::PostgresPlatformCsvExportsUseCase;
use crate::infra::postgres::reporting::platform_fraud_dashboard_use_case::PostgresPlatformFraudDashboardUseCase;
use crate::infra::postgres::reporting::platform_reward_dashboard_use_case::PostgresPlatformRewardDashboardUseCase;
use crate::infra::postgres::reporting::platform_summary_use_case::PostgresPlatformSummaryUseCase;
use crate::infra::postgres::reporting::platform_wallet_reconciliation_use_case::PostgresPlatformWalletReconciliationUseCase;
use crate::infra::postgres::DbPool;

#[derive(Clone)]
pub struct ReportingUseCases {
    pub organization_reward_dashboard: Arc<dyn OrganizationRewardDashboardUseCase>,
    pub organization_summary: Arc<dyn OrganizationSummaryUseCase>,
    pub platform_csv_exports: Arc<dyn PlatformCsvExportsUseCase>,
    pub platform_fraud_dashboard: Arc<dyn PlatformFraudDashboardUseCase>,
    pub platform_reward_dashboard: Arc<dyn PlatformRewardDashboardUseCase>,
    pub platform_summary: Arc<dyn PlatformSummaryUseCase>,
    pub platform_wallet_reconciliation: Arc<dyn PlatformWalletReconciliationUseCase>,
}

pub fn build_reporting_use_cases(pool: &DbPool) -> ReportingUseCases {
    ReportingUseCases {
        organization_reward_dashboard: Arc::new(PostgresOrganizationRewardDashboardUseCase::new(
            pool.clone(),
        )),
        organization_summary: Arc::new(PostgresOrganizationSummaryUseCase::new(pool.clone())),
        platform_csv_exports: Arc::new(PostgresPlatformCsvExportsUseCase::new(pool.clone())),
        platform_fraud_dashboard: Arc::new(PostgresPlatformFraudDashboardUseCase::new(
            pool.clone(),
        )),
        platform_reward_dashboard: Arc::new(PostgresPlatformRewardDashboardUseCase::new(
            pool.clone(),
        )),
        platform_summary: Arc::new(PostgresPlatformSummaryUseCase::new(pool.clone())),
        platform_wallet_reconciliation: Arc::new(PostgresPlatformWalletReconciliationUseCase::new(
            pool.clone(),
        )),
    }
}

pub fn configure_reporting_app_data(cfg: &mut web::ServiceConfig, use_cases: &ReportingUseCases) {
    cfg.app_data(web::Data::new(
        use_cases.organization_reward_dashboard.clone(),
    ))
    .app_data(web::Data::new(use_cases.organization_summary.clone()))
    .app_data(web::Data::new(use_cases.platform_csv_exports.clone()))
    .app_data(web::Data::new(use_cases.platform_fraud_dashboard.clone()))
    .app_data(web::Data::new(use_cases.platform_reward_dashboard.clone()))
    .app_data(web::Data::new(use_cases.platform_summary.clone()))
    .app_data(web::Data::new(
        use_cases.platform_wallet_reconciliation.clone(),
    ));
}
