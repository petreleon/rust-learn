use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::reporting::organization_summary::{
    OrganizationSummaryError, OrganizationSummaryOutput, OrganizationSummaryUseCase,
};
use rust_learn::application::reporting::platform_fraud_dashboard::{
    PlatformFraudDashboardError, PlatformFraudDashboardOutput, PlatformFraudDashboardUseCase,
};

struct RouteOnlyOrganizationSummaryUseCase;
struct RouteOnlyPlatformFraudDashboardUseCase;

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
