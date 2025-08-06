// Utility functions for Ethereum contract interaction
// src/utils/eth_utils.rs

use ethers::prelude::*;
use std::env;

/// Loads wallet from ETH_MNEMONIC in .env
pub fn load_wallet_from_env() -> Wallet<k256::ecdsa::SigningKey> {
    dotenvy::dotenv().ok();
    let mnemonic = env::var("ETH_MNEMONIC").expect("ETH_MNEMONIC not set");
    Wallet::from_mnemonic(&mnemonic, None).expect("Failed to create wallet from mnemonic")
}

/// Connects to local Ethereum node
pub fn get_provider() -> Provider<Http> {
    Provider::<Http>::try_from("http://localhost:8545").expect("Could not connect to provider")
}

/// Compiles the LearnToken contract using ethers-solc
pub fn compile_contract() -> (Abi, Bytes) {
    use ethers_solc::{Solc, Project, ProjectPathsConfig};
    let paths = ProjectPathsConfig::builder()
        .root("./ethereum/contracts")
        .sources("./ethereum/contracts")
        .build().unwrap();
    let project = Project::builder().paths(paths).build().unwrap();
    let output = project.compile().unwrap();
    let contract = output.find("LearnToken").unwrap();
    let abi = contract.abi.unwrap().clone();
    let bytecode = contract.bytecode.unwrap().clone();
    (abi, bytecode)
}

/// Deploys the LearnToken contract
pub async fn deploy_contract(
    wallet: Wallet<k256::ecdsa::SigningKey>,
    provider: Provider<Http>,
    abi: Abi,
    bytecode: Bytes,
    name: String,
    symbol: String,
    decimals: u8,
) -> Address {
    let client = SignerMiddleware::new(provider, wallet);
    let factory = ContractFactory::new(abi, bytecode, client);
    let deployer = factory.deploy((name, symbol, decimals)).unwrap();
    let contract = deployer.send().await.unwrap();
    contract.address()
}
