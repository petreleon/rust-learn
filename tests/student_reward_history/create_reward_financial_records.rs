use crate::create_reward_candidate::*;
use crate::support::*;

pub(crate) async fn create_reward_financial_records(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
    wallet_id: i32,
    amount: BigDecimal,
) -> RewardFinancialFixture {
    let payout_transaction_id = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_payout"))
        .returning(transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create payout transaction");
    let transaction_hash = unique_string("student_reward_history_tx");
    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values((
            external_transactions::amount.eq(amount.clone()),
            external_transactions::blockchain_address.eq("0xstudenthistory"),
            external_transactions::chain_id.eq(Some(31337_i64)),
            external_transactions::contract_address.eq(Some("0xcontract")),
            external_transactions::transaction_hash.eq(Some(transaction_hash.clone())),
            external_transactions::log_index.eq(Some(0_i64)),
            external_transactions::event_type.eq(Some("Transfer")),
            external_transactions::from_address.eq(Some("0xtreasury")),
            external_transactions::to_address.eq(Some("0xstudenthistory")),
        ))
        .returning(external_transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create external transaction");
    diesel::insert_into(transactions_external_transactions::table)
        .values((
            transactions_external_transactions::transaction_id.eq(payout_transaction_id),
            transactions_external_transactions::external_transaction_id.eq(external_transaction_id),
        ))
        .execute(conn)
        .await
        .expect("failed to link payout transaction");
    diesel::insert_into(reward_payout_records::table)
        .values((
            reward_payout_records::reward_candidate_id.eq(candidate_id),
            reward_payout_records::transaction_id.eq(payout_transaction_id),
            reward_payout_records::external_transaction_id.eq(external_transaction_id),
        ))
        .execute(conn)
        .await
        .expect("failed to create payout record");

    let wallet_transaction_id = diesel::insert_into(transactions::table)
        .values(transactions::type_.eq("reward_wallet_credit"))
        .returning(transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create wallet credit transaction");
    let internal_transaction_id = diesel::insert_into(internal_transactions::table)
        .values((
            internal_transactions::wallet_id.eq(wallet_id),
            internal_transactions::amount.eq(amount),
        ))
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
        .expect("failed to create internal transaction");
    diesel::insert_into(transactions_internal_transactions::table)
        .values((
            transactions_internal_transactions::transaction_id.eq(wallet_transaction_id),
            transactions_internal_transactions::internal_transaction_id.eq(internal_transaction_id),
        ))
        .execute(conn)
        .await
        .expect("failed to link wallet credit transaction");
    let wallet_credit_record_id = diesel::insert_into(reward_wallet_credit_records::table)
        .values((
            reward_wallet_credit_records::reward_candidate_id.eq(candidate_id),
            reward_wallet_credit_records::wallet_id.eq(wallet_id),
            reward_wallet_credit_records::transaction_id.eq(wallet_transaction_id),
            reward_wallet_credit_records::internal_transaction_id.eq(internal_transaction_id),
        ))
        .returning(reward_wallet_credit_records::id)
        .get_result(conn)
        .await
        .expect("failed to create wallet credit record");

    RewardFinancialFixture {
        payout_transaction_id,
        external_transaction_id,
        wallet_transaction_id,
        internal_transaction_id,
        wallet_credit_record_id,
        transaction_hash,
    }
}
