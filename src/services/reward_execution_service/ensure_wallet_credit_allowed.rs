async fn ensure_wallet_credit_allowed(
    conn: &mut AsyncPgConnection,
    candidate: &RewardCandidate,
) -> Result<(), RewardExecutionError> {
    if candidate.status == REWARD_STATUS_TOKEN_CONFIRMED {
        return Ok(());
    }

    if candidate.status == REWARD_STATUS_AMOUNT_APPROVED {
        let plan = plan_reward_payout(conn, candidate.id).await?;
        if plan.payout_method == REWARD_PAYOUT_METHOD_OFF_CHAIN {
            return Ok(());
        }
    }

    Err(RewardExecutionError::InvalidStatus(
        "wallet credit requires token confirmation unless the reward policy is off-chain"
            .to_string(),
    ))
}

async fn credit_wallet_balance(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> QueryResult<Wallet> {
    diesel::update(wallets::table.find(wallet_id))
        .set(wallets::value.eq(wallets::value + amount))
        .get_result(conn)
        .await
}

async fn create_internal_transaction(
    conn: &mut AsyncPgConnection,
    wallet_id: i32,
    amount: BigDecimal,
) -> QueryResult<i64> {
    diesel::insert_into(internal_transactions::table)
        .values(NewInternalTransaction { wallet_id, amount })
        .returning(internal_transactions::id)
        .get_result(conn)
        .await
}

async fn create_wallet_credit_transaction(
    conn: &mut AsyncPgConnection,
    internal_transaction_id: i64,
) -> QueryResult<i64> {
    let transaction_id = diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: REWARD_TRANSACTION_TYPE_WALLET_CREDIT,
        })
        .returning(transactions::id)
        .get_result(conn)
        .await?;

    diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .execute(conn)
        .await?;

    Ok(transaction_id)
}

async fn ensure_internal_transaction_link(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    internal_transaction_id: i64,
) -> QueryResult<bool> {
    let inserted = diesel::insert_into(transactions_internal_transactions::table)
        .values(NewTransactionInternalTransactionLink {
            transaction_id,
            internal_transaction_id,
        })
        .on_conflict((
            transactions_internal_transactions::transaction_id,
            transactions_internal_transactions::internal_transaction_id,
        ))
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(inserted > 0)
}

async fn mark_candidate_wallet_credited(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> QueryResult<RewardCandidate> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(REWARD_STATUS_WALLET_CREDITED),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result(conn)
        .await
}

async fn mark_candidate_notified(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> QueryResult<RewardCandidate> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(REWARD_STATUS_NOTIFIED),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result(conn)
        .await
}
