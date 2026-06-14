#[actix_web::test]
async fn platform_hierarchy_middleware_blocks_lower_actor() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "hier_platform_admin").await;
    let target = create_test_user(&mut conn, "hier_platform_target").await;
    let weak_actor = create_test_user(&mut conn, "hier_platform_weak").await;

    force_assign_platform_role(&mut conn, admin.id(), "SUPER_ADMIN").await;
    force_assign_platform_role(&mut conn, target.id(), "SUPER_ADMIN").await;

    let admin_token = generate_token(admin.id());
    let weak_actor_token = generate_token(weak_actor.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(web::resource("/platform/users/{id}").route(
                web::put()
                    .to(|| async { actix_web::HttpResponse::Ok().finish() })
                    .wrap(
                        rust_learn::middlewares::platform_hierarchy_middleware::PlatformHierarchyMiddleware::new(
                            rust_learn::http::request_params::ParamType::Path,
                            "id".to_string(),
                        ),
                    ),
            )),
    )
    .await;

    let req = test::TestRequest::put()
        .uri(&format!("/platform/users/{}", weak_actor.id()))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::OK);

    let req = test::TestRequest::put()
        .uri(&format!("/platform/users/{}", target.id()))
        .insert_header(("Authorization", format!("Bearer {}", weak_actor_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::FORBIDDEN);
}

#[actix_web::test]
async fn organization_hierarchy_middleware_blocks_lower_actor() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();

    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "hier_org_admin").await;
    let student = create_test_user(&mut conn, "hier_org_student").await;

    let new_org = NewOrganization {
        name: unique_string("HierOrg"),
        website_link: None,
        profile_url: None,
    };
    let org = diesel::insert_into(organizations::table)
        .values(&new_org)
        .get_result::<Organization>(&mut conn)
        .await
        .unwrap();

    force_assign_org_role(&mut conn, admin.id(), org.id, "ADMIN").await;
    force_assign_org_role(&mut conn, student.id(), org.id, "STUDENT").await;

    let admin_token = generate_token(admin.id());
    let student_token = generate_token(student.id());

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(
                web::resource("/organizations/{organization_id}/members/{user_id}").route(
                    web::put()
                        .to(|| async { actix_web::HttpResponse::Ok().finish() })
                        .wrap(
                            rust_learn::middlewares::organization_hierarchy_middleware::OrganizationHierarchyMiddleware::new(
                                rust_learn::http::request_params::ParamType::Path,
                                "user_id".to_string(),
                                rust_learn::http::request_params::ParamType::Path,
                                "organization_id".to_string(),
                            ),
                        ),
                ),
            ),
    )
    .await;

    let req = test::TestRequest::put()
        .uri(&format!(
            "/organizations/{}/members/{}",
            org.id,
            student.id()
        ))
        .insert_header(("Authorization", format!("Bearer {}", admin_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::OK);

    let req = test::TestRequest::put()
        .uri(&format!("/organizations/{}/members/{}", org.id, admin.id()))
        .insert_header(("Authorization", format!("Bearer {}", student_token)))
        .to_request();
    assert_eq!(response_status(app.call(req).await), StatusCode::FORBIDDEN);
}
