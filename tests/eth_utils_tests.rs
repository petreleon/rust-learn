use ethers::prelude::*;
use rust_learn::infra::ethereum::operations::compiler::try_compile_contract;
use rust_learn::infra::ethereum::operations::deployer::try_deploy_contract;
use rust_learn::infra::ethereum::operations::provider::try_get_provider;
use rust_learn::infra::ethereum::operations::wallet::try_load_wallet_from_env;

#[test]
fn test_learn_token_artifact_exposes_allowance_burns() {
    let abi_json = std::fs::read_to_string("ethereum/artifacts/LearnToken.abi").unwrap();
    let abi: serde_json::Value = serde_json::from_str(&abi_json).unwrap();
    let functions = abi.as_array().unwrap();

    assert!(has_function(functions, "burn", &["uint256"]));
    assert!(has_function(functions, "burnFrom", &["address", "uint256"]));
    assert!(!has_function(functions, "burn", &["address", "uint256"]));
}

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

    let provider = try_get_provider();
    assert!(provider.is_ok(), "provider failed: {:?}", provider.err());

    let provider = provider.unwrap();
    let block_number = provider.get_block_number().await;
    assert!(
        block_number.is_ok(),
        "failed to get block number: {:?}",
        block_number.err()
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn test_deploy_learn_token_to_anvil() {
    let _ = dotenvy::dotenv();

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

fn has_function(functions: &[serde_json::Value], name: &str, inputs: &[&str]) -> bool {
    functions.iter().any(|item| {
        item.get("type").and_then(serde_json::Value::as_str) == Some("function")
            && item.get("name").and_then(serde_json::Value::as_str) == Some(name)
            && item
                .get("inputs")
                .and_then(serde_json::Value::as_array)
                .map(|abi_inputs| {
                    abi_inputs
                        .iter()
                        .filter_map(|input| input.get("type").and_then(serde_json::Value::as_str))
                        .eq(inputs.iter().copied())
                })
                .unwrap_or(false)
    })
}
