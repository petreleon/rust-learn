use crate::support::*;

#[actix_web::test]
async fn fraud_block_api_separates_read_audit_from_block_management() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let admin = create_test_user(&mut conn, "fraud_api_admin").await;
    let moderator = create_test_user(&mut conn, "fraud_api_moderator").await;
    let teacher = create_test_user(&mut conn, "fraud_api_teacher").await;
    assign_platform_role(&mut conn, admin.id(), "ADMIN").await;
    assign_platform_role(&mut conn, moderator.id(), "MODERATOR").await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(web::Data::new(reward_fraud_block_use_case(&pool)))
            .wrap(rust_learn::http::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::rewards::reward_fraud_block_scope()),
    )
    .await;

    let block_body = json!({
        "scope_type": "teacher",
        "teacher_user_id": teacher.id(),
        "reason": "api suspicious reward approvals",
        "evidence_reference": "case://api-teacher-block"
    });

    let denied_req = test::TestRequest::post()
        .uri("/reward-fraud-blocks")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(moderator.id())),
        ))
        .set_json(&block_body)
        .to_request();
    let denied_resp = test::call_service(&app, denied_req).await;
    assert_eq!(denied_resp.status(), StatusCode::FORBIDDEN);

    let create_req = test::TestRequest::post()
        .uri("/reward-fraud-blocks")
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .set_json(&block_body)
        .to_request();
    let create_resp = test::call_service(&app, create_req).await;
    assert_eq!(create_resp.status(), StatusCode::CREATED);
    let created: Value = test::read_body_json(create_resp).await;
    let block_id = created["id"].as_i64().expect("created fraud block id");
    assert_eq!(created["scope_type"], "teacher");
    assert_eq!(
        created["teacher_user_id"].as_i64(),
        Some(i64::from(teacher.id()))
    );

    let list_req = test::TestRequest::get()
        .uri("/reward-fraud-blocks?active=true&scope_type=teacher")
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(moderator.id())),
        ))
        .to_request();
    let list_resp = test::call_service(&app, list_req).await;
    assert_eq!(list_resp.status(), StatusCode::OK);
    let listed: Value = test::read_body_json(list_resp).await;
    assert!(listed["blocks"]
        .as_array()
        .expect("fraud block list")
        .iter()
        .any(|block| block["id"].as_i64() == Some(block_id)));

    let audit_req = test::TestRequest::get()
        .uri(&format!("/reward-fraud-blocks/{block_id}/audit"))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(moderator.id())),
        ))
        .to_request();
    let audit_resp = test::call_service(&app, audit_req).await;
    assert_eq!(audit_resp.status(), StatusCode::OK);
    let audit: Value = test::read_body_json(audit_resp).await;
    assert!(audit
        .as_array()
        .expect("fraud block audit")
        .iter()
        .any(|event| event["event_type"] == "created"
            && event["actor_user_id"].as_i64() == Some(i64::from(admin.id()))));

    let revoke_req = test::TestRequest::put()
        .uri(&format!("/reward-fraud-blocks/{block_id}/revoke"))
        .insert_header(("Authorization", format!("Bearer {}", token_for(admin.id()))))
        .to_request();
    let revoke_resp = test::call_service(&app, revoke_req).await;
    assert_eq!(revoke_resp.status(), StatusCode::OK);
    let revoked: Value = test::read_body_json(revoke_resp).await;
    assert_eq!(
        revoked["revoked_by_user_id"].as_i64(),
        Some(i64::from(admin.id()))
    );

    let audit_req = test::TestRequest::get()
        .uri(&format!("/reward-fraud-blocks/{block_id}/audit"))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(moderator.id())),
        ))
        .to_request();
    let audit_resp = test::call_service(&app, audit_req).await;
    assert_eq!(audit_resp.status(), StatusCode::OK);
    let audit: Value = test::read_body_json(audit_resp).await;
    assert!(audit
        .as_array()
        .expect("fraud block audit")
        .iter()
        .any(|event| event["event_type"] == "revoked"
            && event["actor_user_id"].as_i64() == Some(i64::from(admin.id()))));
}
