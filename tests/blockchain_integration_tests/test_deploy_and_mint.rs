use crate::support::*;

// This test requires a running Anvil node accessible via the .env configuration.
// It is ignored by default.
// Run it explicitly when your dev environment is up:
// `cargo test --test blockchain_integration_tests -- --ignored`
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_deploy_and_mint() -> Result<()> {
    // 1. Setup: Load wallet, provider, and compile contract
    // Use a dedicated derivation index to avoid nonce clashes with any persistent anvil state
    let mnemonic = std::env::var("ETH_MNEMONIC").context("ETH_MNEMONIC not set for test")?;
    let wallet = MnemonicBuilder::<English>::default()
        .phrase(mnemonic.as_str())
        .index(2u32)
        .context("failed to set derivation index")?
        .build()
        .context("failed to build deployer wallet")?
        .with_chain_id(31337u64);
    let provider = try_get_provider().map_err(anyhow::Error::msg)?;

    // Compile and deploy LearnToken
    let (abi, bytecode) =
        try_compile_contract("LearnToken.sol", "LearnToken").map_err(anyhow::Error::msg)?;
    let token_addr = try_deploy_contract(
        wallet.clone(),
        provider.clone(),
        abi.clone(),
        bytecode,
        ("Test Token".to_string(), "TST".to_string(), 18u8),
    )
    .await
    .map_err(anyhow::Error::msg)?;

    let client = std::sync::Arc::new(SignerMiddleware::new(provider.clone(), wallet.clone()));
    let token = Contract::new(token_addr, abi.clone(), client.clone());

    // Mint tokens to the test wallet
    let my_addr = wallet.address();
    let amount_to_mint = U256::from(100) * U256::from(10).pow(U256::from(18)); // 100 tokens
    let mint_call: ContractCall<_, ()> = token
        .method("mint", (my_addr, amount_to_mint))
        .context("failed to build mint call")?;
    let pending = mint_call.send().await.context("mint send failed")?;
    let _ = pending
        .await
        .context("mint confirmation failed")?
        .context("mint transaction was dropped before receipt")?;

    // Verify minted balance
    let balance: U256 = token
        .method::<_, U256>("balanceOf", my_addr)
        .context("failed to build balanceOf call")?
        .call()
        .await
        .context("balanceOf call failed")?;
    assert_eq!(balance, amount_to_mint);

    // Deploy LearnTokenPresigner
    let (presigner_abi, presigner_bytecode) =
        try_compile_contract("LearnTokenPresigner.sol", "LearnTokenPresigner")
            .map_err(anyhow::Error::msg)?;
    let presigner_addr = try_deploy_contract(
        wallet.clone(),
        provider.clone(),
        presigner_abi.clone(),
        presigner_bytecode,
        (token_addr,),
    )
    .await
    .map_err(anyhow::Error::msg)?;
    let presigner = Contract::new(presigner_addr, presigner_abi.clone(), client.clone());

    // Approve presigner to transfer tokens and deposit
    let approve_call = token
        .method::<_, bool>("approve", (presigner_addr, amount_to_mint))
        .context("failed to build approve call")?;
    let p = approve_call.send().await.context("approve send failed")?;
    let _ = p
        .await
        .context("approve confirmation failed")?
        .context("approve transaction was dropped before receipt")?;

    // Deposit into presigner
    let deposit_call = presigner
        .method::<_, ()>("deposit", amount_to_mint)
        .context("failed to build deposit call")?;
    let d = deposit_call.send().await.context("deposit send failed")?;
    let _ = d
        .await
        .context("deposit confirmation failed")?
        .context("deposit transaction was dropped before receipt")?;

    // Withdraw back
    let withdraw_call = presigner
        .method::<_, ()>("withdraw", amount_to_mint)
        .context("failed to build withdraw call")?;
    let w = withdraw_call.send().await.context("withdraw send failed")?;
    let _ = w
        .await
        .context("withdraw confirmation failed")?
        .context("withdraw transaction was dropped before receipt")?;

    // Deploy PlatformImporter (we won't test permit here)
    let (importer_abi, importer_bytecode) =
        try_compile_contract("PlatformImporter.sol", "PlatformImporter")
            .map_err(anyhow::Error::msg)?;
    let treasury = my_addr; // use self as treasury for test
    let importer_addr = try_deploy_contract(
        wallet.clone(),
        provider.clone(),
        importer_abi.clone(),
        importer_bytecode,
        (treasury,),
    )
    .await
    .map_err(anyhow::Error::msg)?;

    // Sanity: deployed addresses are non-zero
    assert_ne!(token_addr, Address::zero());
    assert_ne!(presigner_addr, Address::zero());
    assert_ne!(importer_addr, Address::zero());
    Ok(())
}
