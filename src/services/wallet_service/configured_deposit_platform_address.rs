use crate::repositories::persistent_state_repository::get_persistent_state;
use diesel_async::AsyncPgConnection;
use std::env;

use super::apply_wallet_token_ledger_entries::normalize_address;
use super::support::{WalletTokenGasPayer, WalletTokenTransferError};

pub(super) async fn configured_deposit_platform_address(
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
