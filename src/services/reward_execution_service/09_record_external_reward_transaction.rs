async fn record_external_reward_transaction(
    conn: &mut AsyncPgConnection,
    request: &RewardTokenConfirmationRequest,
) -> Result<(i64, i64, bool), RewardExecutionError> {
    if let Some(existing) = find_external_transaction_by_chain_tx_log(
        conn,
        request.chain_id,
        &request.transaction_hash,
        request.log_index,
    )
    .await?
    {
        let transaction_id = match find_transaction_for_external(conn, existing.id).await? {
            Some(transaction_id) => transaction_id,
            None => create_transaction_for_external(conn, existing.id, &request.event_type).await?,
        };
        return Ok((transaction_id, existing.id, false));
    }

    let transaction_id =
        create_transaction(conn, transaction_type_for_event(&request.event_type)?).await?;
    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: request.amount.clone(),
            blockchain_address: &request.to_address,
            chain_id: Some(request.chain_id),
            contract_address: Some(&request.contract_address),
            transaction_hash: Some(&request.transaction_hash),
            log_index: Some(request.log_index),
            event_type: Some(request.event_type.as_str()),
            from_address: request.from_address.as_deref(),
            to_address: Some(&request.to_address),
        })
        .returning(external_transactions::id)
        .get_result(conn)
        .await?;
    link_transaction_external(conn, transaction_id, external_transaction_id).await?;

    Ok((transaction_id, external_transaction_id, true))
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
    event_type: &str,
) -> Result<i64, RewardExecutionError> {
    let transaction_id = create_transaction(conn, transaction_type_for_event(event_type)?).await?;
    link_transaction_external(conn, transaction_id, external_transaction_id).await?;
    Ok(transaction_id)
}

async fn create_transaction(
    conn: &mut AsyncPgConnection,
    transaction_type: &str,
) -> QueryResult<i64> {
    diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: transaction_type,
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

async fn ensure_external_transaction_link(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    external_transaction_id: i64,
) -> QueryResult<bool> {
    let inserted = diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .on_conflict((
            transactions_external_transactions::transaction_id,
            transactions_external_transactions::external_transaction_id,
        ))
        .do_nothing()
        .execute(conn)
        .await?;
    Ok(inserted > 0)
}

async fn mark_candidate_token_confirmed(
    conn: &mut AsyncPgConnection,
    candidate_id: i64,
) -> QueryResult<RewardCandidate> {
    diesel::update(reward_candidates::table.find(candidate_id))
        .set((
            reward_candidates::status.eq(REWARD_STATUS_TOKEN_CONFIRMED),
            reward_candidates::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result(conn)
        .await
}

fn transaction_type_for_event(event_type: &str) -> Result<&'static str, RewardExecutionError> {
    match event_type {
        "mint" => Ok("token_mint"),
        "transfer" => Ok("token_transfer"),
        "import" => Ok("token_import"),
        _ => Err(RewardExecutionError::InvalidInput(
            "unsupported token event type".to_string(),
        )),
    }
}
