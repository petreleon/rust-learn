use super::*;

#[test]
fn teacher_scope_uses_teacher_fraud_authorization() {
    assert_eq!(
        authorization_for_scope(RewardFraudBlockScope::Teacher),
        FraudBlockAuthorization::Teacher
    );
}

#[test]
fn organization_scope_uses_organization_fraud_authorization() {
    assert_eq!(
        authorization_for_scope(RewardFraudBlockScope::Organization),
        FraudBlockAuthorization::Organization
    );
}

#[test]
fn course_and_policy_scopes_use_general_fraud_authorization() {
    for scope in [
        RewardFraudBlockScope::Course,
        RewardFraudBlockScope::RewardPolicy,
    ] {
        assert_eq!(
            authorization_for_scope(scope),
            FraudBlockAuthorization::General
        );
    }
}
