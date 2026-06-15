use crate::{permit_helpers::*, support::*};

// Test EIP-2612 permit flow with PlatformImporter.importWithPermit
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn test_permit_import() -> Result<()> {
    // Generate distinct mnemonics for deployer and owner
    let mut entropy_deployer = [0u8; 16];
    getrandom(&mut entropy_deployer).context("failed to get randomness for deployer")?;
    let mnemonic_deployer =
        Mnemonic::from_entropy(&entropy_deployer).context("failed to build deployer mnemonic")?;
    let phrase_deployer = mnemonic_deployer.to_string();

    let mut entropy_owner = [0u8; 16];
    getrandom(&mut entropy_owner).context("failed to get randomness for owner")?;
    let mnemonic_owner =
        Mnemonic::from_entropy(&entropy_owner).context("failed to build owner mnemonic")?;
    let phrase_owner = mnemonic_owner.to_string();

    // Deployer and owner come from different mnemonics (both at index 0)
    let deployer_wallet = MnemonicBuilder::<English>::default()
        .phrase(phrase_deployer.as_str())
        .index(0u32)
        .context("failed to set derivation index")?
        .build()
        .context("failed to build deployer wallet")?
        .with_chain_id(31337u64);
    let provider = try_get_provider().map_err(anyhow::Error::msg)?;

    // Fund the freshly-generated deployer wallet from the default Anvil-funded faucet
    let faucet = try_load_wallet_from_env()
        .map_err(anyhow::Error::msg)?
        .with_chain_id(31337u64);
    let faucet_client = std::sync::Arc::new(SignerMiddleware::new(provider.clone(), faucet));
    let fund_value = U256::from(10u64) * U256::from(10).pow(U256::from(18)); // 10 ETH
    let tx = TransactionRequest::pay(deployer_wallet.address(), fund_value);
    let pending = faucet_client
        .send_transaction(tx, None)
        .await
        .context("funding deployer transaction send failed")?;
    let _ = pending
        .await
        .context("funding deployer transaction confirmation failed")?;

    // Deploy token
    let (abi, bytecode) =
        try_compile_contract("LearnToken.sol", "LearnToken").map_err(anyhow::Error::msg)?;
    let token_addr = try_deploy_contract(
        deployer_wallet.clone(),
        provider.clone(),
        abi.clone(),
        bytecode,
        ("Permit Token".to_string(), "PTKN".to_string(), 18u8),
    )
    .await
    .map_err(anyhow::Error::msg)?;

    let client = std::sync::Arc::new(SignerMiddleware::new(
        provider.clone(),
        deployer_wallet.clone(),
    ));
    let token = Contract::new(token_addr, abi.clone(), client.clone());

    // Derive a separate owner wallet from the same mnemonic at index 1 (this wallet will sign the permit)
    let owner_wallet = MnemonicBuilder::<English>::default()
        .phrase(phrase_owner.as_str())
        .index(0u32)
        .context("failed to set derivation index")?
        .build()
        .context("failed to build owner wallet")?;
    let owner = owner_wallet.address();

    // Mint tokens to owner
    let amount = U256::from(50) * U256::from(10).pow(U256::from(18));
    let _ = token
        .method::<_, ()>("mint", (owner, amount))
        .context("failed to build mint call")?
        .send()
        .await
        .context("mint send failed")?
        .await
        .context("mint confirmation failed")?
        .context("mint transaction was dropped before receipt")?;

    // Deploy importer with treasury = random address
    let (importer_abi, importer_bytecode) =
        try_compile_contract("PlatformImporter.sol", "PlatformImporter")
            .map_err(anyhow::Error::msg)?;
    let treasury = Address::random();
    let importer_addr = try_deploy_contract(
        deployer_wallet.clone(),
        provider.clone(),
        importer_abi.clone(),
        importer_bytecode,
        (treasury,),
    )
    .await
    .map_err(anyhow::Error::msg)?;
    let importer = Contract::new(importer_addr, importer_abi.clone(), client.clone());

    use ethers::core::types::H256;

    let nonce: U256 = token
        .method::<_, U256>("nonces", owner)
        .context("failed to build nonces call")?
        .call()
        .await
        .context("nonces call failed")?;
    let domain_separator: H256 = token
        .method::<_, H256>("DOMAIN_SEPARATOR", ())
        .context("failed to build DOMAIN_SEPARATOR call")?
        .call()
        .await
        .context("DOMAIN_SEPARATOR call failed")?;

    let deadline = U256::from(9999999999u64);
    let digest_h256 = eip2612_permit_digest(
        domain_separator,
        owner,
        importer_addr,
        amount,
        nonce,
        deadline,
    );

    let sig = owner_wallet
        .sign_hash(digest_h256)
        .context("failed to sign permit digest")?;
    let (v, r_bytes, s_bytes) = permit_signature_parts(sig);

    // Call importWithPermit
    let _receipt = importer
        .method::<_, ()>(
            "importWithPermit",
            (token_addr, owner, amount, deadline, v, r_bytes, s_bytes),
        )
        .context("failed to build importWithPermit call")?
        .send()
        .await
        .context("importWithPermit transaction send failed")?
        .await
        .context("importWithPermit transaction confirmation failed")?;

    // Check treasury balance increased
    let bal: U256 = token
        .method::<_, U256>("balanceOf", treasury)
        .context("failed to build treasury balanceOf call")?
        .call()
        .await
        .context("treasury balanceOf call failed")?;
    assert_eq!(bal, amount);
    Ok(())
}
