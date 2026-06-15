use crate::{delegation_helper::*, force_assign_course_role::*, submission_helper::*, support::*};

#[actix_web::test]
async fn delegated_permission_grant_requires_platform_delegate_permission() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("DelegatedDeniedCourse")).await;
    let grantor = create_user_helper(&mut conn, "delegating_denied_grantor").await;
    let operator = create_user_helper(&mut conn, "delegating_denied_operator").await;

    let denied = grant_delegated_permission(
        &mut conn,
        grantor.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::SUBMIT_COURSE_REWARD_EVENT.to_string(),
            scope_type: DELEGATED_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            reason: Some("should not grant without platform permission".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect_err("user without delegation authority must be denied");

    assert!(matches!(
        denied,
        DelegatedPermissionError::PermissionDenied(permission)
            if permission == Permissions::DELEGATE_REWARD_APPROVAL.to_string()
    ));
}

#[actix_web::test]
async fn delegated_organization_permission_submits_for_attached_course_only() {
    let mut conn = setup_conn().await;
    let organization = create_organization(&mut conn, &unique_string("DelegatedOrg")).await;
    let other_organization =
        create_organization(&mut conn, &unique_string("DelegatedOtherOrg")).await;
    let course = create_course(&mut conn, &unique_string("DelegatedOrgCourse")).await;
    let admin = create_user_helper(&mut conn, "delegating_org_admin").await;
    let operator = create_user_helper(&mut conn, "delegated_org_operator").await;
    let student = create_user_helper(&mut conn, "delegated_org_student").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    link_course_to_organization(&mut conn, course.id, organization.id).await;
    create_active_course_reward_policy(&mut conn, course.id).await;

    grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT.to_string(),
            scope_type: DELEGATED_SCOPE_ORGANIZATION.to_string(),
            organization_id: Some(organization.id),
            course_id: None,
            reason: Some("central org reward operations".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should delegate organization reward submission");

    assert!(has_organization_permission(
        &mut conn,
        operator.id(),
        organization.id,
        &Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT.to_string(),
    )
    .await
    .expect("organization permission lookup should succeed"));
    assert!(!has_organization_permission(
        &mut conn,
        operator.id(),
        other_organization.id,
        &Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT.to_string(),
    )
    .await
    .expect("other organization permission lookup should succeed"));

    let candidate = submit_organization_reward_candidate(
        &mut conn,
        operator.id(),
        organization.id,
        course.id,
        reward_request(student.id(), &unique_string("delegated_org_submit")),
    )
    .await
    .expect("delegated organization operator should submit attached course reward");
    assert_eq!(candidate.status, REWARD_STATUS_PENDING_TEACHER_APPROVAL);
}
