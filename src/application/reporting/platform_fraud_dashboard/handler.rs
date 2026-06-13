use crate::application::reporting::platform_fraud_dashboard::store::PlatformFraudDashboardStore;
use crate::application::reporting::platform_fraud_dashboard::{
    PlatformFraudDashboardError, PlatformFraudDashboardOutput,
};

pub async fn load_platform_fraud_dashboard(
    store: &mut impl PlatformFraudDashboardStore,
) -> Result<PlatformFraudDashboardOutput, PlatformFraudDashboardError> {
    store.load_platform_fraud_dashboard().await
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use futures::executor::block_on;
    use futures::future::{ready, BoxFuture, FutureExt};

    use super::*;
    use crate::application::reporting::platform_fraud_dashboard::{
        FraudBlockDashboardRowOutput, FraudBlockScopeSummaryOutput,
    };

    #[test]
    fn loads_platform_fraud_dashboard_through_store_port() {
        let mut store = FakePlatformFraudDashboardStore { called: false };

        let output = block_on(load_platform_fraud_dashboard(&mut store))
            .expect("platform fraud dashboard should load");

        assert!(store.called);
        assert_eq!(output.active_total, 1);
        assert_eq!(output.active_by_scope.teacher, 1);
        assert_eq!(output.active_blocks[0].reason, "teacher pause");
    }

    struct FakePlatformFraudDashboardStore {
        called: bool,
    }

    impl PlatformFraudDashboardStore for FakePlatformFraudDashboardStore {
        fn load_platform_fraud_dashboard(
            &mut self,
        ) -> BoxFuture<'_, Result<PlatformFraudDashboardOutput, PlatformFraudDashboardError>>
        {
            self.called = true;
            let now = Utc::now();
            ready(Ok(PlatformFraudDashboardOutput {
                active_total: 1,
                active_by_scope: FraudBlockScopeSummaryOutput {
                    teacher: 1,
                    ..Default::default()
                },
                active_blocks: vec![FraudBlockDashboardRowOutput {
                    id: 7,
                    scope_type: "teacher".to_string(),
                    teacher_user_id: Some(42),
                    organization_id: None,
                    course_id: None,
                    reward_policy_id: None,
                    reason: "teacher pause".to_string(),
                    evidence_reference: None,
                    created_by_user_id: 1,
                    expires_at: None,
                    created_at: now,
                    updated_at: now,
                }],
            }))
            .boxed()
        }
    }
}
