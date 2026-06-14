use crate::db::DbPool;
use crate::services::wallet_deposit_indexer_service::poll_logging::{
    should_log_indexer_poll, PollFailureLogLevel, PollFailureLogState,
};
use crate::services::wallet_deposit_indexer_service::run_wallet_deposit_indexer_once::run_wallet_deposit_indexer_once;
use std::env;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::{Duration, Instant};

pub(super) const NEXT_BLOCK_STATE_KEY: &str = "wallet_deposit_indexer_next_block";
pub(super) const DEFAULT_POLL_SECONDS: u64 = 15;
pub(super) const DEFAULT_CONFIRMATIONS: u64 = 1;
pub(super) const DEFAULT_BATCH_BLOCKS: u64 = 500;
pub(super) const DEFAULT_LOOKBACK_BLOCKS: u64 = 100;
pub(super) const DEFAULT_IDLE_LOG_SECONDS: u64 = 60;

#[derive(Debug, Clone)]
pub(super) struct WalletDepositIndexerConfig {
    pub(super) poll_seconds: u64,
    pub(super) confirmations: u64,
    pub(super) batch_blocks: u64,
    pub(super) lookback_blocks: u64,
    pub(super) idle_log_seconds: u64,
    pub(super) token_decimals: u32,
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
