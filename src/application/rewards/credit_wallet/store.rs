use futures::future::BoxFuture;

use crate::application::rewards::credit_wallet::{
    RewardWalletCreditError, RewardWalletCreditOutput,
};

pub struct RewardWalletCredit {
    pub candidate_id: i64,
    pub actor_user_id: Option<i32>,
    pub allow_reconciliation_credit: bool,
}

pub trait RewardWalletCreditStore {
    fn can_execute_reward_payout(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardWalletCreditError>>;

    fn credit_reward_wallet(
        &mut self,
        credit: RewardWalletCredit,
    ) -> BoxFuture<'_, Result<RewardWalletCreditOutput, RewardWalletCreditError>>;
}
