// ── normalize_idempotency_key ──

#[test]
fn trims_valid_key() {
    assert_eq!(
        normalize_idempotency_key(Some("  key  ".into())).unwrap(),
        Some("key".into())
    );
}

#[test]
fn rejects_blank_key() {
    assert!(normalize_idempotency_key(Some("   ".into())).is_err());
    assert!(normalize_idempotency_key(Some("".into())).is_err());
}

#[test]
fn none_is_ok() {
    assert_eq!(normalize_idempotency_key(None).unwrap(), None);
}

// ── clean_portfolio_links ──

#[test]
fn trims_and_filters_empty() {
    let links = clean_portfolio_links(Some(vec![
        "  https://a.com  ".into(),
        "   ".into(),
        "https://b.com".into(),
    ]));
    assert_eq!(links, vec!["https://a.com", "https://b.com"]);
}

#[test]
fn empty_input_returns_empty() {
    assert!(clean_portfolio_links(None).is_empty());
    assert!(clean_portfolio_links(Some(vec![])).is_empty());
    assert!(clean_portfolio_links(Some(vec!["  ".into()])).is_empty());
}

// ── portfolio_links_from_json ──

#[test]
fn extracts_strings_from_json_array() {
    let links = portfolio_links_from_json(&json!(["  a  ", " b ", "  "]));
    assert_eq!(links, vec!["a", "b"]);
}

#[test]
fn non_array_json_returns_empty() {
    assert!(portfolio_links_from_json(&json!("not an array")).is_empty());
    assert!(portfolio_links_from_json(&json!(null)).is_empty());
    assert!(portfolio_links_from_json(&json!(123)).is_empty());
}

#[test]
fn non_string_items_skipped() {
    let links = portfolio_links_from_json(&json!(["a", 123, null, " b "]));
    assert_eq!(links, vec!["a", "b"]);
}

// ── validate_requested_scope ──

#[test]
fn platform_scope_always_valid() {
    validate_requested_scope(TEACHER_APPLICATION_SCOPE_PLATFORM, None, None, None).unwrap();
}

#[test]
fn organization_scope_requires_org_or_sponsor() {
    assert!(
        validate_requested_scope(TEACHER_APPLICATION_SCOPE_ORGANIZATION, Some(1), None, None)
            .is_ok()
    );
    assert!(
        validate_requested_scope(TEACHER_APPLICATION_SCOPE_ORGANIZATION, None, None, Some(2))
            .is_ok()
    );
    assert!(
        validate_requested_scope(TEACHER_APPLICATION_SCOPE_ORGANIZATION, None, None, None).is_err()
    );
}
#[test]
fn course_scope_requires_course_id() {
    assert!(
        validate_requested_scope(TEACHER_APPLICATION_SCOPE_COURSE, None, Some(1), None).is_ok()
    );
    assert!(validate_requested_scope(TEACHER_APPLICATION_SCOPE_COURSE, None, None, None).is_err());
}

// ── build_new_application ──

fn valid_request() -> SubmitTeacherApplicationRequest {
    SubmitTeacherApplicationRequest {
        requested_scope: "platform".into(),
        requested_organization_id: None,
        requested_course_id: None,
        experience_summary: "I have taught for 5 years".into(),
        organization_sponsor_id: None,
        portfolio_links: Some(vec!["https://port.link".into()]),
        idempotency_key: Some("my-key".into()),
    }
}
