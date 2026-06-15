use crate::{
    delegation_helper::*, force_assign_course_role::*,
    reward_candidate_error::RewardCandidateError, submission_helper::*, support::*,
};
use rust_learn::domain::rewards::candidate::status::RewardCandidateStatus;

#[actix_web::test]
async fn delegated_course_permission_submits_candidate_without_course_role() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("DelegatedCourse")).await;
    let other_course = create_course(&mut conn, &unique_string("DelegatedOtherCourse")).await;
    let admin = create_user_helper(&mut conn, "delegating_admin").await;
    let operator = create_user_helper(&mut conn, "delegated_operator").await;
    let student = create_user_helper(&mut conn, "delegated_student").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    create_active_course_reward_policy(&mut conn, course.id).await;

    let denied = submit_course_reward_candidate(
        &mut conn,
        operator.id(),
        course.id,
        reward_request(student.id(), &unique_string("delegated_denied")),
    )
    .await
    .expect_err("operator without role or delegation must be denied");
    assert!(matches!(denied, RewardCandidateError::PermissionDenied(_)));

    let delegation = grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string(),
            scope_type: DELEGATED_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            reason: Some("central office reward operations".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should delegate course reward submission");
    let replayed_delegation = grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string(),
            scope_type: DELEGATED_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            reason: Some("retry after client timeout".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("duplicate active delegated permission grant should be idempotent");
    assert_eq!(replayed_delegation.id, delegation.id);

    let active_delegation_count: i64 = delegated_permissions::table
        .filter(delegated_permissions::grantee_user_id.eq(operator.id()))
        .filter(
            delegated_permissions::permission
                .eq(Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string()),
        )
        .filter(delegated_permissions::scope_type.eq(DELEGATED_SCOPE_COURSE))
        .filter(delegated_permissions::course_id.eq(Some(course.id)))
        .filter(delegated_permissions::revoked_at.is_null())
        .count()
        .get_result(&mut conn)
        .await
        .expect("delegated permissions should be countable");
    assert_eq!(active_delegation_count, 1);

    assert!(has_course_permission(
        &mut conn,
        operator.id(),
        course.id,
        &Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string(),
    )
    .await
    .expect("course permission lookup should succeed"));
    assert!(!has_course_permission(
        &mut conn,
        operator.id(),
        other_course.id,
        &Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string(),
    )
    .await
    .expect("other course permission lookup should succeed"));

    let candidate = submit_course_reward_candidate(
        &mut conn,
        operator.id(),
        course.id,
        reward_request(student.id(), &unique_string("delegated_course_submit")),
    )
    .await
    .expect("delegated operator should submit candidate");
    assert_eq!(
        candidate.status,
        RewardCandidateStatus::PendingTeacherApproval
    );
}
