use crate::application::rewards::record_compensation::{
    RewardCompensationError, RewardCompensationRecordOutput, RewardCompensationWalletOutput,
};
use crate::models::reward_compensation_record::RewardCompensationRecord;
use crate::models::wallet::Wallet;

pub(super) fn map_reward_compensation_error(
    error: diesel::result::Error,
) -> RewardCompensationError {
    match error {
        diesel::result::Error::NotFound => RewardCompensationError::NotFound,
        other => RewardCompensationError::Database(other.to_string()),
    }
}

impl From<RewardCompensationRecord> for RewardCompensationRecordOutput {
    fn from(record: RewardCompensationRecord) -> Self {
        Self {
            id: record.id,
            reward_candidate_id: record.reward_candidate_id,
            wallet_id: record.wallet_id,
            transaction_id: record.transaction_id,
            internal_transaction_id: record.internal_transaction_id,
            amount: record.amount,
            reason: record.reason,
            idempotency_key: record.idempotency_key,
            created_by_user_id: record.created_by_user_id,
            created_at: record.created_at,
        }
    }
}

impl From<Wallet> for RewardCompensationWalletOutput {
    fn from(wallet: Wallet) -> Self {
        Self {
            id: wallet.id,
            user_id: wallet.user_id,
            organization_id: wallet.organization_id,
            value: wallet.value,
        }
    }
}
