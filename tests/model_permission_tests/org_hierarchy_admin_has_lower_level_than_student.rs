#[actix_web::test]
async fn org_hierarchy_admin_has_lower_level_than_student() {
    let mut conn = setup_conn().await;
    let admin = user(&mut conn, "org_h_adm").await;
    let student = user(&mut conn, "org_h_stu").await;
    let admin_role_id = OrganizationRole::find_by_name("ADMIN", &mut conn)
        .await
        .unwrap();
    let student_role_id = OrganizationRole::find_by_name("STUDENT", &mut conn)
        .await
        .unwrap();

    organization_role_records::assign_organization_role_to_user(&mut conn, admin.id(), 1, admin_role_id)
        .await
        .unwrap();
    organization_role_records::assign_organization_role_to_user(&mut conn, student.id(), 1, student_role_id)
        .await
        .unwrap();

    let admin_level =
        hierarchy_records::organization_min_level_for_user(&mut conn, admin.id(), 1)
            .await
            .unwrap();
    let student_level =
        hierarchy_records::organization_min_level_for_user(&mut conn, student.id(), 1)
            .await
            .unwrap();

    assert!(
        admin_level.unwrap() < student_level.unwrap(),
        "admin hierarchy level should be lower (higher privilege) than student"
    );
}

// ── Course ──

#[actix_web::test]
async fn course_teacher_has_course_permission() {
    let mut conn = setup_conn().await;
    let u = user(&mut conn, "course_t").await;
    let role_id = CourseRole::find_by_name("TEACHER", &mut conn)
        .await
        .unwrap();

    course_role_records::assign_course_role_to_user(&mut conn, u.id(), 1, role_id)
        .await
        .unwrap();

    assert!(course_role_records::course_user_has_permission(
        &mut conn,
        u.id(),
        1,
        &Permissions::MANAGE_COURSE_SETTINGS.to_string(),
    )
    .await
    .unwrap());
}

#[actix_web::test]
async fn course_stranger_has_no_course_permission() {
    let mut conn = setup_conn().await;
    let u = user(&mut conn, "course_x").await;

    assert!(!course_role_records::course_user_has_permission(
        &mut conn,
        u.id(),
        999,
        &Permissions::MANAGE_COURSE_SETTINGS.to_string(),
    )
    .await
    .unwrap());
}

#[actix_web::test]
async fn course_hierarchy_teacher_above_student() {
    let mut conn = setup_conn().await;
    let teacher = user(&mut conn, "ch_t").await;
    let student = user(&mut conn, "ch_s").await;
    let teacher_role_id = CourseRole::find_by_name("TEACHER", &mut conn)
        .await
        .unwrap();
    let student_role_id = CourseRole::find_by_name("STUDENT", &mut conn)
        .await
        .unwrap();

    course_role_records::assign_course_role_to_user(&mut conn, teacher.id(), 1, teacher_role_id)
        .await
        .unwrap();
    course_role_records::assign_course_role_to_user(&mut conn, student.id(), 1, student_role_id)
        .await
        .unwrap();

    let t_level = hierarchy_records::course_min_level_for_user(&mut conn, teacher.id(), 1)
        .await
        .unwrap();
    let s_level = hierarchy_records::course_min_level_for_user(&mut conn, student.id(), 1)
        .await
        .unwrap();

    assert!(
        t_level.unwrap() < s_level.unwrap(),
        "teacher hierarchy level should be lower (higher privilege) than student"
    );
}

#[actix_web::test]
async fn user_without_course_role_has_no_course_level() {
    let mut conn = setup_conn().await;
    let u = user(&mut conn, "ch_none").await;

    let level = hierarchy_records::course_min_level_for_user(&mut conn, u.id(), 1)
        .await
        .unwrap();
    assert_eq!(level, None);
}
