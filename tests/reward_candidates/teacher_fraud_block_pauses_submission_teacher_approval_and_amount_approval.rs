use crate::{
    amount_decision_helper::*, force_assign_organization_role::*, link_course_to_organization::*,
    reward_candidate_error::RewardCandidateError, submission_helper::*, support::*,
    teacher_decision_helper::*,
};

#[actix_web::test]
async fn teacher_fraud_block_pauses_submission_teacher_approval_and_amount_approval() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("BlockedRewardFlowCourse")).await;
    let teacher = create_user_helper(&mut conn, "blocked_flow_teacher").await;
    let student = create_user_helper(&mut conn, "blocked_flow_student").await;
    let second_student = create_user_helper(&mut conn, "blocked_flow_student_two").await;
    let reviewer = create_user_helper(&mut conn, "blocked_flow_reviewer").await;
    let admin = create_user_helper(&mut conn, "blocked_flow_admin").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    force_assign_course_role(&mut conn, second_student.id(), course.id, "STUDENT").await;
    force_assign_platform_role(&mut conn, reviewer.id(), "MODERATOR").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    create_active_course_reward_policy(&mut conn, course.id, REWARD_EVENT_COURSE_COMPLETION).await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("blocked_flow_candidate")),
    )
    .await
    .expect("teacher should submit before fraud block is active");

    let teacher_block = create_reward_fraud_block(
        &mut conn,
        admin.id(),
        teacher_fraud_block_request(teacher.id()),
    )
    .await
    .expect("admin should block teacher reward activity");

    let denied_submission = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(
            second_student.id(),
            &unique_string("blocked_flow_submission"),
        ),
    )
    .await
    .expect_err("active teacher block should prevent new reward submission");
    assert!(matches!(
        denied_submission,
        RewardCandidateError::InvalidStatus(message)
            if message.contains("teacher reward activity is blocked")
    ));

    let denied_teacher_approval = decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("student completed the course".to_string()),
        },
    )
    .await
    .expect_err("active teacher block should prevent teacher approval");
    assert!(matches!(
        denied_teacher_approval,
        RewardCandidateError::InvalidStatus(message)
            if message.contains("teacher reward activity is blocked")
    ));
    let still_pending = find_candidate(&mut conn, candidate.id)
        .await
        .expect("candidate should remain readable after blocked teacher decision");
    assert_eq!(still_pending.status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);
    assert!(still_pending.teacher_approver_user_id.is_none());

    revoke_reward_fraud_block(&mut conn, admin.id(), teacher_block.id)
        .await
        .expect("admin should revoke teacher fraud block");

    let teacher_approved = decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("student completed the course".to_string()),
        },
    )
    .await
    .expect("teacher approval should resume after revocation");
    assert_eq!(teacher_approved.status, REWARD_STATUS_TEACHER_APPROVED);

    create_reward_fraud_block(
        &mut conn,
        admin.id(),
        teacher_fraud_block_request(teacher.id()),
    )
    .await
    .expect("admin should create a new active teacher fraud block");

    let denied_amount = decide_reward_amount(
        &mut conn,
        reviewer.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(10)),
            decision_reason: Some("amount approved by central reviewer".to_string()),
        },
    )
    .await
    .expect_err("active teacher block should prevent amount approval");
    assert!(matches!(
        denied_amount,
        RewardCandidateError::InvalidStatus(message)
            if message.contains("teacher reward activity is blocked")
    ));
    let still_teacher_approved = find_candidate(&mut conn, candidate.id)
        .await
        .expect("candidate should remain readable after blocked amount decision");
    assert_eq!(
        still_teacher_approved.status,
        REWARD_STATUS_TEACHER_APPROVED
    );
    assert!(still_teacher_approved.amount_reviewer_user_id.is_none());
    assert!(still_teacher_approved.approved_amount.is_none());

    let execution_job = find_job_by_candidate(&mut conn, candidate.id)
        .await
        .expect("execution job lookup should succeed");
    assert!(
        execution_job.is_none(),
        "fraud block must not enqueue reward execution by deciding a candidate"
    );
}
