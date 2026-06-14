use super::{normalize_provider_host_for_debug_host, normalize_provider_url_for_debug_host};

#[test]
fn rewrites_compose_provider_url_when_service_dns_is_unavailable() {
    assert_eq!(
        normalize_provider_url_for_debug_host(
            "http://anvil:8545".to_string(),
            true,
            |host, port| { host != "anvil" || port != 8545 }
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
        normalize_provider_host_for_debug_host("geth".to_string(), "8545", true, |host, port| host
            != "geth"
            || port != 8545,),
        "localhost"
    );
}

#[test]
fn leaves_non_compose_provider_host_unchanged() {
    assert_eq!(
        normalize_provider_host_for_debug_host("localhost".to_string(), "8545", true, |_, _| false),
        "localhost"
    );
}
