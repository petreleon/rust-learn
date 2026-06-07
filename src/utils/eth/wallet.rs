use ethers::prelude::*;
use ethers::signers::coins_bip39::English;
use ethers::signers::MnemonicBuilder;
use std::env;

/// Loads wallet from ETH_MNEMONIC in .env
pub fn try_load_wallet_from_env() -> Result<Wallet<k256::ecdsa::SigningKey>, String> {
    dotenvy::dotenv().ok();
    let mnemonic = env::var("ETH_MNEMONIC").map_err(|_| "ETH_MNEMONIC not set".to_string())?;
    let chain_id = env::var("ETH_CHAIN_ID")
        .unwrap_or_else(|_| "31337".to_string())
        .parse::<u64>()
        .map_err(|error| format!("ETH_CHAIN_ID must be a number: {error}"))?;

    MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .build()
        .map(|wallet| wallet.with_chain_id(chain_id))
        .map_err(|error| format!("Failed to create wallet from mnemonic: {error}"))
}

/// Loads wallet from ETH_MNEMONIC in .env
pub fn load_wallet_from_env() -> Wallet<k256::ecdsa::SigningKey> {
    try_load_wallet_from_env().expect("Failed to load wallet from ETH_MNEMONIC/ETH_CHAIN_ID")
}
