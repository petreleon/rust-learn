use crate::db::DbPool;
use crate::repositories::persistent_state_repository::{
    get_persistent_state, set_persistent_state,
};
use crate::services::wallet_service::{
    credit_observed_wallet_deposit, ObservedWalletDepositEvent, TOKEN_TRANSFER_EVENT_IMPORT,
    TOKEN_TRANSFER_EVENT_TRANSFER,
};
use bigdecimal::BigDecimal;
use ethers::providers::Middleware;
use ethers::types::{Address, BlockNumber, Filter, Log, H256, U256, U64};
use std::collections::HashSet;
use std::env;
use std::str::FromStr;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

const NEXT_BLOCK_STATE_KEY: &str = "wallet_deposit_indexer_next_block";
const DEFAULT_POLL_SECONDS: u64 = 15;
const DEFAULT_CONFIRMATIONS: u64 = 1;
const DEFAULT_BATCH_BLOCKS: u64 = 500;
const DEFAULT_LOOKBACK_BLOCKS: u64 = 100;
const DEFAULT_IDLE_LOG_SECONDS: u64 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PollFailureLogLevel {
    Info,
    Warn,
    Suppress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PollFailureLogDecision {
    level: PollFailureLogLevel,
    consecutive_failures: u64,
    suppressed_failure_count: u64,
    outage_seconds: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PollFailureRecovery {
    consecutive_failures: u64,
    suppressed_failure_count: u64,
    outage_seconds: u64,
}

#[derive(Debug, Default)]
struct PollFailureLogState {
    first_failure_at: Option<Instant>,
    last_warning_at: Option<Instant>,
    consecutive_failures: u64,
    suppressed_failure_count: u64,
}

#[derive(Debug, Clone)]
struct WalletDepositIndexerConfig {
    poll_seconds: u64,
    confirmations: u64,
    batch_blocks: u64,
    lookback_blocks: u64,
    idle_log_seconds: u64,
    token_decimals: u32,
}

pub fn wallet_deposit_indexer_enabled() -> bool {
    env_bool("WALLET_DEPOSIT_INDEXER_ENABLED", true)
}

pub fn spawn_wallet_deposit_indexer(
    pool: DbPool,
    shutdown: Arc<AtomicBool>,
) -> Option<tokio::task::JoinHandle<()>> {
    if !wallet_deposit_indexer_enabled() {
        log::info!("event=wallet_deposit_indexer_disabled");
        return None;
    }

    Some(tokio::spawn(async move {
        run_wallet_deposit_indexer(pool, shutdown).await;
    }))
}

async fn run_wallet_deposit_indexer(pool: DbPool, shutdown: Arc<AtomicBool>) {
    let config = WalletDepositIndexerConfig::from_env();
    let idle_log_interval = Duration::from_secs(config.idle_log_seconds);
    let mut last_idle_log: Option<Instant> = None;
    let mut failure_log_state = PollFailureLogState::default();

    while !shutdown.load(Ordering::SeqCst) {
        match run_wallet_deposit_indexer_once(&pool, &config).await {
            Ok(credited_count) => {
                let now = Instant::now();
                if let Some(recovery) = failure_log_state.record_success(now) {
                    log::info!(
                        "event=wallet_deposit_indexer_poll_recovered consecutive_failures={} suppressed_failure_count={} outage_seconds={}",
                        recovery.consecutive_failures,
                        recovery.suppressed_failure_count,
                        recovery.outage_seconds
                    );
                }

                let should_log =
                    should_log_indexer_poll(credited_count, last_idle_log, idle_log_interval, now);
                if should_log {
                    if credited_count == 0 {
                        last_idle_log = Some(now);
                    }
                    log::info!(
                        "event=wallet_deposit_indexer_poll_complete credited_count={} idle_log_interval_seconds={}",
                        credited_count,
                        config.idle_log_seconds
                    );
                }
            }
            Err(error) => {
                let decision = failure_log_state.record_failure(Instant::now(), idle_log_interval);
                match decision.level {
                    PollFailureLogLevel::Info => {
                        log::info!(
                            "event=wallet_deposit_indexer_poll_retrying consecutive_failures={} outage_seconds={} warn_after_seconds={} error={}",
                            decision.consecutive_failures,
                            decision.outage_seconds,
                            config.idle_log_seconds,
                            error
                        );
                    }
                    PollFailureLogLevel::Warn => {
                        log::warn!(
                            "event=wallet_deposit_indexer_poll_failed consecutive_failures={} suppressed_failure_count={} outage_seconds={} warn_interval_seconds={} error={}",
                            decision.consecutive_failures,
                            decision.suppressed_failure_count,
                            decision.outage_seconds,
                            config.idle_log_seconds,
                            error
                        );
                    }
                    PollFailureLogLevel::Suppress => {}
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(config.poll_seconds)).await;
    }

    log::info!("event=wallet_deposit_indexer_exit reason=shutdown");
}

async fn run_wallet_deposit_indexer_once(
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
        let result = credit_observed_wallet_deposit(&mut conn, event)
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
        TOKEN_TRANSFER_EVENT_TRANSFER,
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
        TOKEN_TRANSFER_EVENT_IMPORT,
        user_address,
        importer_address,
        chain_id,
        token_decimals,
    ))
}

fn build_observed_event(
    log: Log,
    contract_address: Address,
    event_type: &str,
    from_address: Address,
    to_address: Address,
    chain_id: i64,
    token_decimals: u32,
) -> Result<ObservedWalletDepositEvent, String> {
    let transaction_hash = log
        .transaction_hash
        .ok_or_else(|| "log is missing transaction_hash".to_string())?;
    let log_index = log
        .log_index
        .ok_or_else(|| "log is missing log_index".to_string())?
        .as_u64() as i64;
    let amount = amount_from_log_data(log.data.0.as_ref(), token_decimals)?;

    Ok(ObservedWalletDepositEvent {
        chain_id,
        contract_address: format!("{:#x}", contract_address),
        transaction_hash: format!("{:#x}", transaction_hash),
        log_index,
        event_type: event_type.to_string(),
        from_address: format!("{:#x}", from_address),
        to_address: format!("{:#x}", to_address),
        amount,
    })
}

fn amount_from_log_data(data: &[u8], token_decimals: u32) -> Result<BigDecimal, String> {
    if data.len() != 32 {
        return Err(format!(
            "event amount data must be 32 bytes, got {}",
            data.len()
        ));
    }
    let amount = U256::from_big_endian(data);
    u256_to_decimal(amount, token_decimals)
}

fn u256_to_decimal(amount: U256, token_decimals: u32) -> Result<BigDecimal, String> {
    let raw = amount.to_string();
    if token_decimals == 0 {
        return BigDecimal::from_str(&raw)
            .map_err(|error| format!("failed to parse token amount: {error}"));
    }

    let decimals = token_decimals as usize;
    let decimal = if raw.len() <= decimals {
        format!("0.{}{}", "0".repeat(decimals - raw.len()), raw)
    } else {
        let split = raw.len() - decimals;
        format!("{}.{}", &raw[..split], &raw[split..])
    };
    let trimmed = decimal.trim_end_matches('0').trim_end_matches('.');
    BigDecimal::from_str(if trimmed.is_empty() { "0" } else { trimmed })
        .map_err(|error| format!("failed to parse token amount: {error}"))
}

fn address_from_topic(topic: H256) -> Option<Address> {
    Some(Address::from_slice(&topic.as_bytes()[12..]))
}

fn event_signature(signature: &str) -> H256 {
    H256::from_slice(&ethers::utils::keccak256(signature.as_bytes()))
}

fn parse_address(value: &str) -> Result<Address, String> {
    Address::from_str(value.trim())
        .map_err(|error| format!("invalid address '{}': {error}", value.trim()))
}

fn env_bool(key: &str, default: bool) -> bool {
    env::var(key)
        .ok()
        .map(|value| {
            matches!(
                value.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(default)
}

fn should_log_indexer_poll(
    credited_count: usize,
    last_idle_log: Option<Instant>,
    idle_log_interval: Duration,
    now: Instant,
) -> bool {
    if credited_count > 0 {
        return true;
    }

    last_idle_log
        .map(|logged_at| now.duration_since(logged_at) >= idle_log_interval)
        .unwrap_or(true)
}

impl PollFailureLogState {
    fn record_failure(&mut self, now: Instant, warn_interval: Duration) -> PollFailureLogDecision {
        let first_failure_at = *self.first_failure_at.get_or_insert(now);
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
        let outage = now.saturating_duration_since(first_failure_at);

        let warn_due = outage >= warn_interval
            && self
                .last_warning_at
                .map(|last_warning_at| {
                    now.saturating_duration_since(last_warning_at) >= warn_interval
                })
                .unwrap_or(true);

        if warn_due {
            let suppressed_failure_count = self.suppressed_failure_count;
            self.suppressed_failure_count = 0;
            self.last_warning_at = Some(now);
            return PollFailureLogDecision {
                level: PollFailureLogLevel::Warn,
                consecutive_failures: self.consecutive_failures,
                suppressed_failure_count,
                outage_seconds: outage.as_secs(),
            };
        }

        if self.consecutive_failures == 1 {
            return PollFailureLogDecision {
                level: PollFailureLogLevel::Info,
                consecutive_failures: self.consecutive_failures,
                suppressed_failure_count: 0,
                outage_seconds: outage.as_secs(),
            };
        }

        self.suppressed_failure_count = self.suppressed_failure_count.saturating_add(1);
        PollFailureLogDecision {
            level: PollFailureLogLevel::Suppress,
            consecutive_failures: self.consecutive_failures,
            suppressed_failure_count: self.suppressed_failure_count,
            outage_seconds: outage.as_secs(),
        }
    }

    fn record_success(&mut self, now: Instant) -> Option<PollFailureRecovery> {
        let first_failure_at = self.first_failure_at?;
        let recovery = PollFailureRecovery {
            consecutive_failures: self.consecutive_failures,
            suppressed_failure_count: self.suppressed_failure_count,
            outage_seconds: now.saturating_duration_since(first_failure_at).as_secs(),
        };
        *self = Self::default();
        Some(recovery)
    }
}

impl WalletDepositIndexerConfig {
    fn from_env() -> Self {
        WalletDepositIndexerConfig {
            poll_seconds: positive_u64_env(
                "WALLET_DEPOSIT_INDEXER_POLL_SECONDS",
                DEFAULT_POLL_SECONDS,
            ),
            confirmations: positive_u64_env(
                "WALLET_DEPOSIT_INDEXER_CONFIRMATIONS",
                DEFAULT_CONFIRMATIONS,
            ),
            batch_blocks: positive_u64_env(
                "WALLET_DEPOSIT_INDEXER_BATCH_BLOCKS",
                DEFAULT_BATCH_BLOCKS,
            ),
            lookback_blocks: positive_u64_env(
                "WALLET_DEPOSIT_INDEXER_LOOKBACK_BLOCKS",
                DEFAULT_LOOKBACK_BLOCKS,
            ),
            idle_log_seconds: positive_u64_env(
                "WALLET_DEPOSIT_INDEXER_IDLE_LOG_SECONDS",
                DEFAULT_IDLE_LOG_SECONDS,
            ),
            token_decimals: env::var("LEARN_TOKEN_DECIMALS")
                .ok()
                .and_then(|value| value.parse::<u32>().ok())
                .unwrap_or(18),
        }
    }
}

fn positive_u64_env(key: &str, default: u64) -> u64 {
    env::var(key)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::{should_log_indexer_poll, PollFailureLogLevel, PollFailureLogState};
    use std::time::{Duration, Instant};

    #[test]
    fn logs_first_idle_indexer_poll() {
        assert!(should_log_indexer_poll(
            0,
            None,
            Duration::from_secs(60),
            Instant::now()
        ));
    }

    #[test]
    fn throttles_idle_indexer_poll_until_interval_elapses() {
        let now = Instant::now();

        assert!(!should_log_indexer_poll(
            0,
            Some(now - Duration::from_secs(30)),
            Duration::from_secs(60),
            now
        ));
        assert!(should_log_indexer_poll(
            0,
            Some(now - Duration::from_secs(60)),
            Duration::from_secs(60),
            now
        ));
    }

    #[test]
    fn logs_non_idle_indexer_poll_immediately() {
        let now = Instant::now();

        assert!(should_log_indexer_poll(
            1,
            Some(now),
            Duration::from_secs(60),
            now
        ));
    }

    #[test]
    fn logs_first_indexer_poll_failure_as_retrying_info() {
        let now = Instant::now();
        let mut state = PollFailureLogState::default();

        let decision = state.record_failure(now, Duration::from_secs(60));

        assert_eq!(decision.level, PollFailureLogLevel::Info);
        assert_eq!(decision.consecutive_failures, 1);
        assert_eq!(decision.suppressed_failure_count, 0);
        assert_eq!(decision.outage_seconds, 0);
    }

    #[test]
    fn suppresses_transient_indexer_poll_failures_before_warning_interval() {
        let now = Instant::now();
        let mut state = PollFailureLogState::default();
        state.record_failure(now, Duration::from_secs(60));

        let decision = state.record_failure(now + Duration::from_secs(30), Duration::from_secs(60));

        assert_eq!(decision.level, PollFailureLogLevel::Suppress);
        assert_eq!(decision.consecutive_failures, 2);
        assert_eq!(decision.suppressed_failure_count, 1);
        assert_eq!(decision.outage_seconds, 30);
    }

    #[test]
    fn warns_for_sustained_indexer_poll_failures_at_configured_interval() {
        let now = Instant::now();
        let mut state = PollFailureLogState::default();
        state.record_failure(now, Duration::from_secs(60));
        state.record_failure(now + Duration::from_secs(30), Duration::from_secs(60));

        let decision = state.record_failure(now + Duration::from_secs(60), Duration::from_secs(60));

        assert_eq!(decision.level, PollFailureLogLevel::Warn);
        assert_eq!(decision.consecutive_failures, 3);
        assert_eq!(decision.suppressed_failure_count, 1);
        assert_eq!(decision.outage_seconds, 60);
    }

    #[test]
    fn throttles_sustained_indexer_poll_failure_warnings() {
        let now = Instant::now();
        let mut state = PollFailureLogState::default();
        state.record_failure(now, Duration::from_secs(60));
        state.record_failure(now + Duration::from_secs(30), Duration::from_secs(60));
        state.record_failure(now + Duration::from_secs(60), Duration::from_secs(60));

        let suppressed =
            state.record_failure(now + Duration::from_secs(75), Duration::from_secs(60));
        let warned_again =
            state.record_failure(now + Duration::from_secs(120), Duration::from_secs(60));

        assert_eq!(suppressed.level, PollFailureLogLevel::Suppress);
        assert_eq!(suppressed.suppressed_failure_count, 1);
        assert_eq!(warned_again.level, PollFailureLogLevel::Warn);
        assert_eq!(warned_again.consecutive_failures, 5);
        assert_eq!(warned_again.suppressed_failure_count, 1);
        assert_eq!(warned_again.outage_seconds, 120);
    }

    #[test]
    fn logs_indexer_poll_recovery_and_resets_failure_state() {
        let now = Instant::now();
        let mut state = PollFailureLogState::default();
        state.record_failure(now, Duration::from_secs(60));
        state.record_failure(now + Duration::from_secs(30), Duration::from_secs(60));

        let recovery = state
            .record_success(now + Duration::from_secs(45))
            .expect("failure state should record a recovery");

        assert_eq!(recovery.consecutive_failures, 2);
        assert_eq!(recovery.suppressed_failure_count, 1);
        assert_eq!(recovery.outage_seconds, 45);
        assert!(state
            .record_success(now + Duration::from_secs(46))
            .is_none());
    }
}
