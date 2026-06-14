use futures::future::BoxFuture;

use crate::application::rewards::notify_wallet_credit::{
    RewardWalletCreditNotificationError, RewardWalletCreditNotificationOutput,
};

pub trait RewardWalletCreditNotificationUseCase: Send + Sync {
    fn notify_reward_wallet_credit(
        &self,
        candidate_id: i64,
    ) -> BoxFuture<
        '_,
        Result<RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationError>,
    >;

    fn notify_reward_wallet_credit_for_actor(
        &self,
        actor_user_id: i32,
        candidate_id: i64,
    ) -> BoxFuture<
        '_,
        Result<RewardWalletCreditNotificationOutput, RewardWalletCreditNotificationError>,
    >;
}
