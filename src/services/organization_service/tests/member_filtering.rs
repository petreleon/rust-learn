use super::super::support::{OrganizationMemberListItem, OrganizationMemberListQuery};
use super::super::user_has_organization_dashboard_access::member_matches_query;

pub(super) fn test_member(
    name: &str,
    email: &str,
    roles: Vec<&str>,
    permissions: Vec<&str>,
) -> OrganizationMemberListItem {
    OrganizationMemberListItem {
        id: 1,
        name: name.into(),
        email: email.into(),
        email_verified: true,
        kyc_verified: false,
        joined_at: "2024-01-01T00:00:00".into(),
        roles: roles.into_iter().map(String::from).collect(),
        direct_permissions: permissions.clone().into_iter().map(String::from).collect(),
        delegated_permissions: vec![],
        effective_permissions: permissions.into_iter().map(String::from).collect(),
        direct_permission_count: 1,
        delegated_permission_count: 0,
        effective_permission_count: 1,
    }
}

pub(super) fn test_query(
    search: Option<&str>,
    role: Option<&str>,
    permission: Option<&str>,
) -> OrganizationMemberListQuery {
    OrganizationMemberListQuery {
        search: search.map(String::from),
        role: role.map(String::from),
        permission: permission.map(String::from),
        limit: 25,
        offset: 0,
    }
}

// ── member_matches_query ──

#[test]
fn matches_by_name_search() {
    let member = test_member(
        "Alice Johnson",
        "alice@example.com",
        vec!["teacher"],
        vec!["VIEW_COURSE"],
    );
    assert!(member_matches_query(
        &member,
        &test_query(Some("alice"), None, None)
    ));
    assert!(!member_matches_query(
        &member,
        &test_query(Some("bob"), None, None)
    ));
}

#[test]
fn matches_by_email_search() {
    let member = test_member("Alice", "alice@example.com", vec![], vec![]);
    assert!(member_matches_query(
        &member,
        &test_query(Some("example"), None, None)
    ));
    assert!(!member_matches_query(
        &member,
        &test_query(Some("gmail"), None, None)
    ));
}

#[test]
fn matches_by_role_search() {
    let member = test_member("Alice", "a@e.com", vec!["teacher", "admin"], vec![]);
    assert!(member_matches_query(
        &member,
        &test_query(Some("teacher"), None, None)
    ));
    assert!(member_matches_query(
        &member,
        &test_query(Some("admin"), None, None)
    ));
}

#[test]
fn matches_by_permission_search() {
    let member = test_member(
        "Alice",
        "a@e.com",
        vec![],
        vec!["VIEW_COURSE", "MANAGE_USERS"],
    );
    assert!(member_matches_query(
        &member,
        &test_query(Some("view_course"), None, None)
    ));
}
