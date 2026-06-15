use crate::{create_organization::*, support::*};

#[actix_web::test]
async fn course_list_supports_search_org_filter_and_pagination() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let viewer = create_test_user(&mut conn).await;
    assign_platform_role(&mut conn, viewer.id(), "STUDENT").await;

    let org = create_organization(&mut conn, &unique_string("DiscoveryOrg")).await;
    let other_org = create_organization(&mut conn, &unique_string("OtherOrg")).await;
    let prefix = unique_string("DiscoveryCourse");
    let first = create_course(&mut conn, &format!("{} Rust Patterns", prefix)).await;
    let second = create_course(&mut conn, &format!("{} Rust Services", prefix)).await;
    let excluded = create_course(&mut conn, &format!("{} TypeScript", prefix)).await;
    let other_org_course = create_course(&mut conn, &format!("{} Rust External", prefix)).await;

    link_course_to_org(&mut conn, first.id, org.id, 0).await;
    link_course_to_org(&mut conn, second.id, org.id, 1).await;
    link_course_to_org(&mut conn, excluded.id, org.id, 2).await;
    link_course_to_org(&mut conn, other_org_course.id, other_org.id, 0).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(course_discovery_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::learning::course_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/courses?search=rust&organization_id={}&limit=1&offset=1",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(viewer.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["total"].as_i64(), Some(2));
    assert_eq!(body["limit"].as_i64(), Some(1));
    assert_eq!(body["offset"].as_i64(), Some(1));
    assert_eq!(body["search"].as_str(), Some("rust"));
    assert_eq!(body["organization_id"].as_i64(), Some(i64::from(org.id)));
    assert_eq!(body["courses"].as_array().expect("courses array").len(), 1);
    assert_eq!(
        body["courses"][0]["title"].as_str(),
        Some(second.title.as_str())
    );
}
