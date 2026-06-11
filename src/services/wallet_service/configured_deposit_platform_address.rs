async fn configured_deposit_platform_address(
    conn: &mut AsyncPgConnection,
    gas_payer: WalletTokenGasPayer,
) -> Result<String, WalletTokenTransferError> {
    let configured = match gas_payer {
        WalletTokenGasPayer::User => env::var("WALLET_DEPOSIT_TREASURY_ADDRESS")
            .ok()
            .or_else(|| env::var("PLATFORM_TREASURY").ok()),
        WalletTokenGasPayer::Platform => get_persistent_state(conn, "platform_importer_address")
            .await?
            .or_else(|| env::var("WALLET_DEPOSIT_IMPORTER_ADDRESS").ok())
            .or_else(|| env::var("PLATFORM_IMPORTER_ADDRESS").ok()),
    };

    configured
        .map(|value| normalize_address(&value))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            WalletTokenTransferError::InvalidInput(
                "platform deposit receiver is not configured".to_string(),
            )
        })
}

async fn create_wallet_token_transaction(
    conn: &mut AsyncPgConnection,
    operation: WalletTokenOperation,
) -> QueryResult<i64> {
    diesel::insert_into(transactions::table)
        .values(NewTransaction {
            type_: operation.transaction_type(),
        })
        .returning(transactions::id)
        .get_result(conn)
        .await
}

async fn create_wallet_external_transaction(
    conn: &mut AsyncPgConnection,
    operation: WalletTokenOperation,
    transaction_id: i64,
    request: &WalletTokenTransferRequest,
) -> QueryResult<i64> {
    let ethereum_address = request.ethereum_address.trim();
    let platform_address = request.platform_address.as_deref().map(str::trim);
    let (from_address, to_address) = match operation {
        WalletTokenOperation::Deposit => (Some(ethereum_address), platform_address),
        WalletTokenOperation::Retire => (platform_address, Some(ethereum_address)),
    };

    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: request.amount.clone(),
            blockchain_address: ethereum_address,
            chain_id: request.chain_id,
            contract_address: request.contract_address.as_deref().map(str::trim),
            transaction_hash: request.transaction_hash.as_deref().map(str::trim),
            log_index: request.log_index,
            event_type: Some(operation.external_event_type()),
            from_address,
            to_address,
        })
        .returning(external_transactions::id)
        .get_result(conn)
        .await?;

    diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .execute(conn)
        .await?;

    Ok(external_transaction_id)
}

async fn create_observed_deposit_external_transaction(
    conn: &mut AsyncPgConnection,
    transaction_id: i64,
    event: &ObservedWalletDepositEvent,
) -> QueryResult<i64> {
    let from_address = normalize_address(&event.from_address);
    let to_address = normalize_address(&event.to_address);
    let contract_address = normalize_address(&event.contract_address);
    let transaction_hash = event.transaction_hash.trim().to_ascii_lowercase();

    let external_transaction_id = diesel::insert_into(external_transactions::table)
        .values(NewExternalTransaction {
            amount: event.amount.clone(),
            blockchain_address: &from_address,
            chain_id: Some(event.chain_id),
            contract_address: Some(&contract_address),
            transaction_hash: Some(&transaction_hash),
            log_index: Some(event.log_index),
            event_type: Some(event.event_type.as_str()),
            from_address: Some(&from_address),
            to_address: Some(&to_address),
        })
        .returning(external_transactions::id)
        .get_result(conn)
        .await?;

    diesel::insert_into(transactions_external_transactions::table)
        .values(NewTransactionExternalTransactionLink {
            transaction_id,
            external_transaction_id,
        })
        .execute(conn)
        .await?;

    Ok(external_transaction_id)
}

async fn find_deposit_intent_by_chain_event(
    conn: &mut AsyncPgConnection,
    event: &ObservedWalletDepositEvent,
    for_update: bool,
) -> QueryResult<Option<WalletTokenDepositIntent>> {
    let transaction_hash = event.transaction_hash.trim().to_ascii_lowercase();
    let query = wallet_token_deposit_intents::table
        .filter(wallet_token_deposit_intents::chain_id.eq(Some(event.chain_id)))
        .filter(wallet_token_deposit_intents::transaction_hash.eq(Some(transaction_hash)))
        .filter(wallet_token_deposit_intents::log_index.eq(Some(event.log_index)));

    if for_update {
        query.for_update().first(conn).await.optional()
    } else {
        query.first(conn).await.optional()
    }
}
