use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::reporting::platform_fraud_dashboard::store::PlatformFraudDashboardStore;
use crate::application::reporting::platform_fraud_dashboard::{
    platform_fraud_dashboard_from_facts, PlatformFraudDashboardError, PlatformFraudDashboardOutput,
};
use crate::infra::postgres::reporting::platform_fraud_dashboard_blocks::active_fraud_block_facts;

pub struct PostgresPlatformFraudDashboardStore<'a> {
    conn: &'a mut AsyncPgConnection,
}

impl<'a> PostgresPlatformFraudDashboardStore<'a> {
    pub fn new(conn: &'a mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl PlatformFraudDashboardStore for PostgresPlatformFraudDashboardStore<'_> {
    fn load_platform_fraud_dashboard(
        &mut self,
    ) -> BoxFuture<'_, Result<PlatformFraudDashboardOutput, PlatformFraudDashboardError>> {
        async move {
            let active_blocks = active_fraud_block_facts(self.conn).await?;

            Ok(platform_fraud_dashboard_from_facts(active_blocks))
        }
        .boxed()
    }
}
