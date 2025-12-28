use ethers::prelude::*;
use std::convert::TryFrom;

/// Connects to an Ethereum node using env configuration
pub fn get_provider() -> Provider<Http> {
    dotenvy::dotenv().ok();
    let url = std::env::var("ETH_RPC_URL").ok().unwrap_or_else(|| {
        let host = std::env::var("ETH_HOST").unwrap_or_else(|_| "geth".to_string());
        let port = std::env::var("ETH_PORT").unwrap_or_else(|_| "8545".to_string());
        format!("http://{}:{}", host, port)
    });
    Provider::<Http>::try_from(url).expect("Could not create provider from ETH_RPC_URL/ETH_HOST/ETH_PORT")
}
