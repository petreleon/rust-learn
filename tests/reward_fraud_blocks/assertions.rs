use crate::support::*;

pub(crate) async fn assert_active_teacher_block_listed(
    fraud_blocks: &PostgresRewardFraudBlockUseCase,
    actor_user_id: i32,
    teacher_user_id: i32,
    block_id: i64,
) {
    let active_teacher_blocks = fraud_blocks
        .list_reward_fraud_blocks(
            actor_user_id,
            ListRewardFraudBlocksQuery {
                scope_type: Some(REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string()),
                teacher_user_id: Some(teacher_user_id),
                active: Some(true),
                ..Default::default()
            },
        )
        .await
        .expect("admin should list active teacher fraud blocks");
    assert_eq!(active_teacher_blocks.total, 1);
    assert_eq!(active_teacher_blocks.blocks[0].id, block_id);
}

pub(crate) fn teacher_block_request(teacher_user_id: i32) -> CreateRewardFraudBlockCommand {
    CreateRewardFraudBlockCommand {
        scope_type: REWARD_FRAUD_BLOCK_SCOPE_TEACHER.to_string(),
        teacher_user_id: Some(teacher_user_id),
        organization_id: None,
        course_id: None,
        reward_policy_id: None,
        reason: "suspicious reward approvals".to_string(),
        evidence_reference: Some("case://teacher-block".to_string()),
        expires_at: None,
    }
}

pub(crate) fn organization_block_request(organization_id: i32) -> CreateRewardFraudBlockCommand {
    CreateRewardFraudBlockCommand {
        scope_type: REWARD_FRAUD_BLOCK_SCOPE_ORGANIZATION.to_string(),
        teacher_user_id: None,
        organization_id: Some(organization_id),
        course_id: None,
        reward_policy_id: None,
        reason: "organization reward submissions paused".to_string(),
        evidence_reference: None,
        expires_at: None,
    }
}
