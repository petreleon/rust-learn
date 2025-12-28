use ethers::prelude::*;
use ethers::abi::Abi;
use ethers::core::types::Bytes;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use std::env;
use std::str::FromStr;

use crate::utils::db_utils::persistent_state::{get_persistent_state, set_persistent_state};
use super::wallet::load_wallet_from_env;
use super::provider::get_provider;
use super::compiler::compile_contract;

/// Deploys the LearnToken contract
pub async fn deploy_contract(
    wallet: Wallet<k256::ecdsa::SigningKey>,
    provider: Provider<Http>,
    abi: Abi,
    bytecode: Bytes,
    deploy_args: impl ethers::core::abi::Tokenize,
) -> Address {
    let client = std::sync::Arc::new(SignerMiddleware::new(provider, wallet));
    let factory = ContractFactory::new(abi, bytecode, client);
    let deployer = factory.deploy(deploy_args).unwrap();
    let contract = deployer.send().await.unwrap();
    contract.address()
}

pub async fn deploy_learn_token_and_save(
    conn: &mut AsyncPgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<Address> {
    let wallet = load_wallet_from_env();
    let provider = get_provider();

    let (abi, bytecode) = compile_contract("LearnToken.sol", "LearnToken");
    let addr = deploy_contract(
        wallet,
        provider,
        abi,
        bytecode,
        (name.to_string(), symbol.to_string(), decimals),
    )
    .await;

    let addr_hex = format!("{:#x}", addr);
    set_persistent_state(conn, "learn_token_address", &addr_hex).await?;

    Ok(addr)
}

pub async fn deploy_learn_token_presigner_and_save(
    conn: &mut AsyncPgConnection,
    learn_token_addr: Address,
) -> QueryResult<Address> {
    let wallet = load_wallet_from_env();
    let provider = get_provider();

    let (abi, bytecode) = compile_contract("LearnTokenPresigner.sol", "LearnTokenPresigner");
    let addr = deploy_contract(wallet, provider, abi, bytecode, (learn_token_addr,)).await;

    let addr_hex = format!("{:#x}", addr);
    set_persistent_state(conn, "learn_token_presigner_address", &addr_hex).await?;
    Ok(addr)
}

pub async fn deploy_platform_importer_and_save(
    conn: &mut AsyncPgConnection,
    treasury: Address,
) -> QueryResult<Address> {
    let wallet = load_wallet_from_env();
    let provider = get_provider();

    let (abi, bytecode) = compile_contract("PlatformImporter.sol", "PlatformImporter");
    let addr = deploy_contract(wallet, provider, abi, bytecode, (treasury,)).await;

    let addr_hex = format!("{:#x}", addr);
    set_persistent_state(conn, "platform_importer_address", &addr_hex).await?;
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
        Address::from_str(a.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)?
    } else {
        deploy_learn_token_and_save(conn, name, symbol, decimals).await?;
        let s = get_persistent_state(conn, "learn_token_address").await?.unwrap();
        Address::from_str(s.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)?
    };

    let presigner_addr = if let Some(a) = get_persistent_state(conn, "learn_token_presigner_address").await? {
        Some(Address::from_str(a.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)?)
    } else {
        let addr = deploy_learn_token_presigner_and_save(conn, token_addr).await?;
        Some(addr)
    };

    let importer_addr = if let Some(a) = get_persistent_state(conn, "platform_importer_address").await? {
        Some(Address::from_str(a.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)?)
    } else {
        let treasury_addr = if let Ok(t) = env::var("PLATFORM_TREASURY") {
            Address::from_str(t.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)?
        } else {
            let wallet = load_wallet_from_env();
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
        Ok(addr)
    } else {
        deploy_learn_token_and_save(conn, name, symbol, decimals).await
    }
}
