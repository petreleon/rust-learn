use super::*;

#[test]
fn teacher_scope_uses_teacher_fraud_authorization() {
    assert_eq!(
        authorization_for_scope(REWARD_FRAUD_BLOCK_SCOPE_TEACHER).unwrap(),
        FraudBlockAuthorization::Teacher
    );
}

#[test]
fn organization_scope_uses_organization_fraud_authorization() {
    assert_eq!(
        authorization_for_scope(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION).unwrap(),
        FraudBlockAuthorization::Organization
    );
}

#[test]
fn course_and_policy_scopes_use_general_fraud_authorization() {
    for scope in [
        REWARD_FRAUD_BLOCK_SCOPE_COURSE,
        REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY,
    ] {
        assert_eq!(
            authorization_for_scope(scope).unwrap(),
            FraudBlockAuthorization::General
        );
    }
}
