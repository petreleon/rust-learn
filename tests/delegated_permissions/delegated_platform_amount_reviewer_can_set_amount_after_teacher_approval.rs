use crate::{
    delegation_helper::*, force_assign_course_role::*,
    reward_candidate_error::RewardCandidateError, submission_helper::*, support::*,
    teacher_decision_helper::*,
};
use rust_learn::domain::rewards::candidate::status::RewardCandidateStatus;

#[actix_web::test]
async fn delegated_platform_amount_reviewer_can_set_amount_after_teacher_approval() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("DelegatedAmountCourse")).await;
    let admin = create_user_helper(&mut conn, "delegating_amount_admin").await;
    let teacher = create_user_helper(&mut conn, "delegated_amount_teacher").await;
    let operator = create_user_helper(&mut conn, "delegated_amount_operator").await;
    let student = create_user_helper(&mut conn, "delegated_amount_student").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    create_active_course_reward_policy(&mut conn, course.id).await;

    let candidate = submit_course_reward_candidate(
        &mut conn,
        teacher.id(),
        course.id,
        reward_request(student.id(), &unique_string("delegated_amount_candidate")),
    )
    .await
    .expect("teacher should submit candidate");
    let approved = decide_reward_candidate_by_teacher(
        &mut conn,
        teacher.id(),
        course.id,
        candidate.id,
        TeacherRewardCandidateDecisionRequest {
            status: "approved".to_string(),
            decision_reason: None,
        },
    )
    .await
    .expect("teacher should approve candidate");
    assert_eq!(approved.status, RewardCandidateStatus::TeacherApproved);

    let denied = decide_reward_amount(
        &mut conn,
        operator.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(10)),
            decision_reason: None,
        },
    )
    .await
    .expect_err("operator without platform delegation must be denied");
    assert!(matches!(denied, RewardCandidateError::PermissionDenied(_)));

    grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::APPROVE_REWARD_AMOUNT.to_string(),
            scope_type: DELEGATED_SCOPE_PLATFORM.to_string(),
            organization_id: None,
            course_id: None,
            reason: Some("central reward amount review".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should delegate platform amount review");

    assert!(has_platform_permission(
        &mut conn,
        operator.id(),
        &Permissions::APPROVE_REWARD_AMOUNT.to_string(),
    )
    .await
    .expect("platform permission lookup should succeed"));

    let amount_approved = decide_reward_amount(
        &mut conn,
        operator.id(),
        candidate.id,
        RewardAmountDecisionRequest {
            status: "approved".to_string(),
            approved_amount: Some(BigDecimal::from(10)),
            decision_reason: None,
        },
    )
    .await
    .expect("delegated platform reviewer should approve amount");
    assert_eq!(
        amount_approved.status,
        RewardCandidateStatus::AmountApproved
    );
}
