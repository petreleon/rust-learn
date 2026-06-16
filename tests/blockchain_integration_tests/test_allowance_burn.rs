use crate::support::*;

// Run with the ignored blockchain suite when Anvil is available.
#[tokio::test(flavor = "multi_thread")]
#[ignore]
async fn token_burns_require_holder_or_allowance() -> Result<()> {
    let mnemonic = std::env::var("ETH_MNEMONIC").context("ETH_MNEMONIC not set for test")?;
    let owner_wallet = indexed_wallet(&mnemonic, 3)?;
    let user_wallet = indexed_wallet(&mnemonic, 4)?;
    let provider = try_get_provider().map_err(anyhow::Error::msg)?;
    let owner_client = std::sync::Arc::new(SignerMiddleware::new(
        provider.clone(),
        owner_wallet.clone(),
    ));
    let user_client =
        std::sync::Arc::new(SignerMiddleware::new(provider.clone(), user_wallet.clone()));

    let (abi, bytecode) =
        try_compile_contract("LearnToken.sol", "LearnToken").map_err(anyhow::Error::msg)?;
    let token_addr = try_deploy_contract(
        owner_wallet.clone(),
        provider,
        abi.clone(),
        bytecode,
        ("Burn Token".to_string(), "BURN".to_string(), 18u8),
    )
    .await
    .map_err(anyhow::Error::msg)?;
    let owner_token = Contract::new(token_addr, abi.clone(), owner_client);
    let user_token = Contract::new(token_addr, abi, user_client);

    let user = user_wallet.address();
    let owner = owner_wallet.address();
    let starting_balance = U256::from(100);
    await_tx(owner_token.method::<_, ()>("mint", (user, starting_balance))?).await?;

    let unauthorized_call = owner_token.method::<_, ()>("burnFrom", (user, U256::from(10)))?;
    let unauthorized = unauthorized_call.send().await;
    assert!(unauthorized.is_err());
    assert_eq!(balance_of(&owner_token, user).await?, starting_balance);

    await_tx(user_token.method::<_, ()>("burn", U256::from(7))?).await?;
    assert_eq!(balance_of(&owner_token, user).await?, U256::from(93));

    await_tx(user_token.method::<_, bool>("approve", (owner, U256::from(11)))?).await?;
    await_tx(owner_token.method::<_, ()>("burnFrom", (user, U256::from(11)))?).await?;

    assert_eq!(balance_of(&owner_token, user).await?, U256::from(82));
    let allowance: U256 = owner_token
        .method("allowance", (user, owner))?
        .call()
        .await?;
    assert_eq!(allowance, U256::zero());
    Ok(())
}

fn indexed_wallet(mnemonic: &str, index: u32) -> Result<LocalWallet> {
    Ok(MnemonicBuilder::<English>::default()
        .phrase(mnemonic)
        .index(index)
        .context("failed to set derivation index")?
        .build()
        .context("failed to build wallet")?
        .with_chain_id(31337u64))
}

async fn balance_of<M: Middleware + 'static>(
    token: &Contract<M>,
    account: Address,
) -> Result<U256> {
    token
        .method::<_, U256>("balanceOf", account)?
        .call()
        .await
        .context("balanceOf call failed")
}

async fn await_tx<M: Middleware + 'static, D: ethers::abi::Detokenize>(
    call: ContractCall<M, D>,
) -> Result<()> {
    call.send()
        .await
        .context("transaction send failed")?
        .await
        .context("transaction confirmation failed")?
        .context("transaction was dropped before receipt")?;
    Ok(())
}
