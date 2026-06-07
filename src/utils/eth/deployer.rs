use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use ethers::abi::Abi;
use ethers::core::types::Bytes;
use ethers::prelude::*;
use std::env;
use std::str::FromStr;

use super::compiler::compile_contract;
use super::provider::try_get_provider;
use super::wallet::try_load_wallet_from_env;
use crate::repositories::persistent_state_repository::{
    get_persistent_state, set_persistent_state,
};

fn deployment_error(message: impl Into<String>) -> diesel::result::Error {
    diesel::result::Error::QueryBuilderError(Box::new(std::io::Error::other(message.into())))
}

/// Deploys the LearnToken contract
pub async fn try_deploy_contract(
    mut wallet: Wallet<k256::ecdsa::SigningKey>,
    provider: Provider<Http>,
    abi: Abi,
    bytecode: Bytes,
    deploy_args: impl ethers::core::abi::Tokenize,
) -> Result<Address, String> {
    let chain_id = provider
        .get_chainid()
        .await
        .map_err(|error| format!("Failed to get chain id: {error}"))?;
    log::info!("event=eth_deploy_start chain_id={}", chain_id);
    wallet = wallet.with_chain_id(chain_id.as_u64());
    let client = std::sync::Arc::new(SignerMiddleware::new(provider, wallet));
    let factory = ContractFactory::new(abi, bytecode, client);
    let deployer = factory
        .deploy(deploy_args)
        .map_err(|error| format!("Failed to prepare contract deployment: {error}"))?;
    let contract = deployer
        .send()
        .await
        .map_err(|error| format!("Failed to send contract deployment: {error}"))?;
    log::info!("event=eth_deploy_success address={:#x}", contract.address());
    Ok(contract.address())
}

/// Deploys the LearnToken contract
pub async fn deploy_contract(
    wallet: Wallet<k256::ecdsa::SigningKey>,
    provider: Provider<Http>,
    abi: Abi,
    bytecode: Bytes,
    deploy_args: impl ethers::core::abi::Tokenize,
) -> Address {
    try_deploy_contract(wallet, provider, abi, bytecode, deploy_args)
        .await
        .expect("Failed to deploy contract")
}

pub async fn deploy_learn_token_and_save(
    conn: &mut AsyncPgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<Address> {
    let wallet = try_load_wallet_from_env().map_err(deployment_error)?;
    let provider = try_get_provider().map_err(deployment_error)?;

    let (abi, bytecode) = compile_contract("LearnToken.sol", "LearnToken");
    let addr = try_deploy_contract(
        wallet,
        provider,
        abi,
        bytecode,
        (name.to_string(), symbol.to_string(), decimals),
    )
    .await
    .map_err(deployment_error)?;

    let addr_hex = format!("{:#x}", addr);
    set_persistent_state(conn, "learn_token_address", &addr_hex).await?;
    log::info!(
        "event=eth_contract_saved contract=LearnToken address={}",
        addr_hex
    );

    Ok(addr)
}

pub async fn deploy_learn_token_presigner_and_save(
    conn: &mut AsyncPgConnection,
    learn_token_addr: Address,
) -> QueryResult<Address> {
    let wallet = try_load_wallet_from_env().map_err(deployment_error)?;
    let provider = try_get_provider().map_err(deployment_error)?;

    let (abi, bytecode) = compile_contract("LearnTokenPresigner.sol", "LearnTokenPresigner");
    let addr = try_deploy_contract(wallet, provider, abi, bytecode, (learn_token_addr,))
        .await
        .map_err(deployment_error)?;

    let addr_hex = format!("{:#x}", addr);
    set_persistent_state(conn, "learn_token_presigner_address", &addr_hex).await?;
    log::info!(
        "event=eth_contract_saved contract=LearnTokenPresigner address={}",
        addr_hex
    );
    Ok(addr)
}

pub async fn deploy_platform_importer_and_save(
    conn: &mut AsyncPgConnection,
    treasury: Address,
) -> QueryResult<Address> {
    let wallet = try_load_wallet_from_env().map_err(deployment_error)?;
    let provider = try_get_provider().map_err(deployment_error)?;

    let (abi, bytecode) = compile_contract("PlatformImporter.sol", "PlatformImporter");
    let addr = try_deploy_contract(wallet, provider, abi, bytecode, (treasury,))
        .await
        .map_err(deployment_error)?;

    let addr_hex = format!("{:#x}", addr);
    set_persistent_state(conn, "platform_importer_address", &addr_hex).await?;
    log::info!(
        "event=eth_contract_saved contract=PlatformImporter address={}",
        addr_hex
    );
    Ok(addr)
}

pub async fn get_learn_token_address(conn: &mut AsyncPgConnection) -> QueryResult<Option<Address>> {
    if let Some(s) = get_persistent_state(conn, "learn_token_address").await? {
        let parsed = Address::from_str(s.trim_start_matches("0x"))
            .map_err(|_| diesel::result::Error::NotFound)?;
        Ok(Some(parsed))
    } else {
        Ok(None)
    }
}

pub async fn deploy_all_startup(
    conn: &mut AsyncPgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<(Address, Option<Address>, Option<Address>)> {
    let token_addr = if let Some(a) = get_persistent_state(conn, "learn_token_address").await? {
        Address::from_str(a.trim_start_matches("0x"))
            .map_err(|_| diesel::result::Error::NotFound)?
    } else {
        deploy_learn_token_and_save(conn, name, symbol, decimals).await?;
        let s = get_persistent_state(conn, "learn_token_address")
            .await?
            .ok_or_else(|| deployment_error("learn_token_address missing after deployment"))?;
        Address::from_str(s.trim_start_matches("0x"))
            .map_err(|_| diesel::result::Error::NotFound)?
    };

    let presigner_addr =
        if let Some(a) = get_persistent_state(conn, "learn_token_presigner_address").await? {
            Some(
                Address::from_str(a.trim_start_matches("0x"))
                    .map_err(|_| diesel::result::Error::NotFound)?,
            )
        } else {
            let addr = deploy_learn_token_presigner_and_save(conn, token_addr).await?;
            Some(addr)
        };

    let importer_addr =
        if let Some(a) = get_persistent_state(conn, "platform_importer_address").await? {
            Some(
                Address::from_str(a.trim_start_matches("0x"))
                    .map_err(|_| diesel::result::Error::NotFound)?,
            )
        } else {
            let treasury_addr = if let Ok(t) = env::var("PLATFORM_TREASURY") {
                Address::from_str(t.trim_start_matches("0x"))
                    .map_err(|_| diesel::result::Error::NotFound)?
            } else {
                let wallet = try_load_wallet_from_env().map_err(deployment_error)?;
                wallet.address()
            };
            let addr = deploy_platform_importer_and_save(conn, treasury_addr).await?;
            Some(addr)
        };

    Ok((token_addr, presigner_addr, importer_addr))
}

pub async fn deploy_startup(
    conn: &mut AsyncPgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<Address> {
    if let Some(addr) = get_learn_token_address(conn).await? {
        log::info!(
            "event=eth_contract_reused contract=LearnToken address={:#x}",
            addr
        );
        Ok(addr)
    } else {
        log::info!("event=eth_contract_missing contract=LearnToken action=deploy");
        deploy_learn_token_and_save(conn, name, symbol, decimals).await
    }
}
