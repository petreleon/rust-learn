use diesel_async::AsyncPgConnection;

use crate::application::rewards::credit_wallet::RewardWalletCreditError;
use crate::domain::rewards::policy::{
    REWARD_PAYMENT_MINT, REWARD_PAYMENT_OFF_CHAIN, REWARD_PAYMENT_TREASURY_TRANSFER,
};
use crate::infra::postgres::rewards::reward_payout_plan_policy_lookup::active_reward_payout_policy;
use crate::infra::postgres::rewards::reward_wallet_credit_mappers::map_payout_plan_error;

pub(super) async fn reward_policy_is_off_chain(
    conn: &mut AsyncPgConnection,
    course_id: i32,
    event_type: &str,
) -> Result<bool, RewardWalletCreditError> {
    let policy = active_reward_payout_policy(conn, course_id, event_type)
        .await
        .map_err(map_payout_plan_error)?
        .ok_or(RewardWalletCreditError::NoActivePolicy)?;
    match policy.payment_strategy.as_str() {
        REWARD_PAYMENT_OFF_CHAIN => Ok(true),
        REWARD_PAYMENT_MINT | REWARD_PAYMENT_TREASURY_TRANSFER => Ok(false),
        _ => Err(RewardWalletCreditError::InvalidInput(
            "unsupported reward payment strategy".to_string(),
        )),
    }
}
