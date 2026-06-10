use rust_learn::utils::eth_utils::{
    try_compile_contract, try_deploy_contract, try_get_provider, try_load_wallet_from_env,
};
use ethers::prelude::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_compile_learn_token_from_source() {
    let _ = dotenvy::dotenv();
    std::env::set_var("ETH_CONTRACT_COMPILE_FROM_SOURCE", "1");

    let result = try_compile_contract("LearnToken.sol", "LearnToken");
    assert!(result.is_ok(), "compile failed: {:?}", result.err());
    let (abi, bytecode) = result.unwrap();
    assert!(!abi.functions.is_empty(), "ABI should have functions");
    assert!(!bytecode.is_empty(), "bytecode should not be empty");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_compile_learn_token_loads_artifact() {
    let _ = dotenvy::dotenv();
    std::env::set_var("ETH_CONTRACT_COMPILE_FROM_SOURCE", "0");

    let result = try_compile_contract("LearnToken.sol", "LearnToken");
    assert!(result.is_ok(), "artifact load failed: {:?}", result.err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_provider_connects_to_anvil() {
    let _ = dotenvy::dotenv();
    std::env::set_var("ETH_RPC_URL", "http://localhost:8545");
    std::env::set_var("ETH_HOST", "localhost");
    std::env::set_var("ETH_PORT", "8545");

    let provider = try_get_provider();
    assert!(provider.is_ok(), "provider failed: {:?}", provider.err());

    let provider = provider.unwrap();
    let block_number = provider.get_block_number().await;
    assert!(block_number.is_ok(), "failed to get block number: {:?}", block_number.err());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_deploy_learn_token_to_anvil() {
    let _ = dotenvy::dotenv();
    std::env::set_var("ETH_RPC_URL", "http://localhost:8545");
    std::env::set_var("ETH_HOST", "localhost");
    std::env::set_var("ETH_PORT", "8545");

    let wallet = try_load_wallet_from_env();
    assert!(wallet.is_ok(), "wallet load failed: {:?}", wallet.err());
    let wallet = wallet.unwrap().with_chain_id(31337u64);

    let provider = try_get_provider().unwrap();

    let (abi, bytecode) = try_compile_contract("LearnToken.sol", "LearnToken").unwrap();

    let result = try_deploy_contract(
        wallet.clone(),
        provider.clone(),
        abi.clone(),
        bytecode,
        ("EthTest".to_string(), "ETT".to_string(), 18u8),
    )
    .await;

    assert!(result.is_ok(), "deploy failed: {:?}", result.err());
}
