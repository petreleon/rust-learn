use super::*;

#[test]
fn normalizes_scope_types() {
    assert_eq!(
        normalize_scope_type(DELEGATED_SCOPE_PLATFORM).unwrap(),
        "platform"
    );
    assert_eq!(
        normalize_scope_type(DELEGATED_SCOPE_ORGANIZATION).unwrap(),
        "organization"
    );
    assert_eq!(normalize_scope_type("  COURSE  ").unwrap(), "course");
    assert!(normalize_scope_type("unknown").is_err());
}

#[test]
fn validates_platform_permission_scope() {
    let scope = normalize_scope_ids(
        "APPROVE_REWARD_AMOUNT",
        DELEGATED_SCOPE_PLATFORM,
        None,
        None,
    )
    .unwrap();
    assert_eq!(scope.scope_type, DELEGATED_SCOPE_PLATFORM);
    assert_eq!(scope.delegated_scope, DelegatedScope::platform());

    assert!(normalize_scope_ids(
        "SUBMIT_COURSE_REWARD_EVENT",
        DELEGATED_SCOPE_PLATFORM,
        None,
        None,
    )
    .is_err());
}

#[test]
fn validates_organization_permission_scope() {
    let scope = normalize_scope_ids(
        "SUBMIT_ORG_COURSE_REWARD_EVENT",
        DELEGATED_SCOPE_ORGANIZATION,
        Some(9),
        None,
    )
    .unwrap();
    assert_eq!(scope.organization_id, Some(9));
    assert_eq!(scope.delegated_scope, DelegatedScope::organization(9));

    assert!(normalize_scope_ids(
        "APPROVE_REWARD_AMOUNT",
        DELEGATED_SCOPE_ORGANIZATION,
        Some(9),
        None,
    )
    .is_err());
}

#[test]
fn validates_course_permission_scope() {
    let scope = normalize_scope_ids(
        "APPROVE_STUDENT_REWARD_CANDIDATE",
        DELEGATED_SCOPE_COURSE,
        None,
        Some(7),
    )
    .unwrap();
    assert_eq!(scope.course_id, Some(7));
    assert_eq!(scope.delegated_scope, DelegatedScope::course(7));

    assert!(normalize_scope_ids(
        "VIEW_ORG_REWARD_REPORTS",
        DELEGATED_SCOPE_COURSE,
        None,
        Some(7),
    )
    .is_err());
}

#[test]
fn rejects_non_reward_delegation_permission() {
    assert!(normalize_permission("MANAGE_COURSE_SETTINGS").is_err());
    assert!(normalize_permission("").is_err());
}
