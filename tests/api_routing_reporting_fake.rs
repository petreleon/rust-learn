use std::sync::Arc;

use actix_web::web;
use chrono::NaiveDate;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::reporting::organization_reward_dashboard::{
    OrganizationRewardDashboardError, OrganizationRewardDashboardOutput,
    OrganizationRewardDashboardUseCase,
};
use rust_learn::application::reporting::organization_summary::{
    OrganizationSummaryError, OrganizationSummaryOutput, OrganizationSummaryUseCase,
};
use rust_learn::application::reporting::platform_fraud_dashboard::{
    PlatformFraudDashboardError, PlatformFraudDashboardOutput, PlatformFraudDashboardUseCase,
};
use rust_learn::application::reporting::platform_reward_dashboard::{
    PlatformRewardDashboardError, PlatformRewardDashboardOutput, PlatformRewardDashboardUseCase,
};
use rust_learn::application::reporting::platform_wallet_reconciliation::{
    PlatformWalletReconciliationError, PlatformWalletReconciliationOutput,
    PlatformWalletReconciliationUseCase,
};

struct RouteOnlyOrganizationRewardDashboardUseCase;
struct RouteOnlyOrganizationSummaryUseCase;
struct RouteOnlyPlatformFraudDashboardUseCase;
struct RouteOnlyPlatformRewardDashboardUseCase;
struct RouteOnlyPlatformWalletReconciliationUseCase;

pub fn organization_reward_dashboard_data() -> web::Data<Arc<dyn OrganizationRewardDashboardUseCase>>
{
    web::Data::new(Arc::new(RouteOnlyOrganizationRewardDashboardUseCase)
        as Arc<dyn OrganizationRewardDashboardUseCase>)
}

pub fn organization_summary_data() -> web::Data<Arc<dyn OrganizationSummaryUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyOrganizationSummaryUseCase) as Arc<dyn OrganizationSummaryUseCase>
    )
}

pub fn platform_fraud_dashboard_data() -> web::Data<Arc<dyn PlatformFraudDashboardUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyPlatformFraudDashboardUseCase) as Arc<dyn PlatformFraudDashboardUseCase>
    )
}

pub fn platform_reward_dashboard_data() -> web::Data<Arc<dyn PlatformRewardDashboardUseCase>> {
    web::Data::new(Arc::new(RouteOnlyPlatformRewardDashboardUseCase)
        as Arc<dyn PlatformRewardDashboardUseCase>)
}

pub fn platform_wallet_reconciliation_data(
) -> web::Data<Arc<dyn PlatformWalletReconciliationUseCase>> {
    web::Data::new(Arc::new(RouteOnlyPlatformWalletReconciliationUseCase)
        as Arc<dyn PlatformWalletReconciliationUseCase>)
}

impl OrganizationRewardDashboardUseCase for RouteOnlyOrganizationRewardDashboardUseCase {
    fn load_organization_reward_dashboard(
        &self,
        _organization_id: i32,
        _from: Option<NaiveDate>,
        _to: Option<NaiveDate>,
    ) -> BoxFuture<'_, Result<OrganizationRewardDashboardOutput, OrganizationRewardDashboardError>>
    {
        ready(Err(OrganizationRewardDashboardError::Database(
            "route-only use case".to_string(),
        )))
        .boxed()
    }
}

impl OrganizationSummaryUseCase for RouteOnlyOrganizationSummaryUseCase {
    fn load_organization_summary(
        &self,
        _organization_id: i32,
    ) -> BoxFuture<'_, Result<OrganizationSummaryOutput, OrganizationSummaryError>> {
        ready(Err(OrganizationSummaryError::Database(
            "route-only use case".to_string(),
        )))
        .boxed()
    }
}

impl PlatformFraudDashboardUseCase for RouteOnlyPlatformFraudDashboardUseCase {
    fn load_platform_fraud_dashboard(
        &self,
    ) -> BoxFuture<'_, Result<PlatformFraudDashboardOutput, PlatformFraudDashboardError>> {
        ready(Err(PlatformFraudDashboardError::Database(
            "route-only use case".to_string(),
        )))
        .boxed()
    }
}

impl PlatformRewardDashboardUseCase for RouteOnlyPlatformRewardDashboardUseCase {
    fn load_platform_reward_dashboard(
        &self,
    ) -> BoxFuture<'_, Result<PlatformRewardDashboardOutput, PlatformRewardDashboardError>> {
        ready(Err(PlatformRewardDashboardError::Database(
            "route-only use case".to_string(),
        )))
        .boxed()
    }
}

impl PlatformWalletReconciliationUseCase for RouteOnlyPlatformWalletReconciliationUseCase {
    fn load_platform_wallet_reconciliation(
        &self,
    ) -> BoxFuture<'_, Result<PlatformWalletReconciliationOutput, PlatformWalletReconciliationError>>
    {
        ready(Err(PlatformWalletReconciliationError::Database(
            "route-only use case".to_string(),
        )))
        .boxed()
    }
}
