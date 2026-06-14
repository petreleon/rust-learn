#[actix_web::test]
async fn assign_permission_to_admin_and_verify_user_gets_it() {
    let mut conn = setup_conn().await;

    // Choose a permission that ADMIN does not have by default
    // Choose a permission that ADMIN does not have by default
    // Note: we'll reuse the enum value by referring to the constant again later to avoid move issues.
    // Assign it to ADMIN role (idempotent: insert or 0 rows if already exists)
    let rows =
        assign_platform_permission_to_role(&mut conn, Roles::ADMIN, Permissions::MANAGE_S3_OBJECTS)
            .await
            .expect("failed to assign permission to ADMIN");
    assert!(rows == 0 || rows == 1, "unexpected rows affected: {}", rows);

    // Create a fresh user and assign ADMIN role
    let email = unique_email("admin-perm");
    let user = create_user(
        &mut conn,
        "Admin Perm Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1993, 3, 3).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");

    assign_platform_role_to_user(&mut conn, user.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    // Now the permission should be granted to ADMIN users
    let ok = has_platform_permission(
        &mut conn,
        user.id(),
        &Permissions::MANAGE_S3_OBJECTS.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        ok,
        "ADMIN user should have MANAGE_S3_OBJECTS after assignment"
    );
}
