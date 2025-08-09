// Utility functions for Ethereum contract interaction
// src/utils/eth_utils.rs

use crate::utils::db_utils::persistent_state::{get_persistent_state, set_persistent_state};
use diesel::prelude::*;
use diesel::{dsl::excluded, PgConnection, QueryResult};
use ethers::prelude::*;
use std::env;
use std::str::FromStr;

/// Loads wallet from ETH_MNEMONIC in .env
pub fn load_wallet_from_env() -> Wallet<k256::ecdsa::SigningKey> {
    dotenvy::dotenv().ok();
    let mnemonic = env::var("ETH_MNEMONIC").expect("ETH_MNEMONIC not set");
    Wallet::from_mnemonic(&mnemonic, None).expect("Failed to create wallet from mnemonic")
}

/// Connects to an Ethereum node using env configuration
pub fn get_provider() -> Provider<Http> {
    dotenvy::dotenv().ok();
    let url = std::env::var("ETH_RPC_URL").ok().unwrap_or_else(|| {
        let host = std::env::var("ETH_HOST").unwrap_or_else(|_| "geth".to_string());
        let port = std::env::var("ETH_PORT").unwrap_or_else(|_| "8545".to_string());
        format!("http://{}:{}", host, port)
    });
    Provider::<Http>::try_from(url).expect("Could not create provider from ETH_RPC_URL/ETH_HOST/ETH_PORT")
}

/// Compiles the LearnToken contract using ethers-solc
pub fn compile_contract() -> (Abi, Bytes) {
    use ethers_solc::{Project, ProjectPathsConfig, Solc};
    let paths = ProjectPathsConfig::builder()
        .root("./ethereum/contracts")
        .sources("./ethereum/contracts")
        .build()
        .unwrap();
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

/// Deploy LearnToken and store its address in persistent_states under `learn_token_address`
pub async fn deploy_learn_token_and_save(
    conn: &mut PgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<Address> {
    // Load wallet and provider
    let wallet = load_wallet_from_env();
    let provider = get_provider();

    // Compile and deploy
    let (abi, bytecode) = compile_contract();
    let addr = deploy_contract(
        wallet,
        provider,
        abi,
        bytecode,
        name.to_string(),
        symbol.to_string(),
        decimals,
    )
    .await
    .map_err(|_| diesel::result::Error::NotFound)?; // Map ethers error to a Diesel error kind

    // Persist address as 0x-prefixed hex
    let addr_hex = format!("{:#x}", addr);
    set_persistent_state(conn, "learn_token_address", &addr_hex)?;

    Ok(addr)
}

/// Retrieve the stored LearnToken address if present
pub fn get_learn_token_address(conn: &mut PgConnection) -> QueryResult<Option<Address>> {
    if let Some(s) = get_persistent_state(conn, "learn_token_address")? {
        let parsed = Address::from_str(s.trim_start_matches("0x"))
            .map_err(|_| diesel::result::Error::NotFound)?;
        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}

/// On startup, ensure LearnToken is deployed. If already stored, reuse it; otherwise deploy and save.
pub async fn deploy_startup(
    conn: &mut PgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<Address> {
    if let Some(addr) = get_learn_token_address(conn)? {
        Ok(addr)
    } else {
        deploy_learn_token_and_save(conn, name, symbol, decimals).await
    }
}
