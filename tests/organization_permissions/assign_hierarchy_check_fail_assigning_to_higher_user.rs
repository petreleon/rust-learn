#[actix_web::test]
async fn assign_hierarchy_check_fail_assigning_to_higher_user() {
    let mut conn = setup_conn().await;
    let org_name = unique_string("HierFailUser");
    let org = create_organization(&mut conn, &org_name).await;

    // 1. Create a STUDENT (Assigner)
    let member_user_assigner = create_user_helper(&mut conn, "assigner_weak").await;
    force_assign_role(&mut conn, member_user_assigner.id(), org.id, "STUDENT").await;

    // 2. Create an ADMIN (Target User) - already has high status
    let admin_target = create_user_helper(&mut conn, "target_strong").await;
    force_assign_role(&mut conn, admin_target.id(), org.id, "ADMIN").await;

    // 3. Member tries to assign STUDENT role to ADMIN
    // Expect Fail: Student (4) is NOT higher than Admin User (1)
    let result = assign_organization_role_with_hierarchy(
        &mut conn,
        member_user_assigner.id(),
        admin_target.id(),
        org.id,
        "STUDENT",
    )
    .await;
    assert!(
        result.is_err(),
        "STUDENT should NOT be able to modify ADMIN"
    );
}
