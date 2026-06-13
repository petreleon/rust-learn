use futures::future::BoxFuture;

use crate::application::rewards::notify_wallet_credit::{
    RewardWalletCreditNotificationError, RewardWalletCreditNotificationOutput,
};

pub struct RewardWalletCreditNotification {
    pub candidate_id: i64,
    pub actor_user_id: Option<i32>,
    pub allow_reconciliation_repair: bool,
}

pub trait RewardWalletCreditNotificationStore {
    fn can_execute_reward_payout(
        &mut self,
        actor_user_id: i32,
    ) -> BoxFuture<'_, Result<bool, RewardWalletCreditNotificationError>>;

    fn notify_reward_wallet_credit(
        &mut self,
        notification: RewardWalletCreditNotification,
    ) -> BoxFuture<
        '_,
        Result<RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationError>,
    >;
}
