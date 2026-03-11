use ethers::prelude::*;
use ethers::signers::coins_bip39::English;
use ethers::signers::MnemonicBuilder;
use std::env;

/// Loads wallet from ETH_MNEMONIC in .env
pub fn load_wallet_from_env() -> Wallet<k256::ecdsa::SigningKey> {
    dotenvy::dotenv().ok();
    let mnemonic = env::var("ETH_MNEMONIC").expect("ETH_MNEMONIC not set");
    let chain_id = env::var("ETH_CHAIN_ID")
        .unwrap_or_else(|_| "31337".to_string())
        .parse::<u64>()
        .expect("ETH_CHAIN_ID must be a number");

    MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .build()
        .expect("Failed to create wallet from mnemonic")
        .with_chain_id(chain_id)
}
