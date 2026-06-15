use crate::{
    amount_decision_helper::*, force_assign_organization_role::*, link_course_to_organization::*,
    reward_candidate_error::RewardCandidateError, submission_helper::*, support::*,
    teacher_decision_helper::*,
};

#[actix_web::test]
async fn fraud_block_permission_cannot_approve_candidates_or_amounts() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("FraudPermissionBoundaryCourse")).await;
    let teacher = create_user_helper(&mut conn, "fraud_permission_teacher").await;
    let student = create_user_helper(&mut conn, "fraud_permission_student").await;
    let fraud_operator = create_user_helper(&mut conn, "fraud_permission_operator").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    let fraud_role_id = create_custom_platform_role(
        &mut conn,
        &unique_string("FRAUD_BLOCK_ONLY_OPERATOR"),
        &[Permissions::MANAGE_REWARD_FRAUD_BLOCKS],
    )
    .await;
    assign_platform_role_id(&mut conn, fraud_operator.id(), fraud_role_id).await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let block = create_reward_fraud_block(
        &mut conn,
        fraud_operator.id(),
        teacher_fraud_block_request(teacher.id()),
    )
    .await
    .expect("fraud-block permission should create teacher reward block");
    revoke_reward_fraud_block(&mut conn, fraud_operator.id(), block.id)
        .await
        .expect("fraud-block permission should revoke teacher reward block");

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("fraud_permission_candidate")),
    )
    .await
    .expect("teacher should submit after fraud block revocation");

    let denied_candidate_approval = decide_reward_candidate_by_teacher(
        &mut conn,
        fraud_operator.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("fraud operator attempted candidate approval".to_string()),
        },
    )
    .await
    .expect_err("fraud-block permission must not approve reward candidates");
    assert!(matches!(
        denied_candidate_approval,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string()
    ));

    let denied_candidate_rejection = decide_reward_candidate_by_teacher(
        &mut conn,
        fraud_operator.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "rejected".to_string(),
            decision_reason: Some("fraud operator attempted candidate rejection".to_string()),
        },
    )
    .await
    .expect_err("fraud-block permission must not reject reward candidates");
    assert!(matches!(
        denied_candidate_rejection,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string()
    ));

    decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("teacher confirmed reward candidate".to_string()),
        },
    )
    .await
    .expect("teacher should approve candidate");

    let denied_amount = decide_reward_amount(
        &mut conn,
        fraud_operator.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(15)),
            decision_reason: Some("fraud operator attempted amount approval".to_string()),
        },
    )
    .await
    .expect_err("fraud-block permission must not set payout amount");
    assert!(matches!(
        denied_amount,
        RewardCandidateError::PermissionDenied(permission)
            if permission == Permissions::APPROVE_REWARD_AMOUNT.to_string()
    ));

    let execution_job = find_job_by_candidate(&mut conn, candidate.id)
        .await
        .expect("execution job lookup should succeed");
    assert!(
        execution_job.is_none(),
        "fraud-block permission must not enqueue reward execution"
    );
}
