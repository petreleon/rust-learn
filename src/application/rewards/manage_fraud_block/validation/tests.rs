use super::*;
use crate::domain::rewards::fraud_block::{
    REWARD_FRAUD_BLOCK_SCOPE_COURSE, REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION,
    REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY, REWARD_FRAUD_BLOCK_SCOPE_TEACHER,
};

fn block_command(scope: &str) -> CreateRewardFraudBlockCommand {
    CreateRewardFraudBlockCommand {
        scope_type: scope.into(),
        teacher_user_id: None,
        organization_id: None,
        course_id: None,
        reward_policy_id: None,
        reason: "Suspicious activity".into(),
        evidence_reference: None,
        expires_at: None,
    }
}

#[test]
fn normalizes_valid_teacher_block() {
    let mut command = block_command(REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    command.teacher_user_id = Some(42);
    let result = validated_draft(7, command).unwrap();
    assert_eq!(result.scope_type, "teacher");
    assert_eq!(result.teacher_user_id, Some(42));
    assert_eq!(result.created_by_user_id, 7);
}

#[test]
fn normalizes_valid_organization_block() {
    let mut command = block_command(REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION);
    command.organization_id = Some(10);
    assert_eq!(
        validated_draft(7, command).unwrap().scope_type,
        "organization"
    );
}

#[test]
fn normalizes_valid_course_block() {
    let mut command = block_command(REWARD_FRAUD_BLOCK_SCOPE_COURSE);
    command.course_id = Some(5);
    assert_eq!(validated_draft(7, command).unwrap().scope_type, "course");
}

#[test]
fn normalizes_valid_policy_block() {
    let mut command = block_command(REWARD_FRAUD_BLOCK_SCOPE_REWARD_POLICY);
    command.reward_policy_id = Some(1);
    assert_eq!(
        validated_draft(7, command).unwrap().scope_type,
        "reward_policy"
    );
}

#[test]
fn rejects_empty_reason() {
    let mut command = block_command(REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    command.teacher_user_id = Some(1);
    command.reason = "   ".into();
    assert!(validated_draft(7, command).is_err());
}

#[test]
fn rejects_zero_targets() {
    assert!(validated_draft(7, block_command(REWARD_FRAUD_BLOCK_SCOPE_TEACHER)).is_err());
}

#[test]
fn rejects_multiple_targets() {
    let mut command = block_command(REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    command.teacher_user_id = Some(1);
    command.course_id = Some(2);
    assert!(validated_draft(7, command).is_err());
}

#[test]
fn rejects_mismatched_scope_and_target() {
    let mut command = block_command(REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    command.course_id = Some(5);
    assert!(validated_draft(7, command).is_err());
}

#[test]
fn trims_reason_and_evidence() {
    let mut command = block_command(REWARD_FRAUD_BLOCK_SCOPE_TEACHER);
    command.teacher_user_id = Some(1);
    command.reason = "  reason  ".into();
    command.evidence_reference = Some("  ev  ".into());
    let result = validated_draft(7, command).unwrap();
    assert_eq!(result.reason, "reason");
    assert_eq!(result.evidence_reference, Some("ev".into()));
}
