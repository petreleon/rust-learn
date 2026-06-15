use crate::support::*;

#[actix_web::test]
async fn assign_hierarchy_check_fail_assigning_higher_role() {
    let mut conn = setup_conn().await;
    let course_title = unique_string("HierCourseFailRole");
    let course = create_course(&mut conn, &course_title).await;

    // 1. Create a STUDENT (Assigner)
    let student_assigner = create_user_helper(&mut conn, "student_assigner").await;
    force_assign_role(&mut conn, student_assigner.id(), course.id, "STUDENT").await;

    // 2. Create a fresh user
    let new_user = create_user_helper(&mut conn, "target_user").await;

    // 3. Student tries to assign TEACHER
    let result = assign_course_role_via_use_case(
        &mut conn,
        student_assigner.id(),
        new_user.id(),
        course.id,
        "TEACHER",
    )
    .await;
    assert!(
        result.is_err(),
        "STUDENT should NOT be able to assign TEACHER"
    );
}
