use crate::support::*;

#[actix_web::test]
async fn course_reward_candidates_list_filters_by_actor() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let teacher = create_verified_user(&mut conn, "course_candidate_teacher").await;
    let student = create_verified_user(&mut conn, "course_candidate_student").await;
    let other_student = create_verified_user(&mut conn, "course_candidate_other").await;
    let course = create_course(&mut conn).await;
    assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;
    assign_course_role(&mut conn, other_student.id(), course.id, "STUDENT").await;
    let student_candidate_id =
        create_reward_candidate(&mut conn, course.id, student.id(), teacher.id()).await;
    let other_candidate_id =
        create_reward_candidate(&mut conn, course.id, other_student.id(), teacher.id()).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(course_reward_candidates_use_case(&pool)))
            .app_data(rust_learn::bootstrap::auth_token_verifier_app_data())
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::rewards::course_reward_candidates_resource()),
    )
    .await;

    let student_req = test::TestRequest::get()
        .uri(&format!(
            "/courses/{}/reward-candidates?student_user_id={}&status=pending-teacher-approval",
            course.id,
            other_student.id()
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(student.id())),
        ))
        .to_request();
    let student_resp = test::call_service(&app, student_req).await;
    assert_eq!(student_resp.status(), StatusCode::OK);
    let student_body: Vec<Value> = test::read_body_json(student_resp).await;
    assert_eq!(student_body.len(), 1);
    assert_eq!(student_body[0]["id"].as_i64(), Some(student_candidate_id));
    assert_eq!(
        student_body[0]["student_user_id"].as_i64(),
        Some(i64::from(student.id()))
    );

    let teacher_req = test::TestRequest::get()
        .uri(&format!(
            "/courses/{}/reward-candidates?student_user_id={}",
            course.id,
            other_student.id()
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(teacher.id())),
        ))
        .to_request();
    let teacher_resp = test::call_service(&app, teacher_req).await;
    assert_eq!(teacher_resp.status(), StatusCode::OK);
    let teacher_body: Vec<Value> = test::read_body_json(teacher_resp).await;
    assert_eq!(teacher_body.len(), 1);
    assert_eq!(teacher_body[0]["id"].as_i64(), Some(other_candidate_id));
    assert_eq!(
        teacher_body[0]["student_user_id"].as_i64(),
        Some(i64::from(other_student.id()))
    );

    let invalid_req = test::TestRequest::get()
        .uri(&format!(
            "/courses/{}/reward-candidates?status=not-a-real-status",
            course.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(teacher.id())),
        ))
        .to_request();
    let invalid_resp = test::call_service(&app, invalid_req).await;
    assert_eq!(invalid_resp.status(), StatusCode::CONFLICT);
    let body: Value = test::read_body_json(invalid_resp).await;
    assert_eq!(body["error"]["code"], "invalid_reward_status");
    assert_eq!(
        body["error"]["message"],
        "unsupported reward candidate status"
    );
    assert_eq!(body["error"]["status"].as_u64(), Some(409));
}
