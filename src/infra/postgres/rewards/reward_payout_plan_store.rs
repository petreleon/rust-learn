use diesel_async::AsyncPgConnection;
use futures::future::{BoxFuture, FutureExt};

use crate::application::rewards::plan_payout::{
    RewardPayoutCandidate, RewardPayoutPlanError, RewardPayoutPlanStore, RewardPayoutPolicy,
};
use crate::config::constants::permissions::Permissions;
use crate::infra::postgres::rewards::reward_payout_plan_mappers::map_reward_payout_plan_error;
use crate::infra::postgres::rewards::reward_payout_plan_policy_lookup::active_reward_payout_policy;
use crate::repositories::{
    persistent_state_repository, platform_repository, reward_candidate_repository,
};

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
            platform_repository::user_permission_platform_request(
                self.conn,
                actor_user_id,
                &Permissions::EXECUTE_REWARD_PAYOUT.to_string(),
            )
            .await
            .map_err(map_reward_payout_plan_error)
        }
        .boxed()
    }

    fn load_candidate(
        &mut self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardPayoutCandidate, RewardPayoutPlanError>> {
        async move {
            reward_candidate_repository::find_candidate(self.conn, candidate_id)
                .await
                .map(RewardPayoutCandidate::from)
                .map_err(map_reward_payout_plan_error)
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
            persistent_state_repository::get_persistent_state(
                self.conn,
                "learn_token_presigner_address",
            )
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
