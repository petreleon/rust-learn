use super::*;

#[test]
fn teacher_scope_requires_two_permissions() {
    let permissions = required_permissions_for_scope(REWARD_FRAUD_BLOCK_SCOPE_TEACHER).unwrap();
    assert_eq!(permissions.len(), 2);
    assert!(permissions.contains(&Permissions::BLOCK_REWARD_TEACHER));
}

#[test]
fn organization_scope_requires_two_permissions() {
    let permissions =
        required_permissions_for_scope(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION).unwrap();
    assert_eq!(permissions.len(), 2);
    assert!(permissions.contains(&Permissions::BLOCK_REWARD_ORGANIZATION));
}

#[test]
fn course_and_policy_scopes_require_one_permission() {
    for scope in [
        REWARD_FRAUD_BLOCK_SCOPE_COURSE,
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY,
    ] {
        let permissions = required_permissions_for_scope(scope).unwrap();
        assert_eq!(permissions.len(), 1);
        assert_eq!(permissions[0], Permissions::MANAGE_REWARD_FRAUD_BLOCKS);
    }
}
