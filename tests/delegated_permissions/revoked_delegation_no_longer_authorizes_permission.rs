#[actix_web::test]
async fn revoked_delegation_no_longer_authorizes_permission() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("DelegatedRevokeCourse")).await;
    let admin = create_user_helper(&mut conn, "delegating_revoke_admin").await;
    let operator = create_user_helper(&mut conn, "delegated_revoke_operator").await;
    force_assign_platform_role(&mut conn, admin.id(), "ADMIN").await;

    let delegation = grant_delegated_permission(
        &mut conn,
        admin.id(),
        GrantDelegatedPermissionRequest {
            grantee_user_id: operator.id(),
            permission: Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string(),
            scope_type: DELEGATED_SCOPE_COURSE.to_string(),
            organization_id: None,
            course_id: Some(course.id),
            reason: Some("temporary course approval coverage".to_string()),
            expires_at: None,
        },
    )
    .await
    .expect("admin should delegate course approval");

    assert!(user_permission_course_request(
        &mut conn,
        operator.id(),
        course.id,
        &Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string(),
    )
    .await
    .expect("course permission lookup should succeed"));

    revoke_delegated_permission(
        &mut conn,
        admin.id(),
        delegation.id,
        Some("coverage ended".to_string()),
    )
    .await
    .expect("admin should revoke delegated permission");

    assert!(!user_permission_course_request(
        &mut conn,
        operator.id(),
        course.id,
        &Permissions::APPROVE_STUDENT_REWARD_CANDIDATE.to_string(),
    )
    .await
    .expect("course permission lookup after revoke should succeed"));

    let stored = find_delegated_permission(&mut conn, delegation.id)
        .await
        .expect("delegation record should remain after revoke");
    assert_eq!(stored.revoked_by_user_id, Some(admin.id()));
    assert!(stored.revoked_at.is_some());
}
