use futures::future::BoxFuture;

use crate::application::reporting::platform_fraud_dashboard::{
    PlatformFraudDashboardError, PlatformFraudDashboardOutput,
};

pub trait PlatformFraudDashboardUseCase: Send + Sync {
    fn load_platform_fraud_dashboard(
        &self,
    ) -> BoxFuture<'_, Result<PlatformFraudDashboardOutput, PlatformFraudDashboardError>>;
}
