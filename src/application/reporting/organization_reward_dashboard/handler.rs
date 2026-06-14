use chrono::NaiveDate;

use crate::application::reporting::organization_reward_dashboard::store::OrganizationRewardDashboardStore;
use crate::application::reporting::organization_reward_dashboard::{
    OrganizationRewardDashboardError, OrganizationRewardDashboardOutput,
};

pub async fn load_organization_reward_dashboard(
    store: &mut impl OrganizationRewardDashboardStore,
    organization_id: i32,
    from: Option<NaiveDate>,
    to: Option<NaiveDate>,
) -> Result<OrganizationRewardDashboardOutput, OrganizationRewardDashboardError> {
    store
        .load_organization_reward_dashboard(organization_id, from, to)
        .await
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use futures::executor::block_on;
    use futures::future::{ready, BoxFuture, FutureExt};

    use super::load_organization_reward_dashboard;
    use crate::application::reporting::organization_reward_dashboard::store::OrganizationRewardDashboardStore;
    use crate::application::reporting::organization_reward_dashboard::{
        OrganizationRewardDashboardError, OrganizationRewardDashboardOutput,
        TeacherApplicationDashboardSummaryOutput,
    };

    #[test]
    fn loads_dashboard_through_store_port_with_filters() {
        let mut store = FakeOrganizationRewardDashboardStore::default();
        let from = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();

        let output = block_on(load_organization_reward_dashboard(
            &mut store,
            42,
            Some(from),
            Some(to),
        ))
        .expect("organization reward dashboard should load");

        assert_eq!(store.requested_id, Some(42));
        assert_eq!(store.requested_from, Some(from));
        assert_eq!(store.requested_to, Some(to));
        assert_eq!(output.organization_id, 42);
        assert_eq!(output.course_reward_count, 3);
    }

    #[derive(Default)]
    struct FakeOrganizationRewardDashboardStore {
        requested_id: Option<i32>,
        requested_from: Option<NaiveDate>,
        requested_to: Option<NaiveDate>,
    }

    impl OrganizationRewardDashboardStore for FakeOrganizationRewardDashboardStore {
        fn load_organization_reward_dashboard(
            &mut self,
            organization_id: i32,
            from: Option<NaiveDate>,
            to: Option<NaiveDate>,
        ) -> BoxFuture<
            '_,
            Result<OrganizationRewardDashboardOutput, OrganizationRewardDashboardError>,
        > {
            self.requested_id = Some(organization_id);
            self.requested_from = from;
            self.requested_to = to;
            ready(Ok(OrganizationRewardDashboardOutput {
                organization_id,
                organization_name: "Org".to_string(),
                sponsored_teacher_applications: TeacherApplicationDashboardSummaryOutput::default(),
                course_reward_count: 3,
                approved_reward_count: 2,
                approved_amount_total: "20".to_string(),
                courses: vec![],
                wallets: vec![],
                wallet_balance_total: "0".to_string(),
            }))
            .boxed()
        }
    }
}
