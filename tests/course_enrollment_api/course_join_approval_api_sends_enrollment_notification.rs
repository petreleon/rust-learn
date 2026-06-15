use crate::support::*;

#[actix_web::test]
async fn course_join_approval_api_sends_enrollment_notification() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let student = create_test_user(&mut conn, "api_join_student").await;
    let teacher = create_test_user(&mut conn, "api_join_teacher").await;
    let course = create_course(&mut conn, &unique_string("ApiJoinCourse")).await;
    assign_platform_role(&mut conn, student.id(), "USER").await;
    assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    drop(conn);

    let app = test::init_service(course_enrollment_test_app(pool.clone())).await;

    let request_join = test::TestRequest::post()
        .uri(&format!("/courses/{}/join-requests", course.id))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let response = test::call_service(&app, request_join).await;
    assert_eq!(response.status(), StatusCode::CREATED);
    let body: Value = test::read_body_json(response).await;
    let request_id = body["id"].as_i64().expect("join request id");

    let approve_join = test::TestRequest::put()
        .uri(&format!(
            "/courses/{}/join-requests/{}/decision",
            course.id, request_id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(teacher.id())),
        ))
        .set_json(serde_json::json!({
            "status": COURSE_JOIN_STATUS_APPROVED,
            "decision_reason": "welcome"
        }))
        .to_request();
    let response = test::call_service(&app, approve_join).await;
    assert_eq!(response.status(), StatusCode::OK);
    let body: Value = test::read_body_json(response).await;
    assert_eq!(body["status"].as_str(), Some(COURSE_JOIN_STATUS_APPROVED));

    let mut conn = setup_conn(&pool).await;
    assert_eq!(
        notification_count(&mut conn, student.id(), "course:enrolled").await,
        1
    );
}

#[actix_web::test]
async fn course_role_assignment_api_does_not_send_enrollment_notification() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let teacher = create_test_user(&mut conn, "api_role_teacher").await;
    let target = create_test_user(&mut conn, "api_role_target").await;
    let course = create_course(&mut conn, &unique_string("ApiRoleCourse")).await;
    assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    drop(conn);

    let app = test::init_service(course_enrollment_test_app(pool.clone())).await;

    let assign_role = test::TestRequest::post()
        .uri(&format!(
            "/courses/{}/users/{}/roles",
            course.id,
            target.id()
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(teacher.id())),
        ))
        .set_json(serde_json::json!({ "role_name": "STUDENT" }))
        .to_request();
    let response = test::call_service(&app, assign_role).await;
    assert_eq!(response.status(), StatusCode::OK);

    let mut conn = setup_conn(&pool).await;
    assert_eq!(
        notification_count(&mut conn, target.id(), "role:assigned").await,
        1
    );
    assert_eq!(
        notification_count(&mut conn, target.id(), "course:enrolled").await,
        0
    );
}
