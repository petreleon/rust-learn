async fn next_block_to_scan(
    conn: &mut diesel_async::AsyncPgConnection,
    confirmed_to_block: u64,
    config: &WalletDepositIndexerConfig,
) -> Result<u64, String> {
    if let Some(value) = get_persistent_state(conn, NEXT_BLOCK_STATE_KEY)
        .await
        .map_err(|error| format!("failed to read next indexer block: {error}"))?
    {
        return value
            .parse::<u64>()
            .map_err(|error| format!("invalid next indexer block '{value}': {error}"));
    }

    if let Ok(value) = env::var("WALLET_DEPOSIT_INDEXER_START_BLOCK") {
        return value
            .parse::<u64>()
            .map_err(|error| format!("invalid WALLET_DEPOSIT_INDEXER_START_BLOCK: {error}"));
    }

    Ok(confirmed_to_block.saturating_sub(config.lookback_blocks))
}

async fn fetch_transfer_deposit_events<M: Middleware>(
    provider: &M,
    token_address: Address,
    treasury_addresses: HashSet<Address>,
    chain_id: i64,
    from_block: u64,
    to_block: u64,
    token_decimals: u32,
) -> Result<Vec<ObservedWalletDepositEvent>, String> {
    if treasury_addresses.is_empty() {
        return Ok(Vec::new());
    }

    let filter = Filter::new()
        .address(token_address)
        .topic0(event_signature("Transfer(address,address,uint256)"))
        .from_block(BlockNumber::Number(U64::from(from_block)))
        .to_block(BlockNumber::Number(U64::from(to_block)));
    let logs = provider
        .get_logs(&filter)
        .await
        .map_err(|error| format!("failed to fetch Transfer logs: {error}"))?;

    logs.into_iter()
        .filter_map(|log| parse_transfer_log(log, &treasury_addresses, chain_id, token_decimals))
        .collect::<Result<Vec<_>, _>>()
}

async fn fetch_imported_deposit_events<M: Middleware>(
    provider: &M,
    token_address: Address,
    importer_address: Address,
    chain_id: i64,
    from_block: u64,
    to_block: u64,
    token_decimals: u32,
) -> Result<Vec<ObservedWalletDepositEvent>, String> {
    let filter = Filter::new()
        .address(importer_address)
        .topic0(event_signature("Imported(address,uint256,address)"))
        .from_block(BlockNumber::Number(U64::from(from_block)))
        .to_block(BlockNumber::Number(U64::from(to_block)));
    let logs = provider
        .get_logs(&filter)
        .await
        .map_err(|error| format!("failed to fetch Imported logs: {error}"))?;

    logs.into_iter()
        .filter_map(|log| {
            parse_imported_log(
                log,
                token_address,
                importer_address,
                chain_id,
                token_decimals,
            )
        })
        .collect::<Result<Vec<_>, _>>()
}

fn parse_transfer_log(
    log: Log,
    treasury_addresses: &HashSet<Address>,
    chain_id: i64,
    token_decimals: u32,
) -> Option<Result<ObservedWalletDepositEvent, String>> {
    if log.topics.len() < 3 {
        return None;
    }
    let from_address = address_from_topic(log.topics[1])?;
    let to_address = address_from_topic(log.topics[2])?;
    if !treasury_addresses.contains(&to_address) {
        return None;
    }

    let contract_address = log.address;
    Some(build_observed_event(
        log,
        contract_address,
        WALLET_DEPOSIT_EVENT_TRANSFER,
        from_address,
        to_address,
        chain_id,
        token_decimals,
    ))
}

fn parse_imported_log(
    log: Log,
    token_address: Address,
    importer_address: Address,
    chain_id: i64,
    token_decimals: u32,
) -> Option<Result<ObservedWalletDepositEvent, String>> {
    if log.topics.len() < 3 {
        return None;
    }
    let user_address = address_from_topic(log.topics[1])?;
    let event_token_address = address_from_topic(log.topics[2])?;
    if event_token_address != token_address {
        return None;
    }

    Some(build_observed_event(
        log,
        token_address,
        WALLET_DEPOSIT_EVENT_IMPORT,
        user_address,
        importer_address,
        chain_id,
        token_decimals,
    ))
}
