use crate::application::wallet::index_deposit::index_observed_deposit;
use crate::db::DbPool;
use crate::infra::postgres::wallet::wallet_deposit_index_store::PostgresWalletDepositIndexStore;
use crate::repositories::persistent_state_repository::{
    get_persistent_state, set_persistent_state,
};
use crate::services::wallet_deposit_indexer_service::ethereum_log_helpers::parse_address;
use crate::services::wallet_deposit_indexer_service::next_block_to_scan::{
    fetch_imported_deposit_events, fetch_transfer_deposit_events, next_block_to_scan,
};
use crate::services::wallet_deposit_indexer_service::support::{
    WalletDepositIndexerConfig, NEXT_BLOCK_STATE_KEY,
};
use ethers::providers::Middleware;
use ethers::types::Address;
use std::collections::HashSet;
use std::env;

pub(super) async fn run_wallet_deposit_indexer_once(
    pool: &DbPool,
    config: &WalletDepositIndexerConfig,
) -> Result<usize, String> {
    let provider = crate::utils::eth::provider::try_get_provider()?;
    let latest = provider
        .get_block_number()
        .await
        .map_err(|error| format!("failed to fetch latest block: {error}"))?
        .as_u64();
    let to_block = latest.saturating_sub(config.confirmations);

    let mut conn = pool
        .get()
        .await
        .map_err(|error| format!("failed to get DB connection: {error}"))?;
    let token_address = configured_learn_token_address(&mut conn).await?;
    let importer_address = configured_importer_address(&mut conn).await?;
    let treasury_addresses = configured_treasury_addresses();
    let next_block = next_block_to_scan(&mut conn, to_block, config).await?;
    drop(conn);

    if next_block > to_block {
        return Ok(0);
    }

    let scan_to_block = next_block
        .saturating_add(config.batch_blocks)
        .saturating_sub(1)
        .min(to_block);
    let chain_id = provider
        .get_chainid()
        .await
        .map_err(|error| format!("failed to fetch chain id: {error}"))?
        .as_u64() as i64;

    let transfer_events = fetch_transfer_deposit_events(
        &provider,
        token_address,
        treasury_addresses,
        chain_id,
        next_block,
        scan_to_block,
        config.token_decimals,
    )
    .await?;

    let imported_events = if let Some(importer_address) = importer_address {
        fetch_imported_deposit_events(
            &provider,
            token_address,
            importer_address,
            chain_id,
            next_block,
            scan_to_block,
            config.token_decimals,
        )
        .await?
    } else {
        Vec::new()
    };

    let imported_transaction_hashes = imported_events
        .iter()
        .map(|event| event.transaction_hash.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    let mut observed_events = transfer_events
        .into_iter()
        .filter(|event| {
            !imported_transaction_hashes.contains(&event.transaction_hash.to_ascii_lowercase())
        })
        .collect::<Vec<_>>();
    observed_events.extend(imported_events);

    let mut credited_count = 0;
    for event in observed_events {
        let mut conn = pool
            .get()
            .await
            .map_err(|error| format!("failed to get DB connection: {error}"))?;
        let mut store = PostgresWalletDepositIndexStore::new(&mut conn);
        let result = index_observed_deposit(&mut store, event)
            .await
            .map_err(|error| format!("failed to credit observed wallet deposit: {error:?}"))?;
        if result.credited {
            credited_count += 1;
        }
    }

    let mut conn = pool
        .get()
        .await
        .map_err(|error| format!("failed to get DB connection: {error}"))?;
    set_persistent_state(
        &mut conn,
        NEXT_BLOCK_STATE_KEY,
        &scan_to_block.saturating_add(1).to_string(),
    )
    .await
    .map_err(|error| format!("failed to persist next indexer block: {error}"))?;

    Ok(credited_count)
}

async fn configured_learn_token_address(
    conn: &mut diesel_async::AsyncPgConnection,
) -> Result<Address, String> {
    let configured = get_persistent_state(conn, "learn_token_address")
        .await
        .map_err(|error| format!("failed to read learn_token_address: {error}"))?
        .or_else(|| env::var("LEARN_TOKEN_ADDRESS").ok())
        .ok_or_else(|| "learn token address is not configured".to_string())?;
    parse_address(&configured)
}

async fn configured_importer_address(
    conn: &mut diesel_async::AsyncPgConnection,
) -> Result<Option<Address>, String> {
    let configured = get_persistent_state(conn, "platform_importer_address")
        .await
        .map_err(|error| format!("failed to read platform_importer_address: {error}"))?
        .or_else(|| env::var("WALLET_DEPOSIT_IMPORTER_ADDRESS").ok())
        .or_else(|| env::var("PLATFORM_IMPORTER_ADDRESS").ok());

    configured.map(|value| parse_address(&value)).transpose()
}

fn configured_treasury_addresses() -> HashSet<Address> {
    ["WALLET_DEPOSIT_TREASURY_ADDRESS", "PLATFORM_TREASURY"]
        .into_iter()
        .filter_map(|key| env::var(key).ok())
        .filter_map(|value| match parse_address(&value) {
            Ok(address) => Some(address),
            Err(error) => {
                log::warn!(
                    "event=wallet_deposit_indexer_invalid_treasury_address value={} error={}",
                    value,
                    error
                );
                None
            }
        })
        .collect()
}
