async fn assign_course_role(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    course_id: i32,
    role_id: i32,
) {
    course_role_records::assign_course_role_to_user(conn, user_id, course_id, role_id)
        .await
        .expect("failed to assign course role");
}

// ── identity bootstrap accounts ──

#[actix_web::test]
async fn test_create_user_with_verified_email() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "repo_user", true).await;
    assert_eq!(user.name, format!("repo_user Test"));
}

#[actix_web::test]
async fn test_create_user_with_unverified_email() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "repo_user_unver", false).await;
    assert_eq!(user.name, format!("repo_user_unver Test"));
}

// ── access-control organization helpers ──

#[actix_web::test]
async fn test_org_permission_check_admin_has_admin_perms() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "org_perm_admin", true).await;
    let org = create_organization(&mut conn, &unique_string("org_perm")).await;
    let admin_role_id = get_org_admin_role_id(&mut conn).await;
    assign_org_role(&mut conn, user.id(), org.id, admin_role_id).await;

    let has_permission = user_permission_organization_request(
        &mut conn,
        user.id(),
        org.id,
        &Permissions::VIEW_ORGANIZATION.to_string(),
    )
    .await
    .unwrap();
    assert!(has_permission);
}

#[actix_web::test]
async fn test_org_permission_check_stranger_has_no_perms() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "org_perm_stranger", true).await;
    let org = create_organization(&mut conn, &unique_string("org_perm_stranger")).await;

    let has_permission = user_permission_organization_request(
        &mut conn,
        user.id(),
        org.id,
        &Permissions::VIEW_ORGANIZATION.to_string(),
    )
    .await
    .unwrap();
    assert!(!has_permission);
}

#[actix_web::test]
async fn test_org_hierarchy_admin_above_member() {
    let mut conn = setup_conn().await;
    let admin = create_user_helper(&mut conn, "org_hier_admin", true).await;
    let member = create_user_helper(&mut conn, "org_hier_mem", true).await;
    let org = create_organization(&mut conn, &unique_string("org_hier")).await;
    let admin_role_id = get_org_admin_role_id(&mut conn).await;
    let member_role_id = get_org_member_role_id(&mut conn).await;
    assign_org_role(&mut conn, admin.id(), org.id, admin_role_id).await;
    assign_org_role(&mut conn, member.id(), org.id, member_role_id).await;

    let cmp = user_hierarchy_compare_organization(&mut conn, org.id, admin.id(), member.id())
        .await
        .unwrap();
    assert_eq!(cmp, Ordering::Greater);
}

#[actix_web::test]
async fn test_org_hierarchy_equal_users() {
    let mut conn = setup_conn().await;
    let user1 = create_user_helper(&mut conn, "org_hier_eq1", true).await;
    let user2 = create_user_helper(&mut conn, "org_hier_eq2", true).await;
    let org = create_organization(&mut conn, &unique_string("org_hier_eq")).await;
    let admin_role_id = get_org_admin_role_id(&mut conn).await;
    assign_org_role(&mut conn, user1.id(), org.id, admin_role_id).await;
    assign_org_role(&mut conn, user2.id(), org.id, admin_role_id).await;

    let cmp = user_hierarchy_compare_organization(&mut conn, org.id, user1.id(), user2.id())
        .await
        .unwrap();
    assert_eq!(cmp, Ordering::Equal);
}

#[actix_web::test]
async fn test_org_role_assignment_admin_can_assign_member() {
    let mut conn = setup_conn().await;
    let admin = create_user_helper(&mut conn, "org_role_admin", true).await;
    let member = create_user_helper(&mut conn, "org_role_mem", true).await;
    let org = create_organization(&mut conn, &unique_string("org_role")).await;
    let admin_role_id = get_org_admin_role_id(&mut conn).await;
    assign_org_role(&mut conn, admin.id(), org.id, admin_role_id).await;

    let result =
        assign_role_to_user_in_organization(&mut conn, admin.id(), member.id(), org.id, "STUDENT")
            .await;
    assert!(result.is_ok());
}
