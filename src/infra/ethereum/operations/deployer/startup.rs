use super::{
    deploy_learn_token_and_save, deploy_learn_token_presigner_and_save,
    deploy_platform_importer_and_save, deployment_error,
};
use crate::infra::ethereum::operations::wallet::try_load_wallet_from_env;
use crate::infra::postgres::operations::persistent_state::get_persistent_state;
use diesel::QueryResult;
use diesel_async::AsyncPgConnection;
use ethers::prelude::*;
use std::env;
use std::str::FromStr;

pub async fn get_learn_token_address(conn: &mut AsyncPgConnection) -> QueryResult<Option<Address>> {
    parse_saved_address(conn, "learn_token_address").await
}

pub async fn deploy_all_startup(
    conn: &mut AsyncPgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<(Address, Option<Address>, Option<Address>)> {
    let token_addr = resolve_learn_token(conn, name, symbol, decimals).await?;
    let presigner_addr = resolve_presigner(conn, token_addr).await?;
    let importer_addr = resolve_importer(conn).await?;

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

async fn resolve_learn_token(
    conn: &mut AsyncPgConnection,
    name: &str,
    symbol: &str,
    decimals: u8,
) -> QueryResult<Address> {
    if let Some(addr) = parse_saved_address(conn, "learn_token_address").await? {
        return Ok(addr);
    }

    deploy_learn_token_and_save(conn, name, symbol, decimals).await?;
    parse_saved_address(conn, "learn_token_address")
        .await?
        .ok_or_else(|| deployment_error("learn_token_address missing after deployment"))
}

async fn resolve_presigner(
    conn: &mut AsyncPgConnection,
    token_addr: Address,
) -> QueryResult<Option<Address>> {
    if let Some(addr) = parse_saved_address(conn, "learn_token_presigner_address").await? {
        return Ok(Some(addr));
    }

    deploy_learn_token_presigner_and_save(conn, token_addr)
        .await
        .map(Some)
}

async fn resolve_importer(conn: &mut AsyncPgConnection) -> QueryResult<Option<Address>> {
    if let Some(addr) = parse_saved_address(conn, "platform_importer_address").await? {
        return Ok(Some(addr));
    }

    let treasury_addr = treasury_address()?;
    deploy_platform_importer_and_save(conn, treasury_addr)
        .await
        .map(Some)
}

fn treasury_address() -> QueryResult<Address> {
    if let Ok(treasury) = env::var("PLATFORM_TREASURY") {
        return parse_address(&treasury);
    }

    try_load_wallet_from_env()
        .map(|wallet| wallet.address())
        .map_err(deployment_error)
}

async fn parse_saved_address(
    conn: &mut AsyncPgConnection,
    key: &str,
) -> QueryResult<Option<Address>> {
    get_persistent_state(conn, key)
        .await?
        .map(|address| parse_address(&address))
        .transpose()
}

fn parse_address(address: &str) -> QueryResult<Address> {
    Address::from_str(address.trim_start_matches("0x")).map_err(|_| diesel::result::Error::NotFound)
}
