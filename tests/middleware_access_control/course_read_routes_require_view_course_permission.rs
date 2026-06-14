#[actix_web::test]
async fn course_read_routes_require_view_course_permission() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    let mut conn = setup_conn(&pool).await;
    let student = create_test_user(&mut conn, "course_reader").await;
    let stranger = create_test_user(&mut conn, "course_stranger").await;

    force_assign_platform_role(&mut conn, student.id(), "STUDENT").await;

    let new_course = NewCourse {
        title: unique_string("ReadableCourse"),
        description: None,
        topics: None,
        prerequisites: None,
    };
    let course = diesel::insert_into(courses::table)
        .values(&new_course)
        .get_result::<Course>(&mut conn)
        .await
        .unwrap();

    let student_token = generate_token(student.id());
    let stranger_token = generate_token(stranger.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(course_discovery_use_case_data(&pool))
            .app_data(course_read_use_case_data(&pool))
            .app_data(course_organizations_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/courses")
        .insert_header(("Authorization", format!("Bearer {}", stranger_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri("/courses")
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/courses/{}", course.id))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/courses/{}/organizations", course.id))
        .insert_header(("Authorization", format!("Bearer {}", stranger_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri(&format!("/courses/{}/organizations", course.id))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::OK);
}

#[actix_web::test]
async fn organization_read_routes_require_view_organization_permission() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "org_reader_admin").await;
    let stranger = create_test_user(&mut conn, "org_reader_stranger").await;

    force_assign_platform_role(&mut conn, admin.id(), "SUPER_ADMIN").await;

    let new_org = NewOrganization {
        name: unique_string("ReadableOrg"),
        website_link: None,
        profile_url: None,
    };
    let org = diesel::insert_into(organizations::table)
        .values(&new_org)
        .get_result::<Organization>(&mut conn)
        .await
        .unwrap();

    let admin_token = generate_token(admin.id());
    let stranger_token = generate_token(stranger.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(web::resource("/organizations").route(
                web::get()
                    .to(|| async { actix_web::HttpResponse::Ok().finish() })
                    .wrap(
                        rust_learn::middlewares::platform_permission_middleware::PlatformPermissionMiddleware::require(
                            rust_learn::config::constants::permissions::Permissions::VIEW_ORGANIZATION.to_string(),
                        ),
                    ),
            ))
            .service(web::resource("/organizations/{id}").route(
                web::get()
                    .to(|| async { actix_web::HttpResponse::Ok().finish() })
                    .wrap(
                        rust_learn::middlewares::platform_permission_middleware::PlatformPermissionMiddleware::require(
                            rust_learn::config::constants::permissions::Permissions::VIEW_ORGANIZATION.to_string(),
                        ),
                    ),
            )),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/organizations")
        .insert_header(("Authorization", format!("Bearer {}", stranger_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri("/organizations")
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::OK);

    let req = test::TestRequest::get()
        .uri(&format!("/organizations/{}", org.id))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::OK);
}
