use futures::future::BoxFuture;

use crate::application::reporting::platform_fraud_dashboard::{
    PlatformFraudDashboardError, PlatformFraudDashboardOutput,
};

pub trait PlatformFraudDashboardStore {
    fn load_platform_fraud_dashboard(
        &mut self,
    ) -> BoxFuture<'_, Result<PlatformFraudDashboardOutput, PlatformFraudDashboardError>>;
}
