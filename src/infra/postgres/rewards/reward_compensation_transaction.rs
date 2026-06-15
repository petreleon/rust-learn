use diesel::QueryDsl;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::record_compensation::{
    RewardCompensation, RewardCompensationError, RewardCompensationOutput,
};
use crate::infra::postgres::models::reward_compensation_record::NewRewardCompensationRecord;
use crate::infra::postgres::models::wallet::Wallet;
use crate::infra::postgres::rewards::reward_candidate_records::find_candidate;
use crate::infra::postgres::rewards::reward_compensation_mappers::map_reward_compensation_error;
use crate::infra::postgres::rewards::reward_compensation_records::{
    create_reward_compensation_record, find_reward_compensation_record_by_idempotency_key,
};
use crate::infra::postgres::rewards::reward_compensation_transactions::{
    create_compensation_transaction, create_internal_transaction,
};
use crate::infra::postgres::rewards::reward_compensation_wallets::{
    apply_wallet_adjustment, link_user_wallet,
};
use crate::infra::postgres::schema::wallets;

pub(super) async fn record_reward_compensation(
    conn: &mut AsyncPgConnection,
    compensation: RewardCompensation,
) -> Result<RewardCompensationOutput, RewardCompensationError> {
    let command = compensation.command;
    if let Some(existing) =
        find_reward_compensation_record_by_idempotency_key(conn, &command.idempotency_key)
            .await
            .map_err(map_reward_compensation_error)?
    {
        let wallet = wallets::table
            .find(existing.wallet_id)
            .first::<Wallet>(conn)
            .await
            .map_err(map_reward_compensation_error)?;
        return Ok(RewardCompensationOutput {
            record: existing.into(),
            wallet: wallet.into(),
            created: false,
        });
    }

    let candidate = find_candidate(conn, command.reward_candidate_id)
        .await
        .map_err(map_reward_compensation_error)?;
    let linked_wallet = link_user_wallet(conn, candidate.student_user_id).await?;
    let wallet = apply_wallet_adjustment(conn, linked_wallet.id, command.amount.clone()).await?;
    let internal_transaction_id =
        create_internal_transaction(conn, wallet.id, command.amount.clone()).await?;
    let transaction_id = create_compensation_transaction(conn, internal_transaction_id).await?;
    let record = create_reward_compensation_record(
        conn,
        NewRewardCompensationRecord {
            reward_candidate_id: candidate.id,
            wallet_id: wallet.id,
            transaction_id,
            internal_transaction_id,
            amount: command.amount,
            reason: command.reason.trim().to_string(),
            idempotency_key: command.idempotency_key,
            created_by_user_id: compensation.actor_user_id,
        },
    )
    .await
    .map_err(map_reward_compensation_error)?;

    Ok(RewardCompensationOutput {
        record: record.into(),
        wallet: wallet.into(),
        created: true,
    })
}
