#[actix_web::test]
async fn course_scope_approval_assigns_course_teacher_permission_bundle() {
    let mut conn = setup_conn().await;
    let course = create_course(&mut conn, &unique_string("teacher_scope_course")).await;
    let applicant = create_user_helper(&mut conn, "teacher_course_applicant").await;
    let admin = create_user_helper(&mut conn, "teacher_course_admin").await;

    assign_role_to_user(&mut conn, applicant.id(), Roles::USER)
        .await
        .expect("failed to assign USER role");
    assign_role_to_user(&mut conn, admin.id(), Roles::ADMIN)
        .await
        .expect("failed to assign ADMIN role");

    let application = submit_application(
        &mut conn,
        applicant.id(),
        SubmitTeacherApplicationRequest {
            requested_scope: "course".to_string(),
            requested_organization_id: None,
            requested_course_id: Some(course.id),
            experience_summary: "Course-specific Rust instructor.".to_string(),
            organization_sponsor_id: None,
            portfolio_links: None,
            idempotency_key: None,
        },
    )
    .await
    .expect("course-scope application should be created");

    decide_application(
        &mut conn,
        admin.id(),
        application.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("approved for this course".to_string()),
        },
    )
    .await
    .expect("admin should approve course-scope teacher application");

    let applicant_can_manage_course_settings = user_permission_course_request(
        &mut conn,
        applicant.id(),
        course.id,
        &Permissions::MANAGE_COURSE_SETTINGS.to_string(),
    )
    .await
    .expect("permission query failed");
    assert!(
        applicant_can_manage_course_settings,
        "approved course-scope applicant should receive the course teaching bundle"
    );
}
