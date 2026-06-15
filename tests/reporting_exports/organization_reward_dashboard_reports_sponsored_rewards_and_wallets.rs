use crate::{link_course_to_org::*, reporting_app_data::*, support::*};

#[actix_web::test]
async fn organization_reward_dashboard_reports_sponsored_rewards_and_wallets() {
    let _ = dotenvy::dotenv();
    let pool = establish_connection();
    let mut conn = setup_conn(&pool).await;
    let org_admin = create_test_user(&mut conn, "report_org_reward_admin").await;
    let stranger = create_test_user(&mut conn, "report_org_reward_stranger").await;
    let applicant = create_test_user(&mut conn, "report_org_reward_applicant").await;
    let student = create_test_user(&mut conn, "report_org_reward_student").await;
    let submitter = create_test_user(&mut conn, "report_org_reward_submitter").await;
    let org = create_organization(&mut conn).await;
    let other_org = create_organization(&mut conn).await;
    let course = create_course(&mut conn).await;
    let other_course = create_course(&mut conn).await;
    link_course_to_org(&mut conn, course.id, org.id).await;
    link_course_to_org(&mut conn, other_course.id, other_org.id).await;
    create_org_wallet_with_value(&mut conn, org.id, BigDecimal::from(77)).await;
    assign_organization_role(&mut conn, org_admin.id(), org.id, "ADMIN").await;
    create_sponsored_teacher_application(&mut conn, applicant.id(), org.id).await;
    let _first_reward = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
    )
    .await;
    let _second_reward = create_reward_candidate_with_status(
        &mut conn,
        course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_TOKEN_CONFIRMED,
    )
    .await;
    let _unrelated_reward = create_reward_candidate_with_status(
        &mut conn,
        other_course.id,
        student.id(),
        submitter.id(),
        REWARD_STATUS_AMOUNT_APPROVED,
    )
    .await;
    drop(conn);

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(|cfg| {
                rust_learn::bootstrap::configure_access_control_check_app_data(cfg, &pool)
            })
            .app_data(organization_reward_dashboard_use_case(&pool))
            .wrap(rust_learn::middlewares::jwt_middleware::JwtMiddleware)
            .configure(rust_learn::http::reporting::configure_routes),
    )
    .await;

    let forbidden_req = test::TestRequest::get()
        .uri(&format!(
            "/reports/organizations/{}/reward-dashboard",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(stranger.id())),
        ))
        .to_request();
    let forbidden_status = match app.call(forbidden_req).await {
        Ok(resp) => resp.status(),
        Err(err) => err.error_response().status(),
    };
    assert_eq!(forbidden_status, StatusCode::FORBIDDEN);

    let req = test::TestRequest::get()
        .uri(&format!(
            "/reports/organizations/{}/reward-dashboard",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body: Value = test::read_body_json(resp).await;
    assert_eq!(body["organization_id"].as_i64(), Some(i64::from(org.id)));
    assert!(
        body["sponsored_teacher_applications"]["submitted"]
            .as_i64()
            .unwrap_or_default()
            >= 1
    );
    assert!(body["course_reward_count"].as_i64().unwrap_or_default() >= 2);
    assert!(body["approved_reward_count"].as_i64().unwrap_or_default() >= 2);
    assert_eq!(body["approved_amount_total"], "20");
    assert_eq!(body["wallet_balance_total"], "77");
    assert!(body["courses"]
        .as_array()
        .expect("organization reward course rows")
        .iter()
        .any(
            |row| row["course_id"].as_i64() == Some(i64::from(course.id))
                && row["reward_candidate_count"].as_i64().unwrap_or_default() >= 2
                && row["approved_amount_total"] == "20"
        ));
    assert!(
        !body["courses"]
            .as_array()
            .expect("organization reward course rows")
            .iter()
            .any(|row| row["course_id"].as_i64() == Some(i64::from(other_course.id))),
        "organization dashboard must not include another organization's course rewards"
    );

    let export_req = test::TestRequest::get()
        .uri(&format!(
            "/reports/organizations/{}/reward-dashboard.csv",
            org.id
        ))
        .insert_header((
            "Authorization",
            format!("Bearer {}", token_for(org_admin.id())),
        ))
        .to_request();
    let export_resp = test::call_service(&app, export_req).await;
    assert_eq!(export_resp.status(), StatusCode::OK);
    let csv =
        String::from_utf8(test::read_body(export_resp).await.to_vec()).expect("valid utf8 csv");
    assert!(csv.starts_with("section,metric,value"));
    assert!(csv.contains("teacher_applications,submitted,"));
    assert!(csv.contains("reward_candidates,approved_amount_total,20"));
    assert!(csv.contains("wallets,balance_total,77"));
    assert!(csv.contains("courses,"));
}
