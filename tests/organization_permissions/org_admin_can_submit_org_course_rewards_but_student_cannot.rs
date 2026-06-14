#[actix_web::test]
async fn org_admin_can_submit_org_course_rewards_but_student_cannot() {
    let mut conn = setup_conn().await;
    let org_name = unique_string("OrgRewardTest");
    let org = create_organization(&mut conn, &org_name).await;

    let admin = create_user_helper(&mut conn, "reward_org_admin").await;
    let student = create_user_helper(&mut conn, "reward_org_student").await;
    force_assign_role(&mut conn, admin.id(), org.id, "ADMIN").await;
    force_assign_role(&mut conn, student.id(), org.id, "STUDENT").await;

    let admin_can_submit = has_organization_permission(
        &mut conn,
        admin.id(),
        org.id,
        &Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        admin_can_submit,
        "organization ADMIN should submit org reward events"
    );

    let student_can_submit = has_organization_permission(
        &mut conn,
        student.id(),
        org.id,
        &Permissions::SUBMIT_ORG_COURSE_REWARD_EVENT.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        !student_can_submit,
        "organization STUDENT should not submit org reward events"
    );
}

#[actix_web::test]
async fn org_member_has_limited_permissions() {
    let mut conn = setup_conn().await;
    let org_name = unique_string("OrgMemberTest");
    let org = create_organization(&mut conn, &org_name).await;

    let subject_user = create_user_helper(&mut conn, "subject_member").await;
    force_assign_role(&mut conn, subject_user.id(), org.id, "STUDENT").await;

    // Assuming STUDENT does not have MANAGE_ORG_SETTINGS
    let denied_permissions = [Permissions::MANAGE_ORG_SETTINGS];

    for p in denied_permissions {
        let has_perm = has_organization_permission(
            &mut conn,
            subject_user.id(),
            org.id,
            &p.to_string(),
        )
        .await
        .expect("permission query failed");
        assert!(!has_perm, "STUDENT should NOT have permission: {:?}", p);
    }
}

#[actix_web::test]
async fn assign_hierarchy_check_success() {
    let mut conn = setup_conn().await;
    let org_name = unique_string("HierSuccess");
    let org = create_organization(&mut conn, &org_name).await;

    // 1. Create an ADMIN (Assigner)
    let admin_user = create_user_helper(&mut conn, "assigner").await;
    force_assign_role(&mut conn, admin_user.id(), org.id, "ADMIN").await;

    // 2. Create a fresh user (Assignee)
    let member_user = create_user_helper(&mut conn, "assignee").await;

    // 3. Admin assigns STUDENT role to fresh user
    // Expect Success: Admin (1) is higher than Student role (4), and Admin (1) is higher than user (no role)
    let result = assign_organization_role_with_hierarchy(
        &mut conn,
        admin_user.id(),
        member_user.id(),
        org.id,
        "STUDENT",
    )
    .await;
    assert!(result.is_ok(), "ADMIN should be able to assign STUDENT");
}

#[actix_web::test]
async fn assign_hierarchy_check_fail_assigning_higher_role() {
    let mut conn = setup_conn().await;
    let org_name = unique_string("HierFailRole");
    let org = create_organization(&mut conn, &org_name).await;

    // 1. Create a STUDENT (Assigner) - trying to punch up
    let member_user_assigner = create_user_helper(&mut conn, "assigner_weak").await;
    force_assign_role(&mut conn, member_user_assigner.id(), org.id, "STUDENT").await;

    // 2. Create a fresh user (Assignee)
    let new_user = create_user_helper(&mut conn, "assignee_new").await;

    // 3. Member tries to assign ADMIN
    // Expect Fail: Student (4) is NOT higher than Admin role (1)
    let result = assign_organization_role_with_hierarchy(
        &mut conn,
        member_user_assigner.id(),
        new_user.id(),
        org.id,
        "ADMIN",
    )
    .await;
    assert!(
        result.is_err(),
        "STUDENT should NOT be able to assign ADMIN"
    );
}
