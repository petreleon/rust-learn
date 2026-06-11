use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use ethers::abi::Abi;
use ethers::core::types::Bytes;
use ethers::prelude::*;

use super::compiler::try_compile_contract;
use super::provider::try_get_provider;
use super::wallet::try_load_wallet_from_env;
use crate::repositories::persistent_state_repository::set_persistent_state;

mod startup;

pub use startup::{deploy_all_startup, deploy_startup, get_learn_token_address};

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

pub async fn deploy_learn_token_and_save(
    conn: &mut AsyncPgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<Address> {
    let wallet = try_load_wallet_from_env().map_err(deployment_error)?;
    let provider = try_get_provider().map_err(deployment_error)?;

    let (abi, bytecode) =
        try_compile_contract("LearnToken.sol", "LearnToken").map_err(deployment_error)?;
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

    let (abi, bytecode) = try_compile_contract("LearnTokenPresigner.sol", "LearnTokenPresigner")
        .map_err(deployment_error)?;
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

    let (abi, bytecode) = try_compile_contract("PlatformImporter.sol", "PlatformImporter")
        .map_err(deployment_error)?;
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
