use futures::future::BoxFuture;

use crate::application::rewards::credit_wallet::{
    RewardWalletCreditError, RewardWalletCreditOutput,
};

pub trait RewardWalletCreditUseCase: Send + Sync {
    fn credit_reward_wallet(
        &self,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardWalletCreditOutput, RewardWalletCreditError>>;

    fn credit_reward_wallet_for_actor(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
    ) -> BoxFuture<'_, Result<RewardWalletCreditOutput, RewardWalletCreditError>>;
}
