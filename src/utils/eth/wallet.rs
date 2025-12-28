use ethers::prelude::*;
use ethers::signers::coins_bip39::English;
use ethers::signers::MnemonicBuilder;
use std::env;

/// Loads wallet from ETH_MNEMONIC in .env
pub fn load_wallet_from_env() -> Wallet<k256::ecdsa::SigningKey> {
    dotenvy::dotenv().ok();
    let mnemonic = env::var("ETH_MNEMONIC").expect("ETH_MNEMONIC not set");
    MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .build()
        .expect("Failed to create wallet from mnemonic")
}
