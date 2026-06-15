use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::plan_payout::{
    RewardPayoutCandidate, RewardPayoutPlanError, RewardPayoutPlanStore, RewardPayoutPolicy,
};
use crate::infra::postgres::operations::persistent_state::get_persistent_state;
use crate::infra::postgres::rewards::reward_authorization_access;
use crate::infra::postgres::rewards::reward_candidate_records::find_candidate;
use crate::infra::postgres::rewards::reward_payout_plan_mappers::{
    map_reward_payout_candidate, map_reward_payout_plan_error,
};
use crate::infra::postgres::rewards::reward_payout_plan_policy_lookup::active_reward_payout_policy;

pub struct PostgresRewardPayoutPlanStore<'conn> {
    conn: &'conn mut AsyncPgConnection,
}

impl<'conn> PostgresRewardPayoutPlanStore<'conn> {
    pub fn new(conn: &'conn mut AsyncPgConnection) -> Self {
        Self { conn }
    }
}

impl RewardPayoutPlanStore for PostgresRewardPayoutPlanStore<'_> {
    fn can_execute_reward_payout(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardPayoutPlanError>> {
        async move {
            reward_authorization_access::can_execute_reward_payout(self.conn, actor_user_id)
                .await
                .map_err(|error| RewardPayoutPlanError::Database(error.to_string()))
        }
        .boxed()
    }

    fn load_candidate(
        &mut self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardPayoutCandidate, RewardPayoutPlanError>> {
        async move {
            find_candidate(self.conn, candidate_id)
                .await
                .map_err(map_reward_payout_plan_error)
                .and_then(map_reward_payout_candidate)
        }
        .boxed()
    }

    fn active_policy_for_candidate(
        &mut self,
        course_id: i32,
        event_type: String,
    ) -> BoxFuture<'_, Result<Option<RewardPayoutPolicy>, RewardPayoutPlanError>> {
        async move { active_reward_payout_policy(self.conn, course_id, &event_type).await }.boxed()
    }

    fn has_presigner_contract(&mut self) -> BoxFuture<'_, Result<bool, RewardPayoutPlanError>> {
        async move {
            get_persistent_state(self.conn, "learn_token_presigner_address")
                .await
                .map(|address| {
                    address
                        .map(|value| !value.trim().is_empty())
                        .unwrap_or(false)
                })
                .map_err(map_reward_payout_plan_error)
        }
        .boxed()
    }
}
