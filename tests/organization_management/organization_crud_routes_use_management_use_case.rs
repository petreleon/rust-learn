use crate::support::*;

#[actix_web::test]
async fn organization_crud_routes_use_management_use_case() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let admin = create_test_user(&mut conn, "org_crud_admin").await;
    assign_platform_role_to_user(&mut conn, admin.id(), Roles::SUPER_ADMIN)
        .await
        .expect("failed to assign platform super admin");
    let course = create_course(&mut conn, "Organization CRUD course").await;
    let delete_target = create_organization(&mut conn, &unique_string("CrudDeleteOrg")).await;
    delegate_manage_org_settings(&mut conn, admin.id(), delete_target.id).await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(organization_management_use_case_data(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::organizations::organization_scope()),
    )
    .await;

    let token = token_for(admin.id());
    let create_resp = test::call_service(
        &app,
        test::TestRequest::post()
            .uri("/organizations")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(serde_json::json!({
                "name": unique_string("CrudCreatedOrg"),
                "website_link": "https://rust.example/org",
                "profile_url": "https://cdn.example/org.png",
                "course_ids": [course.id]
            }))
            .to_request(),
    )
    .await;
    assert_eq!(create_resp.status(), StatusCode::CREATED);
    let created: Value = test::read_body_json(create_resp).await;
    let created_id = created["id"].as_i64().expect("created id") as i32;
    assert_eq!(
        course_link_order(&pool, created_id, course.id).await,
        Some(0)
    );

    let list_resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri("/organizations")
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request(),
    )
    .await;
    assert_eq!(list_resp.status(), StatusCode::OK);
    let list: Value = test::read_body_json(list_resp).await;
    assert!(list
        .as_array()
        .unwrap()
        .iter()
        .any(|item| item["id"].as_i64() == Some(i64::from(created_id))));

    let mut conn = setup_conn(&pool).await;
    assign_organization_role(&mut conn, admin.id(), created_id, "ADMIN").await;
    drop(conn);
    let update_resp = test::call_service(
        &app,
        test::TestRequest::put()
            .uri(&format!("/organizations/{created_id}"))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .set_json(serde_json::json!({
                "name": "Updated CRUD Organization",
                "website_link": "https://rust.example/updated",
                "profile_url": "https://cdn.example/updated.png"
            }))
            .to_request(),
    )
    .await;
    assert_eq!(update_resp.status(), StatusCode::OK);

    let get_resp = test::call_service(
        &app,
        test::TestRequest::get()
            .uri(&format!("/organizations/{created_id}"))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request(),
    )
    .await;
    assert_eq!(get_resp.status(), StatusCode::OK);
    let fetched: Value = test::read_body_json(get_resp).await;
    assert_eq!(fetched["name"].as_str(), Some("Updated CRUD Organization"));

    let delete_resp = test::call_service(
        &app,
        test::TestRequest::delete()
            .uri(&format!("/organizations/{}", delete_target.id))
            .insert_header(("Authorization", format!("Bearer {}", token)))
            .to_request(),
    )
    .await;
    assert_eq!(delete_resp.status(), StatusCode::OK);
}
