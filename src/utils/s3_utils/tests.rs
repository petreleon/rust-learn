use super::normalize_internal_s3_host_for_debug;

#[test]
fn leaves_non_compose_s3_hosts_unchanged() {
    assert_eq!(
        normalize_internal_s3_host_for_debug("localhost".to_string(), "9000", true, |_, _| {
            false
        }),
        "localhost"
    );
}

#[test]
fn rewrites_compose_s3_host_when_service_dns_is_unavailable() {
    assert_eq!(
        normalize_internal_s3_host_for_debug("rustfs".to_string(), "9000", true, |host, port| host
            != "rustfs"
            || port != 9000,),
        "localhost"
    );
}

#[test]
fn leaves_compose_s3_host_when_service_dns_is_available() {
    assert_eq!(
        normalize_internal_s3_host_for_debug("rustfs".to_string(), "9000", true, |_, _| true),
        "rustfs"
    );
}

#[test]
fn leaves_compose_s3_host_in_release_builds() {
    assert_eq!(
        normalize_internal_s3_host_for_debug("rustfs".to_string(), "9000", false, |_, _| { false }),
        "rustfs"
    );
}
