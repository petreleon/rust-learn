#[test]
fn search_is_case_insensitive() {
    let member = test_member(
        "ALICE",
        "ALICE@EXAMPLE.COM",
        vec!["TEACHER"],
        vec!["VIEW_COURSE"],
    );
    assert!(member_matches_query(
        &member,
        &test_query(Some("alice"), None, None)
    ));
}

#[test]
fn matches_by_exact_role_filter() {
    let member = test_member("Alice", "a@e.com", vec!["teacher"], vec![]);
    assert!(member_matches_query(
        &member,
        &test_query(None, Some("teacher"), None)
    ));
    assert!(!member_matches_query(
        &member,
        &test_query(None, Some("admin"), None)
    ));
}

#[test]
fn role_filter_is_case_insensitive() {
    let member = test_member("Alice", "a@e.com", vec!["TEACHER"], vec![]);
    assert!(member_matches_query(
        &member,
        &test_query(None, Some("teacher"), None)
    ));
}

#[test]
fn matches_by_exact_permission_filter() {
    let member = test_member("Alice", "a@e.com", vec![], vec!["VIEW_COURSE"]);
    assert!(member_matches_query(
        &member,
        &test_query(None, None, Some("VIEW_COURSE"))
    ));
    assert!(!member_matches_query(
        &member,
        &test_query(None, None, Some("MANAGE_USERS"))
    ));
}

#[test]
fn permission_filter_is_case_insensitive() {
    let member = test_member("Alice", "a@e.com", vec![], vec!["VIEW_COURSE"]);
    assert!(member_matches_query(
        &member,
        &test_query(None, None, Some("view_course"))
    ));
}

#[test]
fn all_filters_null_always_matches() {
    let member = test_member("Alice", "a@e.com", vec![], vec![]);
    assert!(member_matches_query(&member, &test_query(None, None, None)));
}

#[test]
fn combined_filters_all_must_match() {
    let member = test_member("Alice", "alice@e.com", vec!["teacher"], vec!["VIEW_COURSE"]);
    assert!(member_matches_query(
        &member,
        &test_query(Some("alice"), Some("teacher"), Some("VIEW_COURSE"))
    ));
    assert!(!member_matches_query(
        &member,
        &test_query(Some("bob"), Some("teacher"), Some("VIEW_COURSE"))
    ));
    assert!(!member_matches_query(
        &member,
        &test_query(Some("alice"), Some("admin"), Some("VIEW_COURSE"))
    ));
}

// ── normalize_query_value ──

#[test]
fn returns_trimmed_value() {
    assert_eq!(
        normalize_query_value(Some("  hello  ".into())),
        Some("hello".into())
    );
}

#[test]
fn returns_none_for_empty_or_whitespace() {
    assert_eq!(normalize_query_value(None), None);
    assert_eq!(normalize_query_value(Some("".into())), None);
    assert_eq!(normalize_query_value(Some("   ".into())), None);
}

// ── sorted_vec ──

#[test]
fn sorts_and_collects() {
    let mut set = BTreeSet::new();
    set.insert("c".into());
    set.insert("a".into());
    set.insert("b".into());
    assert_eq!(sorted_vec(set), vec!["a", "b", "c"]);
}
