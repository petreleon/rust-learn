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

    // Compile and deploy LearnToken
    let (abi, bytecode) = compile_contract("LearnToken.sol", "LearnToken");
    let token_addr = deploy_contract(
        wallet.clone(),
        provider.clone(),
        abi.clone(),
        bytecode,
        ("Test Token".to_string(), "TST".to_string(), 18u8),
    )
    .await;

    let client = std::sync::Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
    let token = Contract::new(token_addr, abi.clone(), client.clone());

    // Mint tokens to the test wallet
    let my_addr = wallet.address();
    let amount_to_mint = U256::from(100) * U256::from(10).pow(U256::from(18)); // 100 tokens
    let mint_call: ContractCall<_, ()> = token.method("mint", (my_addr, amount_to_mint)).unwrap();
    let pending = mint_call.send().await.expect("mint send");
    let _ = pending.await.expect("mint confirm").unwrap();

    // Verify minted balance
    let balance: U256 = token.method::<_, U256>("balanceOf", my_addr).unwrap().call().await.expect("balanceOf");
    assert_eq!(balance, amount_to_mint);

    // Deploy LearnTokenPresigner
    let (presigner_abi, presigner_bytecode) = compile_contract("LearnTokenPresigner.sol", "LearnTokenPresigner");
    let presigner_addr = deploy_contract(
        wallet.clone(),
        provider.clone(),
        presigner_abi.clone(),
        presigner_bytecode,
        (token_addr,),
    )
    .await;
    let presigner = Contract::new(presigner_addr, presigner_abi.clone(), client.clone());

    // Approve presigner to transfer tokens and deposit
    let approve_call = token.method::<_, bool>("approve", (presigner_addr, amount_to_mint)).unwrap();
    let p = approve_call.send().await.expect("approve send");
    let _ = p.await.expect("approve confirm").unwrap();

    // Deposit into presigner
    let deposit_call = presigner.method::<_, ()>("deposit", amount_to_mint).unwrap();
    let d = deposit_call.send().await.expect("deposit send");
    let _ = d.await.expect("deposit confirm").unwrap();

    // Withdraw back
    let withdraw_call = presigner.method::<_, ()>("withdraw", amount_to_mint).unwrap();
    let w = withdraw_call.send().await.expect("withdraw send");
    let _ = w.await.expect("withdraw confirm").unwrap();

    // Deploy PlatformImporter (we won't test permit here)
    let (importer_abi, importer_bytecode) = compile_contract("PlatformImporter.sol", "PlatformImporter");
    let treasury = my_addr; // use self as treasury for test
    let importer_addr = deploy_contract(
        wallet.clone(),
        provider.clone(),
        importer_abi.clone(),
        importer_bytecode,
        (treasury,),
    )
    .await;

    // Sanity: deployed addresses are non-zero
    assert_ne!(token_addr, Address::zero());
    assert_ne!(presigner_addr, Address::zero());
    assert_ne!(importer_addr, Address::zero());
}
