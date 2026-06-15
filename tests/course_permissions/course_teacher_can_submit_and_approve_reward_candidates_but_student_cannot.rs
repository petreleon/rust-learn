use crate::support::*;

#[actix_web::test]
async fn course_teacher_can_submit_and_approve_reward_candidates_but_student_cannot() {
    let mut conn = setup_conn().await;
    let course_title = unique_string("RewardCourse");
    let course = create_course(&mut conn, &course_title).await;

    let teacher = create_user_helper(&mut conn, "reward_teacher").await;
    let student = create_user_helper(&mut conn, "reward_student").await;
    force_assign_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_role(&mut conn, student.id(), course.id, "STUDENT").await;

    for permission in [
        Permissions::SUBMIT_COURSE_REWARD_EVENT,
        Permissions::APPROVE_STUDENT_REWARD_CANDIDATE,
        Permissions::GRADE_REWARDABLE_ASSESSMENT,
    ] {
        let teacher_has =
            has_course_permission(&mut conn, teacher.id(), course.id, &permission.to_string())
                .await
                .expect("permission query failed");
        assert!(teacher_has, "TEACHER should have {:?}", permission);

        let student_has =
            has_course_permission(&mut conn, student.id(), course.id, &permission.to_string())
                .await
                .expect("permission query failed");
        assert!(!student_has, "STUDENT should not have {:?}", permission);
    }

    let student_can_view_reward_status = has_course_permission(
        &mut conn,
        student.id(),
        course.id,
        &Permissions::VIEW_COURSE_REWARD_STATUS.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        student_can_view_reward_status,
        "STUDENT should be able to view own course reward status"
    );
}

#[actix_web::test]
async fn student_has_limited_permissions() {
    let mut conn = setup_conn().await;
    let course_title = unique_string("StudentCourse");
    let course = create_course(&mut conn, &course_title).await;

    let email = unique_string("student") + "@example.com";
    let user = create_user(
        &mut conn,
        "Student Test",
        &email,
        Some(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        "password123",
    )
    .await
    .expect("failed to create user");

    force_assign_role(&mut conn, user.id(), course.id, "STUDENT").await;

    // Assuming STUDENT does not have MANAGE_COURSE_SETTINGS
    let denied_permissions = [Permissions::MANAGE_COURSE_SETTINGS];

    for p in denied_permissions {
        let has_perm = has_course_permission(&mut conn, user.id(), course.id, &p.to_string())
            .await
            .expect("permission query failed");
        assert!(!has_perm, "STUDENT should NOT have permission: {:?}", p);
    }
}

#[actix_web::test]
async fn assign_hierarchy_check_success() {
    let mut conn = setup_conn().await;
    let course_title = unique_string("HierCourseSuccess");
    let course = create_course(&mut conn, &course_title).await;

    // 1. Create a TEACHER (Assigner)
    let teacher_user = create_user_helper(&mut conn, "teacher_assigner").await;
    force_assign_role(&mut conn, teacher_user.id(), course.id, "TEACHER").await;

    // 2. Create a fresh user (Assignee)
    let student_user = create_user_helper(&mut conn, "new_student").await;

    // 3. Teacher assigns STUDENT role
    let result = assign_course_role_via_use_case(
        &mut conn,
        teacher_user.id(),
        student_user.id(),
        course.id,
        "STUDENT",
    )
    .await;
    assert!(result.is_ok(), "TEACHER should be able to assign STUDENT");
}
