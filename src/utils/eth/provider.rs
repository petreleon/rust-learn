use ethers::prelude::*;
use std::convert::TryFrom;

fn provider_url_from_env() -> String {
    std::env::var("ETH_RPC_URL").ok().unwrap_or_else(|| {
        let host = std::env::var("ETH_HOST").unwrap_or_else(|_| "geth".to_string());
        let port = std::env::var("ETH_PORT").unwrap_or_else(|_| "8545".to_string());
        format!("http://{}:{}", host, port)
    })
}

pub fn try_get_provider() -> Result<Provider<Http>, String> {
    dotenvy::dotenv().ok();
    let url = provider_url_from_env();
    Provider::<Http>::try_from(url.clone())
        .map_err(|err| format!("Could not create provider from {url}: {err}"))
}
