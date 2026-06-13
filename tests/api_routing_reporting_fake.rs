use std::sync::Arc;

use actix_web::web;
use futures::future::{ready, BoxFuture, FutureExt};
use rust_learn::application::reporting::platform_fraud_dashboard::{
    PlatformFraudDashboardError, PlatformFraudDashboardOutput, PlatformFraudDashboardUseCase,
};

struct RouteOnlyPlatformFraudDashboardUseCase;

pub fn platform_fraud_dashboard_data() -> web::Data<Arc<dyn PlatformFraudDashboardUseCase>> {
    web::Data::new(
        Arc::new(RouteOnlyPlatformFraudDashboardUseCase) as Arc<dyn PlatformFraudDashboardUseCase>
    )
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
