use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::list_reward_history::{
    StudentRewardHistoryError, StudentRewardTokenTransaction, StudentRewardWalletCredit,
};
use crate::infra::postgres::rewards::reward_history_mappers::{
    map_reward_history_error, wallet_credit, WalletCreditRow,
};
use crate::infra::postgres::rewards::reward_history_token_transactions::{
    token_transaction, TokenTransactionRow,
};
use crate::infra::postgres::schema::{
    external_transactions, internal_transactions, reward_payout_records,
    reward_wallet_credit_records,
};

pub(super) async fn load_wallet_credit(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> Result<Option<StudentRewardWalletCredit>, StudentRewardHistoryError> {
    let row = reward_wallet_credit_records::table
        .inner_join(internal_transactions::table.on(
            reward_wallet_credit_records::internal_transaction_id.eq(internal_transactions::id),
        ))
        .filter(reward_wallet_credit_records::reward_candidate_id.eq(reward_candidate_id))
        .select((
            reward_wallet_credit_records::id,
            reward_wallet_credit_records::wallet_id,
            reward_wallet_credit_records::transaction_id,
            reward_wallet_credit_records::internal_transaction_id,
            reward_wallet_credit_records::notification_id,
            reward_wallet_credit_records::notified_at,
            reward_wallet_credit_records::created_at,
            internal_transactions::amount,
        ))
        .first::<WalletCreditRow>(conn)
        .await
        .optional()
        .map_err(map_reward_history_error)?;

    Ok(row.map(wallet_credit))
}

pub(super) async fn load_token_transaction(
    conn: &mut AsyncPgConnection,
    reward_candidate_id: i64,
) -> Result<Option<StudentRewardTokenTransaction>, StudentRewardHistoryError> {
    let row = reward_payout_records::table
        .inner_join(
            external_transactions::table
                .on(reward_payout_records::external_transaction_id.eq(external_transactions::id)),
        )
        .filter(reward_payout_records::reward_candidate_id.eq(reward_candidate_id))
        .select((
            reward_payout_records::id,
            reward_payout_records::transaction_id,
            reward_payout_records::external_transaction_id,
            reward_payout_records::created_at,
            external_transactions::amount,
            external_transactions::blockchain_address,
            external_transactions::chain_id,
            external_transactions::contract_address,
            external_transactions::transaction_hash,
            external_transactions::log_index,
            external_transactions::event_type,
            external_transactions::from_address,
            external_transactions::to_address,
        ))
        .first::<TokenTransactionRow>(conn)
        .await
        .optional()
        .map_err(map_reward_history_error)?;

    row.map(token_transaction).transpose()
}
