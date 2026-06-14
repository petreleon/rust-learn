#[actix_web::test]
async fn platform_audit_permission_reads_candidate_events() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let auditor = create_test_user(&mut conn, "candidate_audit_auditor").await;
    let student = create_test_user(&mut conn, "candidate_audit_student").await;
    let submitter = create_test_user(&mut conn, "candidate_audit_submitter").await;
    assign_platform_role(&mut conn, auditor.id(), "ADMIN").await;
    let course = create_course(&mut conn).await;
    let candidate = create_candidate(&mut conn, &course, &student, &submitter).await;
    let event = diesel::insert_into(reward_audit_events::table)
        .values(NewRewardAuditEvent {
            reward_candidate_id: candidate.id,
            actor_user_id: Some(submitter.id()),
            event_type: REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED.to_string(),
            from_status: None,
            to_status: REWARD_STATUS_PENDING_TEACHER_APPROVAL.to_string(),
            reason: Some("student completed the course".to_string()),
            metadata: json!({"source": "api-test"}),
        })
        .get_result::<rust_learn::models::reward_audit_event::RewardAuditEvent>(&mut conn)
        .await
        .expect("failed to create reward audit event");
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(reward_candidate_audit_use_case(&pool)))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::http::rewards::reward_candidate_audit_resource()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!("/reward-candidates/{}/audit", candidate.id))
        .insert_header(("Authorization", format!("Bearer {}", token_for(auditor.id()))))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Vec<Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["id"].as_i64(), Some(event.id));
    assert_eq!(
        body[0]["reward_candidate_id"].as_i64(),
        Some(candidate.id)
    );
    assert_eq!(body[0]["actor_user_id"].as_i64(), Some(i64::from(submitter.id())));
    assert_eq!(body[0]["event_type"], REWARD_AUDIT_EVENT_CANDIDATE_SUBMITTED);
    assert_eq!(body[0]["to_status"], REWARD_STATUS_PENDING_TEACHER_APPROVAL);
    assert_eq!(body[0]["metadata"]["source"], "api-test");

    let denied_req = test::TestRequest::get()
        .uri(&format!("/reward-candidates/{}/audit", candidate.id))
        .insert_header(("Authorization", format!("Bearer {}", token_for(student.id()))))
        .to_request();
    let denied_resp = test::call_service(&app, denied_req).await;
    assert_eq!(denied_resp.status(), StatusCode::FORBIDDEN);
    let body = to_bytes(denied_resp.into_body()).await.unwrap();
    assert_eq!(body.as_ref(), b"User does not have reward candidate permission");

    let missing_req = test::TestRequest::get()
        .uri("/reward-candidates/922337203685477580/audit")
        .insert_header(("Authorization", format!("Bearer {}", token_for(auditor.id()))))
        .to_request();
    let missing_resp = test::call_service(&app, missing_req).await;
    assert_eq!(missing_resp.status(), StatusCode::NOT_FOUND);
    let body = to_bytes(missing_resp.into_body()).await.unwrap();
    assert_eq!(body.as_ref(), b"Reward candidate not found");
}
