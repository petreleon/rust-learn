use crate::support::*;

#[actix_web::test]
async fn teacher_has_permissions() {
    let mut conn = setup_conn().await;
    let course_title = unique_string("TeacherCourse");
    let course = create_course(&mut conn, &course_title).await;

    let email = unique_string("teacher") + "@example.com";
    let user = create_user(
        &mut conn,
        "Teacher Test",
        &email,
        Some(NaiveDate::from_ymd_opt(1980, 5, 5).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");

    force_assign_role(&mut conn, user.id(), course.id, "TEACHER").await;

    let allowed_permissions = [Permissions::MANAGE_COURSE_SETTINGS];

    for permission in allowed_permissions {
        let has_permission =
            has_course_permission(&mut conn, user.id(), course.id, &permission.to_string())
                .await
                .expect("permission query failed");
        assert!(
            has_permission,
            "TEACHER should have permission: {:?}",
            permission
        );
    }
}
