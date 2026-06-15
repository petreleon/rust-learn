use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::application::rewards::record_token_confirmation::RewardTokenConfirmationCommand;
use crate::db::schema::{external_transactions, transactions, transactions_external_transactions};
use crate::domain::rewards::token::RewardTokenTransactionType;
use crate::infra::postgres::rewards::reward_token_confirmation_mappers::RewardTokenConfirmationTransactionError;
use crate::models::transaction::{
    ExternalTransaction, NewExternalTransaction, NewTransaction,
    NewTransactionExternalTransactionLink,
};

pub(super) struct RecordedExternalRewardTransaction {
    pub transaction_id: i64,
    pub external_transaction_id: i64,
    pub inserted_external_transaction: bool,
}

pub(super) async fn record_external_reward_transaction(
    conn: &mut AsyncPgConnection,
    command: &RewardTokenConfirmationCommand,
    transaction_type: RewardTokenTransactionType,
) -> Result<RecordedExternalRewardTransaction, RewardTokenConfirmationTransactionError> {
    if let Some(existing) = find_external_transaction_by_chain_tx_log(
        conn,
        command.chain_id,
        &command.transaction_hash,
        command.log_index,
    )
    .await?
    {
        let transaction_id = match find_transaction_for_external(conn, existing.id).await? {
            Some(transaction_id) => transaction_id,
            None => create_transaction_for_external(conn, existing.id, transaction_type).await?,
        };
        return Ok(RecordedExternalRewardTransaction {
            transaction_id,
            external_transaction_id: existing.id,
            inserted_external_transaction: false,
        });
    }

    let transaction_id = create_transaction(conn, transaction_type).await?;
    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: command.amount.clone(),
            blockchain_address: &command.to_address,
            chain_id: Some(command.chain_id),
            contract_address: Some(&command.contract_address),
            transaction_hash: Some(&command.transaction_hash),
            log_index: Some(command.log_index),
            event_type: Some(command.event_type.as_str()),
            from_address: command.from_address.as_deref(),
            to_address: Some(&command.to_address),
        })
        .returning(external_transactions::id)
        .get_result(conn)
        .await?;
    link_transaction_external(conn, transaction_id, external_transaction_id).await?;

    Ok(RecordedExternalRewardTransaction {
        transaction_id,
        external_transaction_id,
        inserted_external_transaction: true,
    })
}

async fn find_external_transaction_by_chain_tx_log(
    conn: &mut AsyncPgConnection,
    chain_id: i64,
    transaction_hash: &str,
    log_index: i64,
) -> QueryResult<Option<ExternalTransaction>> {
    external_transactions::table
        .filter(external_transactions::chain_id.eq(chain_id))
        .filter(external_transactions::transaction_hash.eq(transaction_hash))
        .filter(external_transactions::log_index.eq(log_index))
        .first(conn)
        .await
        .optional()
}

async fn find_transaction_for_external(
    conn: &mut AsyncPgConnection,
    external_transaction_id: i64,
) -> QueryResult<Option<i64>> {
    transactions_external_transactions::table
        .filter(
            transactions_external_transactions::external_transaction_id.eq(external_transaction_id),
        )
        .select(transactions_external_transactions::transaction_id)
        .first(conn)
        .await
        .optional()
}

async fn create_transaction_for_external(
    conn: &mut AsyncPgConnection,
    external_transaction_id: i64,
    transaction_type: RewardTokenTransactionType,
) -> QueryResult<i64> {
    let transaction_id = create_transaction(conn, transaction_type).await?;
    link_transaction_external(conn, transaction_id, external_transaction_id).await?;
    Ok(transaction_id)
}

async fn create_transaction(
    conn: &mut AsyncPgConnection,
    transaction_type: RewardTokenTransactionType,
) -> QueryResult<i64> {
    diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: transaction_type.as_str(),
        })
        .returning(transactions::id)
        .get_result(conn)
        .await
}

async fn link_transaction_external(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    external_transaction_id: i64,
) -> QueryResult<usize> {
    diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .execute(conn)
        .await
}
