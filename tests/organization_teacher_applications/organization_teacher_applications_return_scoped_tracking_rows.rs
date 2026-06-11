#[actix_web::test]
async fn organization_teacher_applications_return_scoped_tracking_rows() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;

    let org = create_organization(&mut conn, &unique_string("TeacherAppsOrg")).await;
    let other_org = create_organization(&mut conn, &unique_string("OtherTeacherAppsOrg")).await;
    let operator = create_test_user(&mut conn, "org_teacher_operator").await;
    let reviewer = create_test_user(&mut conn, "org_teacher_reviewer").await;
    let submitted_applicant = create_test_user(&mut conn, "submitted_candidate").await;
    let approved_applicant = create_test_user(&mut conn, "approved_candidate").await;
    let other_applicant = create_test_user(&mut conn, "other_candidate").await;
    assign_organization_role(&mut conn, operator.id(), org.id, "ADMIN").await;
    assign_organization_role(&mut conn, operator.id(), other_org.id, "ADMIN").await;
    assign_role_to_user(&mut conn, reviewer.id(), Roles::ADMIN)
        .await
        .expect("failed to assign platform reviewer");

    let submitted = nominate_application(
        &mut conn,
        operator.id(),
        org.id,
        OrganizationTeacherNominationRequest {
            applicant_user_id: submitted_applicant.id(),
            requested_scope: None,
            requested_course_id: None,
            experience_summary: "Submitted candidate mentors Rust beginners.".to_string(),
            portfolio_links: Some(vec!["https://example.test/submitted".to_string()]),
            idempotency_key: None,
        },
    )
    .await
    .expect("org operator should nominate submitted applicant");

    let approved = nominate_application(
        &mut conn,
        operator.id(),
        org.id,
        OrganizationTeacherNominationRequest {
            applicant_user_id: approved_applicant.id(),
            requested_scope: None,
            requested_course_id: None,
            experience_summary: "Approved candidate runs ownership workshops.".to_string(),
            portfolio_links: None,
            idempotency_key: None,
        },
    )
    .await
    .expect("org operator should nominate approved applicant");
    decide_application(
        &mut conn,
        reviewer.id(),
        approved.id,
        TeacherApplicationDecisionRequest {
            status: "approved".to_string(),
            decision_reason: Some("strong organization sponsor".to_string()),
        },
    )
    .await
    .expect("platform reviewer should approve nomination");

    nominate_application(
        &mut conn,
        operator.id(),
        other_org.id,
        OrganizationTeacherNominationRequest {
            applicant_user_id: other_applicant.id(),
            requested_scope: None,
            requested_course_id: None,
            experience_summary: "Other organization candidate.".to_string(),
            portfolio_links: None,
            idempotency_key: None,
        },
    )
    .await
    .expect("other organization nomination should be created");
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .service(rust_learn::api::organizations::organization_scope()),
    )
    .await;

    let req = test::TestRequest::get()
        .uri(&format!(
            "/organizations/{}/teacher-applications?status=submitted&search=submitted&limit=10",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(operator.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: Value = test::read_body_json(resp).await;
    assert_eq!(
        body["organization"]["name"].as_str(),
        Some(org.name.as_str())
    );
    assert_eq!(body["total"].as_i64(), Some(1));
    assert_eq!(body["summary"]["total"].as_i64(), Some(2));
    assert_eq!(body["summary"]["submitted"].as_i64(), Some(1));
    assert_eq!(body["summary"]["approved"].as_i64(), Some(1));
    assert_eq!(
        body["operator_permissions"]["can_view_applications"].as_bool(),
        Some(true)
    );
    assert_eq!(
        body["operator_permissions"]["can_nominate_teachers"].as_bool(),
        Some(true)
    );

    let application = &body["applications"][0];
    assert_eq!(application["id"].as_i64(), Some(submitted.id));
    assert_eq!(
        application["applicant"]["name"].as_str(),
        Some(submitted_applicant.name.as_str())
    );
    assert_eq!(application["status"].as_str(), Some("submitted"));
    assert_eq!(
        application["requested_organization"]["name"].as_str(),
        Some(org.name.as_str())
    );
    assert_eq!(
        application["sponsored_by_this_organization"].as_bool(),
        Some(true)
    );
    assert_eq!(
        application["portfolio_links"][0].as_str(),
        Some("https://example.test/submitted")
    );
    assert_eq!(application["audit"]["event_count"].as_u64(), Some(1));
    assert_eq!(
        application["audit"]["latest_event_type"].as_str(),
        Some("organization_nominated")
    );
}
