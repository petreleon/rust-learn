use bigdecimal::BigDecimal;
use futures::future::{ready, BoxFuture, FutureExt};

use crate::application::rewards::plan_payout::{
    plan_reward_payout, plan_reward_payout_for_actor, RewardPayoutCandidate, RewardPayoutPlanError,
    RewardPayoutPlanStore, RewardPayoutPolicy,
};
use crate::domain::rewards::candidate::status::RewardCandidateStatus;
use crate::domain::rewards::payout::{
    REWARD_PAYOUT_METHOD_MINT, REWARD_PAYOUT_METHOD_OFF_CHAIN,
    REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER,
};
use crate::domain::rewards::policy::{
    REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN, REWARD_PAYMENT_TREASURY_TRANSFER,
};

struct FakeStore {
    can_execute: bool,
    candidate: RewardPayoutCandidate,
    policy: Option<RewardPayoutPolicy>,
    has_presigner: bool,
    loaded_candidate: bool,
}

impl RewardPayoutPlanStore for FakeStore {
    fn can_execute_reward_payout(
        &mut self,
        _actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardPayoutPlanError>> {
        ready(Ok(self.can_execute)).boxed()
    }

    fn load_candidate(
        &mut self,
        _candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardPayoutCandidate, RewardPayoutPlanError>> {
        self.loaded_candidate = true;
        ready(Ok(self.candidate.clone())).boxed()
    }

    fn active_policy_for_candidate(
        &mut self,
        _course_id: i32,
        _event_type: String,
    ) -> BoxFuture<'_, Result<Option<RewardPayoutPolicy>, RewardPayoutPlanError>> {
        ready(Ok(self.policy.clone())).boxed()
    }

    fn has_presigner_contract(&mut self) -> BoxFuture<'_, Result<bool, RewardPayoutPlanError>> {
        ready(Ok(self.has_presigner)).boxed()
    }
}

#[tokio::test]
async fn plans_treasury_payout_with_presigner() {
    let mut store = fake(REWARD_PAYMENT_TREASURY_TRANSFER);
    store.has_presigner = true;

    let plan = plan_reward_payout(&mut store, 1).await.unwrap();

    assert_eq!(plan.payout_method, REWARD_PAYOUT_METHOD_PRESIGNER_TRANSFER);
    assert!(plan.requires_token_confirmation);
    assert_eq!(plan.amount, BigDecimal::from(10));
}

#[tokio::test]
async fn off_chain_payout_does_not_require_token_confirmation() {
    let mut store = fake(REWARD_PAYMENT_OFF_CHAIN);

    let plan = plan_reward_payout(&mut store, 1).await.unwrap();

    assert_eq!(plan.payout_method, REWARD_PAYOUT_METHOD_OFF_CHAIN);
    assert!(!plan.requires_token_confirmation);
}

#[tokio::test]
async fn mint_payout_requires_token_confirmation() {
    let mut store = fake(REWARD_PAYMENT_MINT);

    let plan = plan_reward_payout(&mut store, 1).await.unwrap();

    assert_eq!(plan.payout_method, REWARD_PAYOUT_METHOD_MINT);
    assert!(plan.requires_token_confirmation);
}

#[tokio::test]
async fn actor_permission_denial_happens_before_candidate_load() {
    let mut store = fake(REWARD_PAYMENT_MINT);
    store.can_execute = false;

    let error = plan_reward_payout_for_actor(&mut store, 7, 1)
        .await
        .unwrap_err();

    assert_eq!(
        error,
        RewardPayoutPlanError::PermissionDenied("EXECUTE_REWARD_PAYOUT".to_string())
    );
    assert!(!store.loaded_candidate);
}

#[tokio::test]
async fn missing_policy_is_no_active_policy() {
    let mut store = fake(REWARD_PAYMENT_MINT);
    store.policy = None;

    assert_eq!(
        plan_reward_payout(&mut store, 1).await.unwrap_err(),
        RewardPayoutPlanError::NoActivePolicy
    );
}

fn fake(payment_strategy: &str) -> FakeStore {
    FakeStore {
        can_execute: true,
        candidate: RewardPayoutCandidate {
            id: 1,
            course_id: 2,
            event_type: "course_completion".to_string(),
            status: RewardCandidateStatus::AmountApproved,
            approved_amount: Some(BigDecimal::from(10)),
        },
        policy: Some(RewardPayoutPolicy {
            id: 3,
            payment_strategy: payment_strategy.to_string(),
        }),
        has_presigner: false,
        loaded_candidate: false,
    }
}
