use std::sync::Arc;

use crate::support::*;

use rust_learn::application::reporting::organization_reward_dashboard::OrganizationRewardDashboardUseCase;
use rust_learn::application::reporting::organization_summary::OrganizationSummaryUseCase;
use rust_learn::application::reporting::platform_csv_exports::PlatformCsvExportsUseCase;
use rust_learn::application::reporting::platform_fraud_dashboard::PlatformFraudDashboardUseCase;
use rust_learn::application::reporting::platform_reward_dashboard::PlatformRewardDashboardUseCase;
use rust_learn::application::reporting::platform_summary::PlatformSummaryUseCase;
use rust_learn::application::reporting::platform_wallet_reconciliation::PlatformWalletReconciliationUseCase;
use rust_learn::infra::postgres::reporting::organization_reward_dashboard_use_case::PostgresOrganizationRewardDashboardUseCase;
use rust_learn::infra::postgres::reporting::organization_summary_use_case::PostgresOrganizationSummaryUseCase;
use rust_learn::infra::postgres::reporting::platform_csv_export_use_case::PostgresPlatformCsvExportsUseCase;
use rust_learn::infra::postgres::reporting::platform_fraud_dashboard_use_case::PostgresPlatformFraudDashboardUseCase;
use rust_learn::infra::postgres::reporting::platform_reward_dashboard_use_case::PostgresPlatformRewardDashboardUseCase;
use rust_learn::infra::postgres::reporting::platform_summary_use_case::PostgresPlatformSummaryUseCase;
use rust_learn::infra::postgres::reporting::platform_wallet_reconciliation_use_case::PostgresPlatformWalletReconciliationUseCase;

pub(crate) fn platform_summary_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn PlatformSummaryUseCase>> {
    web::Data::new(Arc::new(PostgresPlatformSummaryUseCase::new(pool.clone()))
        as Arc<dyn PlatformSummaryUseCase>)
}

pub(crate) fn organization_summary_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationSummaryUseCase>> {
    web::Data::new(
        Arc::new(PostgresOrganizationSummaryUseCase::new(pool.clone()))
            as Arc<dyn OrganizationSummaryUseCase>,
    )
}

pub(crate) fn organization_reward_dashboard_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn OrganizationRewardDashboardUseCase>> {
    web::Data::new(Arc::new(PostgresOrganizationRewardDashboardUseCase::new(
        pool.clone(),
    )) as Arc<dyn OrganizationRewardDashboardUseCase>)
}

pub(crate) fn platform_fraud_dashboard_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn PlatformFraudDashboardUseCase>> {
    web::Data::new(
        Arc::new(PostgresPlatformFraudDashboardUseCase::new(pool.clone()))
            as Arc<dyn PlatformFraudDashboardUseCase>,
    )
}

pub(crate) fn platform_reward_dashboard_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn PlatformRewardDashboardUseCase>> {
    web::Data::new(
        Arc::new(PostgresPlatformRewardDashboardUseCase::new(pool.clone()))
            as Arc<dyn PlatformRewardDashboardUseCase>,
    )
}

pub(crate) fn platform_csv_exports_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn PlatformCsvExportsUseCase>> {
    web::Data::new(
        Arc::new(PostgresPlatformCsvExportsUseCase::new(pool.clone()))
            as Arc<dyn PlatformCsvExportsUseCase>,
    )
}

pub(crate) fn platform_wallet_reconciliation_use_case(
    pool: &DbPool,
) -> web::Data<Arc<dyn PlatformWalletReconciliationUseCase>> {
    web::Data::new(Arc::new(PostgresPlatformWalletReconciliationUseCase::new(
        pool.clone(),
    )) as Arc<dyn PlatformWalletReconciliationUseCase>)
}
