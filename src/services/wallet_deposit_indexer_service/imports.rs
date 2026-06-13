use crate::application::wallet::index_deposit::{
    index_observed_deposit, ObservedWalletDepositEvent,
};
use crate::db::DbPool;
use crate::domain::wallet::deposit::{
    WALLET_DEPOSIT_EVENT_IMPORT, WALLET_DEPOSIT_EVENT_TRANSFER,
};
use crate::infra::postgres::wallet::wallet_deposit_index_store::PostgresWalletDepositIndexStore;
use crate::repositories::persistent_state_repository::{
    get_persistent_state, set_persistent_state,
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
