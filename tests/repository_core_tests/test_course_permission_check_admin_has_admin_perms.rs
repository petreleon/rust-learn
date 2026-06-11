// ── course_repository ──

#[actix_web::test]
async fn test_course_permission_check_admin_has_admin_perms() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "course_perm_admin", true).await;
    let course = create_course(&mut conn, &unique_string("course_perm")).await;
    let admin_role_id = get_course_admin_role_id(&mut conn).await;
    assign_course_role(&mut conn, user.id(), course.id, admin_role_id).await;

    let has_permission = user_permission_course_request(
        &mut conn,
        user.id(),
        course.id,
        &Permissions::MANAGE_COURSE_SETTINGS.to_string(),
    )
    .await
    .unwrap();
    assert!(has_permission);
}

#[actix_web::test]
async fn test_course_permission_check_student_has_no_admin_perms() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "course_perm_student", true).await;
    let course = create_course(&mut conn, &unique_string("course_perm_student")).await;
    let student_role_id = get_course_student_role_id(&mut conn).await;
    assign_course_role(&mut conn, user.id(), course.id, student_role_id).await;

    let has_permission = user_permission_course_request(
        &mut conn,
        user.id(),
        course.id,
        &Permissions::MANAGE_COURSE_SETTINGS.to_string(),
    )
    .await
    .unwrap();
    assert!(!has_permission);
}

#[actix_web::test]
async fn test_course_role_assignment_admin_can_assign_student() {
    let mut conn = setup_conn().await;
    let admin = create_user_helper(&mut conn, "course_role_admin", true).await;
    let student = create_user_helper(&mut conn, "course_role_stud", true).await;
    let course = create_course(&mut conn, &unique_string("course_role")).await;
    let admin_role_id = get_course_admin_role_id(&mut conn).await;
    assign_course_role(&mut conn, admin.id(), course.id, admin_role_id).await;

    let result = rust_learn::repositories::course_repository::assign_role_to_user_in_course(
        &mut conn,
        admin.id(),
        student.id(),
        course.id,
        "STUDENT",
    )
    .await;
    assert!(result.is_ok());
}

// ── platform_repository ──

#[actix_web::test]
async fn test_platform_permission_check_super_admin_has_perms() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "plat_perm_admin", true).await;
    let role_id = PlatformRole::find_by_name("SUPER_ADMIN", &mut conn)
        .await
        .expect("super admin role not found");
    UserRolePlatform::assign(&mut conn, user.id(), role_id)
        .await
        .expect("failed to assign platform role");

    let has_permission = user_permission_platform_request(
        &mut conn,
        user.id(),
        &Permissions::VIEW_REWARD_AUDIT.to_string(),
    )
    .await
    .unwrap();
    assert!(has_permission);
}

#[actix_web::test]
async fn test_platform_permission_check_regular_user_has_no_perm() {
    let mut conn = setup_conn().await;
    let user = create_user_helper(&mut conn, "plat_perm_regular", true).await;

    let has_permission = user_permission_platform_request(
        &mut conn,
        user.id(),
        &Permissions::VIEW_REWARD_AUDIT.to_string(),
    )
    .await
    .unwrap();
    assert!(!has_permission);
}
