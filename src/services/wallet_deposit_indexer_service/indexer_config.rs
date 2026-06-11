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
mod tests;
