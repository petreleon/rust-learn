use ethers::prelude::*;
use std::convert::TryFrom;
use std::net::ToSocketAddrs;

fn provider_url_from_env() -> String {
    std::env::var("ETH_RPC_URL")
        .ok()
        .map(|url| {
            normalize_provider_url_for_debug_host(url, cfg!(debug_assertions), host_port_resolves)
        })
        .unwrap_or_else(|| {
            let host = std::env::var("ETH_HOST").unwrap_or_else(|_| "geth".to_string());
            let port = std::env::var("ETH_PORT").unwrap_or_else(|_| "8545".to_string());
            let host = normalize_provider_host_for_debug_host(
                host,
                &port,
                cfg!(debug_assertions),
                host_port_resolves,
            );
            format!("http://{}:{}", host, port)
        })
}

fn normalize_provider_url_for_debug_host<F>(
    url: String,
    is_debug_build: bool,
    host_resolves: F,
) -> String
where
    F: Fn(&str, u16) -> bool,
{
    if !is_debug_build {
        return url;
    }

    let Ok(uri) = url.parse::<http::Uri>() else {
        return url;
    };
    let Some(authority) = uri.authority() else {
        return url;
    };
    let host = authority.host();
    if !is_compose_eth_host(host) {
        return url;
    }

    let port = authority.port_u16().unwrap_or(8545);
    if host_resolves(host, port) {
        return url;
    }

    let scheme = uri.scheme_str().unwrap_or("http");
    let port_suffix = authority
        .port_u16()
        .map(|port| format!(":{port}"))
        .unwrap_or_default();
    let path_and_query = uri
        .path_and_query()
        .map(|path_and_query| path_and_query.as_str())
        .unwrap_or("");
    let original_origin = format!("{scheme}://{}", authority.as_str());
    let path_and_query = if path_and_query == "/" && url == original_origin {
        ""
    } else {
        path_and_query
    };

    format!("{scheme}://localhost{port_suffix}{path_and_query}")
}

fn normalize_provider_host_for_debug_host<F>(
    host: String,
    port: &str,
    is_debug_build: bool,
    host_resolves: F,
) -> String
where
    F: Fn(&str, u16) -> bool,
{
    if !is_debug_build || !is_compose_eth_host(&host) {
        return host;
    }

    let Ok(port) = port.parse::<u16>() else {
        return host;
    };

    if host_resolves(&host, port) {
        host
    } else {
        "localhost".to_string()
    }
}

fn is_compose_eth_host(host: &str) -> bool {
    matches!(host, "anvil" | "geth")
}

fn host_port_resolves(host: &str, port: u16) -> bool {
    (host, port)
        .to_socket_addrs()
        .map(|mut addresses| addresses.next().is_some())
        .unwrap_or(false)
}

pub fn try_get_provider() -> Result<Provider<Http>, String> {
    dotenvy::dotenv().ok();
    let url = provider_url_from_env();
    Provider::<Http>::try_from(url.clone())
        .map_err(|err| format!("Could not create provider from {url}: {err}"))
}

#[cfg(test)]
mod tests {
    use super::{normalize_provider_host_for_debug_host, normalize_provider_url_for_debug_host};

    #[test]
    fn rewrites_compose_provider_url_when_service_dns_is_unavailable() {
        assert_eq!(
            normalize_provider_url_for_debug_host(
                "http://anvil:8545".to_string(),
                true,
                |host, port| host != "anvil" || port != 8545,
            ),
            "http://localhost:8545"
        );
    }

    #[test]
    fn preserves_provider_url_path_when_rewriting_host() {
        assert_eq!(
            normalize_provider_url_for_debug_host(
                "http://geth:8545/rpc?tenant=dev".to_string(),
                true,
                |host, port| host != "geth" || port != 8545,
            ),
            "http://localhost:8545/rpc?tenant=dev"
        );
    }

    #[test]
    fn leaves_compose_provider_url_when_service_dns_is_available() {
        let url = "http://anvil:8545".to_string();

        assert_eq!(
            normalize_provider_url_for_debug_host(url.clone(), true, |_, _| true),
            url
        );
    }

    #[test]
    fn leaves_provider_url_in_release_builds() {
        let url = "http://anvil:8545".to_string();

        assert_eq!(
            normalize_provider_url_for_debug_host(url.clone(), false, |_, _| false),
            url
        );
    }

    #[test]
    fn rewrites_compose_provider_host_when_service_dns_is_unavailable() {
        assert_eq!(
            normalize_provider_host_for_debug_host(
                "geth".to_string(),
                "8545",
                true,
                |host, port| host != "geth" || port != 8545,
            ),
            "localhost"
        );
    }

    #[test]
    fn leaves_non_compose_provider_host_unchanged() {
        assert_eq!(
            normalize_provider_host_for_debug_host(
                "localhost".to_string(),
                "8545",
                true,
                |_, _| false,
            ),
            "localhost"
        );
    }
}
