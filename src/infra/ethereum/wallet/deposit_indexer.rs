mod build_observed_event;
mod ethereum_log_helpers;
mod indexer_config;
mod next_block_to_scan;
mod poll_logging;
mod run_wallet_deposit_indexer_once;
mod support;

pub use support::{spawn_wallet_deposit_indexer, wallet_deposit_indexer_enabled};

#[cfg(test)]
mod tests;
