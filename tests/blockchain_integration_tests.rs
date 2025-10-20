use rust_learn::utils::eth_utils::{compile_contract, deploy_contract, get_provider, load_wallet_from_env};
use ethers::prelude::*;

// This test requires a running Anvil node accessible via the .env configuration.
// It is ignored by default.
// Run it explicitly when your dev environment is up:
// `cargo test --test blockchain_integration_tests -- --ignored`
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_deploy_and_mint() {
    // 1. Setup: Load wallet, provider, and compile contract
    let wallet = load_wallet_from_env().with_chain_id(31337u64);
    let provider = get_provider();
    let (abi, bytecode) = compile_contract();

    // 2. Deploy the contract
    let contract_address = deploy_contract(
        wallet.clone(),
        provider.clone(),
        abi.clone(),
        bytecode,
        "Test Token".to_string(),
        "TST".to_string(),
        18,
    )
    .await;

    // 3. Create a contract instance
    let client = std::sync::Arc::new(SignerMiddleware::new(provider, wallet.clone()));
    let contract = Contract::new(contract_address, abi, client);

    // 4. Mint tokens
    let mint_to_address: Address = "0x70997970C51812dc3A010C7d01b50e0d17dc79C8".parse().unwrap();
    let amount_to_mint = U256::from(100) * U256::from(10).pow(U256::from(18)); // 100 tokens

    let mint_call: ContractCall<_, ()> = contract.method("mint", (mint_to_address, amount_to_mint)).unwrap();
    
    let pending_tx = mint_call.send().await.expect("Minting transaction failed to send");
    let receipt = pending_tx.await.expect("Minting transaction failed to confirm").unwrap();

    assert_eq!(receipt.status, Some(1.into()), "Minting transaction failed");

    // 5. Verify the balance
    let balance_call: ContractCall<_, U256> = contract.method("balanceOf", mint_to_address).unwrap();
    let balance = balance_call.call().await.expect("Failed to get balance");

    assert_eq!(balance, amount_to_mint, "The token balance is incorrect after minting.");
}
