#[actix_web::test]
async fn test_course_permission_middleware() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    // Setup Data
    let mut conn = setup_conn(&pool).await;
    let teacher = create_test_user(&mut conn, "teacher").await;
    let student = create_test_user(&mut conn, "student").await;

    let new_course = NewCourse {
        title: unique_string("TestCourse"),
        description: None,
        topics: None,
        prerequisites: None,
    };
    let course = diesel::insert_into(courses::table)
        .values(&new_course)
        .get_result::<Course>(&mut conn)
        .await
        .unwrap();

    force_assign_course_role(&mut conn, teacher.id(), course.id, "TEACHER").await;
    force_assign_course_role(&mut conn, student.id(), course.id, "STUDENT").await;

    let teacher_token = generate_token(teacher.id());
    let student_token = generate_token(student.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::courses::course_scope()),
    )
    .await;

    // 1. Student tries to UPDATE course -> 403 (MANAGE_COURSE_SETTINGS required)
    // STUDENT does NOT have MANAGE_COURSE_SETTINGS.
    let req = test::TestRequest::put()
        .uri(&format!("/courses/{}", course.id))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .set_json(serde_json::json!({ "title": "Hacked Title" }))
        .to_request();

    let result = app.call(req).await;
    match result {
        Ok(resp) => assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN),
        Err(e) => {
            let resp = e.error_response();
            assert_eq!(resp.status(), actix_web::http::StatusCode::FORBIDDEN);
        }
    }

    // 2. Teacher tries to UPDATE course -> Should pass (200)
    // TEACHER has MANAGE_COURSE_SETTINGS.
    let req = test::TestRequest::put()
        .uri(&format!("/courses/{}", course.id))
        .insert_header(("Authorization", format!("Bearer {}", teacher_token)))
        .set_json(serde_json::json!({ "title": "Updated Title" }))
        .to_request();

    let result = app.call(req).await;
    match result {
        Ok(resp) => assert!(resp.status().is_success(), "Teacher request failed"),
        Err(e) => panic!("Teacher request returned error: {}", e),
    }
}

#[actix_web::test]
async fn read_user_routes_require_view_user_or_self() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "read_user_admin").await;
    let target = create_test_user(&mut conn, "read_user_target").await;
    let stranger = create_test_user(&mut conn, "read_user_stranger").await;

    force_assign_platform_role(&mut conn, admin.id(), "SUPER_ADMIN").await;

    let admin_token = generate_token(admin.id());
    let target_token = generate_token(target.id());
    let stranger_token = generate_token(stranger.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::users::user_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/user")
        .insert_header(("Authorization", format!("Bearer {}", stranger_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri("/user")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/user/{}", target.id()))
        .insert_header(("Authorization", format!("Bearer {}", target_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/user/{}", target.id()))
        .insert_header(("Authorization", format!("Bearer {}", stranger_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::FORBIDDEN);
}
